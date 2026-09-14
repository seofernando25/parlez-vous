use crate::ai::EmbeddingProvider;
use rusqlite::Connection;
use std::path::Path;
use std::sync::{Arc, Mutex};

fn chunk_text(text: &str, max_len: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current_chunk = String::new();

    // Split by lines to preserve paragraph structure and headings
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // If adding this line pushes us over the limit, save the chunk
        if current_chunk.len() + line.len() > max_len && !current_chunk.is_empty() {
            chunks.push(current_chunk.clone());

            // Overlap: Keep the last ~150 bytes, but safely align to UTF-8 character boundaries
            let mut overlap_start = current_chunk.len().saturating_sub(150);

            // SAFTEY CHECK: Step forward until we hit a valid UTF-8 character boundary.
            // Since max UTF-8 char is 4 bytes, this loops a maximum of 3 times.
            while overlap_start < current_chunk.len()
                && !current_chunk.is_char_boundary(overlap_start)
            {
                overlap_start += 1;
            }

            if let Some(idx) = current_chunk[overlap_start..].find(' ') {
                current_chunk = current_chunk[overlap_start + idx..].to_string();
            } else {
                current_chunk = String::new();
            }
        }

        // Preserve the newline character!
        if !current_chunk.is_empty() {
            current_chunk.push('\n');
        }
        current_chunk.push_str(line);
    }

    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }

    chunks
}

pub async fn ingest_pdf(
    db: Arc<Mutex<Connection>>,
    ai: Arc<dyn EmbeddingProvider + Send + Sync>,
    file_path: String,
    model: String,
) -> Result<(), String> {
    println!("[RAG] Starting ingestion for file: {}", file_path);

    let path = Path::new(&file_path);
    let file_name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    // 1. Extract Text
    let raw_text = tokio::task::spawn_blocking(move || {
        pdf_extract::extract_text(&file_path)
            .map_err(|e| format!("Failed to extract text from PDF: {}", e))
    })
    .await
    .map_err(|e| e.to_string())??;

    // Split by form-feed character (\x0C) to guess page boundaries
    let pages: Vec<&str> = raw_text.split('\x0C').collect();
    println!("[RAG] Extracted {} pages from PDF.", pages.len());

    let mut total_chunks = 0;
    {
        let conn = db.lock().map_err(|_| "Failed to lock database")?;
        conn.execute("DELETE FROM document_chunks WHERE document_name = ?1", [&file_name])
            .map_err(|error| error.to_string())?;
    }

    for (page_idx, page_text) in pages.iter().enumerate() {
        let page_number = (page_idx + 1) as i32;
        let chunks = chunk_text(page_text, 1000);
        let chunks_len = chunks.len();
        if chunks_len == 0 {
            continue;
        }

        for (i, chunk) in chunks.into_iter().enumerate() {
            total_chunks += 1;
            println!(
                "[RAG] Embedding chunk {}/{} for page {}",
                i + 1,
                chunks_len,
                page_number
            );

            // Add Nomic prefix for documents
            let embed_text = if model.contains("nomic") {
                format!("search_document: {}", chunk)
            } else {
                chunk.clone()
            };

            let embedding = ai.generate_embedding(embed_text, model.clone()).await?;

            // 4. Database Insertion
            let db_clone = db.clone();
            let file_name_clone = file_name.clone();
            let embedding_model = model.clone();

            tokio::task::spawn_blocking(move || {
            let conn = db_clone.lock().map_err(|_| "Failed to lock database")?;

            // Convert Vec<f64> to Vec<f32> and then to bytes for sqlite-vec
            let f32_vec: Vec<f32> = embedding.iter().map(|&x| x as f32).collect();
            let bytes: &[u8] = unsafe {
                std::slice::from_raw_parts(
                    f32_vec.as_ptr() as *const u8,
                    f32_vec.len() * std::mem::size_of::<f32>(),
                )
            };

            conn.execute(
                "INSERT INTO document_chunks (document_name, chunk_text, page_number, embedding, embedding_dimensions, embedding_model) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                rusqlite::params![file_name_clone, chunk, page_number, bytes, f32_vec.len() as i64, embedding_model],
            )
            .map_err(|e| e.to_string())?;

            Ok::<(), String>(())
        })
        .await
        .map_err(|e| e.to_string())??;
        }
    }

    println!(
        "[RAG] Successfully ingested PDF ({} total chunks).",
        total_chunks
    );
    Ok(())
}

