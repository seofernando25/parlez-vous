use async_trait::async_trait;
use crate::ai::{ConjugationExercise, ConjugationProvider, TenseStat, litert_adapter::LiteRtAdapter};

#[async_trait]
impl ConjugationProvider for LiteRtAdapter {
    async fn generate_conjugation_exercise(
        &self,
        language: String,
        _model: String,
        previously_used: Vec<String>,
        tense_stats: Vec<TenseStat>,
        active_theme: String,
    ) -> Result<ConjugationExercise, String> {
        self.ensure_initialized().await?;
        let prompt = crate::ai::build_conjugation_prompt(
            &language,
            &previously_used,
            &tense_stats,
            &active_theme,
        );

        let initial_exercise: ConjugationExercise = self.execute_json_generation(prompt, "Conjugation")?;

        let verify_prompt = crate::ai::build_conjugation_verification_prompt(&language, &initial_exercise);
        self.execute_json_generation(verify_prompt, "Conjugation Verification")
    }
}
