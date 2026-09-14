use anyhow::Result;

// ============================================================================
// Utility Functions
// ============================================================================

#[allow(dead_code)]
pub fn timer<F, T>(name: &str, f: F) -> Result<T>
where
    F: FnOnce() -> Result<T>,
{
    let start = std::time::Instant::now();
    println!("{}...", name);
    let result = f()?;
    let elapsed = start.elapsed().as_secs_f64();
    println!("  -> {} completed in {:.2} sec", name, elapsed);
    Ok(result)
}

#[allow(dead_code)]
pub fn sanitize_filename(text: &str, max_len: usize) -> String {
    // Take first max_len characters (Unicode code points, not bytes)
    text.chars()
        .take(max_len)
        .map(|c| {
            // is_alphanumeric() works with all Unicode letters and digits
            if c.is_alphanumeric() {
                c
            } else {
                '_'
            }
        })
        .collect()
}