pub async fn list_textbooks(db: Arc<Mutex<Connection>>) -> Result<Vec<String>, String> {
    tokio::task::spawn_blocking(move || {
        let conn = db.lock().map_err(|_| "Failed to lock database")?;
        let mut stmt = conn
            .prepare("SELECT DISTINCT document_name FROM document_chunks")
            .map_err(|e| e.to_string())?;

        let rows = stmt
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|e| e.to_string())?;

        let mut books = Vec::new();
        for row in rows {
            if let Ok(name) = row {
                books.push(name);
            }
        }
        Ok(books)
    })
    .await
    .map_err(|e| e.to_string())?
}

pub async fn query_context(
    db: Arc<Mutex<Connection>>,
    ai: Arc<dyn EmbeddingProvider + Send + Sync>,
    document_name: String,
    query: String,
    model: String,
    current_page: Option<i32>,
    active_theme: Option<String>,
) -> Result<Vec<String>, String> {
    // RAG Theme Dilution Prevention: Only expand if query is very short
    let word_count = query.split_whitespace().count();
    let expanded_query = if word_count < 3 {
        if let Some(theme) = active_theme {
            format!("{} {}", query, theme)
        } else {
            query.clone()
        }
    } else {
        query.clone()
    };

    // Add Nomic prefix for queries
    let embed_query = if model == "parlezvous-embed" || model.to_ascii_lowercase().contains("qwen3-embedding") {
        format!("Instruct: Given a language-learning textbook query, retrieve relevant passages that answer or explain it\nQuery: {}", expanded_query)
    } else if model.contains("nomic") {
        format!("search_query: {}", expanded_query)
    } else {
        expanded_query.clone()
    };
    println!("Query: {}", embed_query.clone());

    // 1. Embed query
    let embedding = ai.generate_embedding(embed_query, model.clone()).await?;

    // 2. Retrieve matching chunks. Vector dimensions live with each row, so the
    // schema is not coupled to one embedding family. We only compare rows produced
    // by the selected model, which guarantees equal vector lengths for sqlite-vec.
    tokio::task::spawn_blocking(move || {
        let conn = db.lock().map_err(|_| "Failed to lock database")?;
        let f32_vec: Vec<f32> = embedding.iter().map(|&value| value as f32).collect();
        let bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                f32_vec.as_ptr() as *const u8,
                f32_vec.len() * std::mem::size_of::<f32>(),
            )
        };

        let mut sql = String::from(
            "SELECT chunk_text FROM document_chunks \
             WHERE document_name = ?1 AND embedding_model = ?2 AND embedding_dimensions = ?4 AND embedding IS NOT NULL",
        );
        if let Some(page) = current_page {
            sql.push_str(&format!(
                " AND page_number BETWEEN {} AND {}",
                page.saturating_sub(1).max(1),
                page + 1
            ));
        }
        sql.push_str(" ORDER BY vec_distance_cosine(embedding, ?3) ASC LIMIT 10");

        let mut stmt = conn.prepare(&sql).map_err(|error| error.to_string())?;
        let rows = stmt
            .query_map(rusqlite::params![document_name, model, bytes, f32_vec.len() as i64], |row| row.get::<_, String>(0))
            .map_err(|error| error.to_string())?;
        rows.collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())
    })
    .await
    .map_err(|error| error.to_string())?
}
