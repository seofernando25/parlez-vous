use async_trait::async_trait;
use crate::ai::{
    ConjugationExercise, ConjugationProvider, TenseStat,
    openai_compat_adapter::{text_message, OpenAiCompatibleAdapter},
};

#[async_trait]
impl ConjugationProvider for OpenAiCompatibleAdapter {
    async fn generate_conjugation_exercise(
        &self,
        language: String,
        model: String,
        previously_used: Vec<String>,
        tense_stats: Vec<TenseStat>,
        active_theme: String,
    ) -> Result<ConjugationExercise, String> {
        let prompt = crate::ai::build_conjugation_prompt(
            &language,
            &previously_used,
            &tense_stats,
            &active_theme,
        );
        let mut candidate: ConjugationExercise = self.generate_json(
            &model,
            vec![text_message("user", prompt)],
            600,
            45,
        ).await?;

        let verification = crate::ai::build_conjugation_verification_prompt(&language, &candidate);
        if let Ok(revised) = self.generate_json::<ConjugationExercise>(
            &model,
            vec![text_message("user", verification)],
            900,
            45,
        ).await {
            candidate = revised;
        }
        Ok(candidate)
    }
}
