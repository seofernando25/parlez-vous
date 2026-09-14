use anyhow::Result;
use ndarray::{Array, Array3};
use ort::{session::Session, value::Value};
use super::{chunking::chunk_text, config::Config, text::{sample_noisy_latent, UnicodeProcessor}};

pub struct Style {
    pub ttl: Array3<f32>,
    pub dp: Array3<f32>,
}

pub struct TextToSpeech {
    cfgs: Config,
    text_processor: UnicodeProcessor,
    dp_ort: Session,
    text_enc_ort: Session,
    vector_est_ort: Session,
    vocoder_ort: Session,
    pub sample_rate: i32,
}

impl TextToSpeech {
    pub fn new(
        cfgs: Config,
        text_processor: UnicodeProcessor,
        dp_ort: Session,
        text_enc_ort: Session,
        vector_est_ort: Session,
        vocoder_ort: Session,
    ) -> Self {
        let sample_rate = cfgs.ae.sample_rate;
        TextToSpeech {
            cfgs,
            text_processor,
            dp_ort,
            text_enc_ort,
            vector_est_ort,
            vocoder_ort,
            sample_rate,
        }
    }

    fn _infer(
        &mut self,
        text_list: &[String],
        lang_list: &[String],
        style: &Style,
        total_step: usize,
        speed: f32,
    ) -> Result<(Vec<f32>, Vec<f32>)> {
        let bsz = text_list.len();

        // Process text
        let (text_ids, text_mask) = self.text_processor.call(text_list, lang_list)?;

        let text_ids_array = {
            let text_ids_shape = (bsz, text_ids[0].len());
            let mut flat = Vec::new();
            for row in &text_ids {
                flat.extend_from_slice(row);
            }
            Array::from_shape_vec(text_ids_shape, flat)?
        };

        let text_ids_value = Value::from_array(text_ids_array)?;
        let text_mask_value = Value::from_array(text_mask.clone())?;
        let style_dp_value = Value::from_array(style.dp.clone())?;

        // Predict duration
        let dp_outputs = self.dp_ort.run(ort::inputs!{
            "text_ids" => &text_ids_value,
            "style_dp" => &style_dp_value,
            "text_mask" => &text_mask_value
        })?;

        let duration_data = dp_outputs["duration"].try_extract_tensor::<f32>()?;
        let mut duration: Vec<f32> = duration_data.1.to_vec();

        // Apply speed factor to duration
        for dur in duration.iter_mut() {
            *dur /= speed;
        }

        // Encode text
        let style_ttl_value = Value::from_array(style.ttl.clone())?;
        let text_enc_outputs = self.text_enc_ort.run(ort::inputs!{
            "text_ids" => &text_ids_value,
            "style_ttl" => &style_ttl_value,
            "text_mask" => &text_mask_value
        })?;

        let text_emb_data = text_enc_outputs["text_emb"].try_extract_tensor::<f32>()?;
        let text_emb_shape = text_emb_data.0;
        let text_emb = Array3::from_shape_vec(
            (text_emb_shape[0] as usize, text_emb_shape[1] as usize, text_emb_shape[2] as usize),
            text_emb_data.1.to_vec()
        )?;

        // Sample noisy latent
        let (mut xt, latent_mask) = sample_noisy_latent(
            &duration,
            self.sample_rate,
            self.cfgs.ae.base_chunk_size,
            self.cfgs.ttl.chunk_compress_factor,
            self.cfgs.ttl.latent_dim,
        );

        // Prepare constant arrays
        let total_step_array = Array::from_elem(bsz, total_step as f32);

        // Denoising loop
        for step in 0..total_step {
            let current_step_array = Array::from_elem(bsz, step as f32);

            let xt_value = Value::from_array(xt.clone())?;
            let text_emb_value = Value::from_array(text_emb.clone())?;
            let latent_mask_value = Value::from_array(latent_mask.clone())?;
            let text_mask_value2 = Value::from_array(text_mask.clone())?;
            let current_step_value = Value::from_array(current_step_array)?;
            let total_step_value = Value::from_array(total_step_array.clone())?;

            let vector_est_outputs = self.vector_est_ort.run(ort::inputs!{
                "noisy_latent" => &xt_value,
                "text_emb" => &text_emb_value,
                "style_ttl" => &style_ttl_value,
                "latent_mask" => &latent_mask_value,
                "text_mask" => &text_mask_value2,
                "current_step" => &current_step_value,
                "total_step" => &total_step_value
            })?;

            let denoised_data = vector_est_outputs["denoised_latent"].try_extract_tensor::<f32>()?;
            let denoised_shape = denoised_data.0;
            xt = Array3::from_shape_vec(
                (denoised_shape[0] as usize, denoised_shape[1] as usize, denoised_shape[2] as usize),
                denoised_data.1.to_vec()
            )?;
        }

        // Generate waveform
        let final_latent_value = Value::from_array(xt)?;
        let vocoder_outputs = self.vocoder_ort.run(ort::inputs!{
            "latent" => &final_latent_value
        })?;

        let wav_data = vocoder_outputs["wav_tts"].try_extract_tensor::<f32>()?;
        let wav: Vec<f32> = wav_data.1.to_vec();

        Ok((wav, duration))
    }

    pub fn call<F>(
        &mut self,
        text: &str,
        lang: &str,
        style: &Style,
        total_step: usize,
        speed: f32,
        silence_duration: f32,
        mut callback: F,
    ) -> Result<(Vec<f32>, f32)>
    where F: FnMut(usize, usize, Option<&[f32]>) -> bool {
        let max_len = if lang == "ko" { 120 } else { 300 };
        let chunks = chunk_text(text, Some(max_len));
        let num_chunks = chunks.len();

        let mut wav_cat: Vec<f32> = Vec::new();
        let mut dur_cat: f32 = 0.0;

        for (i, chunk) in chunks.iter().enumerate() {
            // Notify start of chunk (audio is None)
            if !callback(i, num_chunks, None) {
                return Err(anyhow::anyhow!("Synthesis cancelled by user"));
            }

            let (wav, duration) = self._infer(&[chunk.clone()], &[lang.to_string()], style, total_step, speed)?;

            // Truncate audio based on predicted duration to remove trailing silence
            let dur = duration[0];
            let sample_count = (dur * self.sample_rate as f32) as usize;
            let wav_chunk = if sample_count < wav.len() {
                &wav[..sample_count]
            } else {
                &wav[..]
            };

            // Send audio chunk
            if !callback(i, num_chunks, Some(wav_chunk)) {
                 return Err(anyhow::anyhow!("Synthesis cancelled by user"));
            }

            if i == 0 {
                wav_cat.extend_from_slice(wav_chunk);
                dur_cat = dur;
            } else {
                let silence_len = (silence_duration * self.sample_rate as f32) as usize;
                let silence = vec![0.0f32; silence_len];

                wav_cat.extend_from_slice(&silence);
                wav_cat.extend_from_slice(wav_chunk);
                dur_cat += silence_duration + dur;
            }
        }
        callback(num_chunks, num_chunks, None);

        Ok((wav_cat, dur_cat))
    }

    #[allow(dead_code)]
    pub fn batch(
        &mut self,
        text_list: &[String],
        lang_list: &[String],
        style: &Style,
        total_step: usize,
        speed: f32,
    ) -> Result<(Vec<f32>, Vec<f32>)> {
        self._infer(text_list, lang_list, style, total_step, speed)
    }
}
