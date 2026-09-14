export interface ConjugationExercise {
    subject: string;
    tense: string;
    sentence: string;
    verb: string;
    answer: string;
    translation: string;
}
export interface ConjugationResponse { exercise: ConjugationExercise; history_id: number; }
export interface HistoryEntry { exercise: ConjugationExercise; userAnswer: string; correct: boolean; }
