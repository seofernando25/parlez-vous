<script lang="ts">
    import './styles/conjugation-game.1.css';
    import './styles/conjugation-game.2.css';
    import verbData from '../data/verb.json';

    // Type declarations
    interface VerbConjugation {
        present: string;
        past: string;
    }
    
    interface LanguageData {
        language: string;
        verbs: {
            speak: VerbConjugation;
            eat: VerbConjugation;
            learn: VerbConjugation;
        };
    }

    const typedVerbData = verbData as Record<string, LanguageData>;

    // List of languages for selection
    const languages = Object.entries(typedVerbData).map(([code, data]) => ({
        code,
        name: data.language
    })).sort((a, b) => a.name.localeCompare(b.name));

    // Game state variables
    let { 
        selectedLangCode = $bindable('fr'),
        startTrigger = $bindable(0)
    } = $props<{ 
        selectedLangCode?: string; 
        startTrigger?: number;
    }>();

    $effect(() => {
        if (startTrigger > 0) {
            generateGame();
        }
    });

    let gameState = $state<'intro' | 'playing' | 'completed'>('intro');
    
    let currentQuestionIndex = $state(0);
    let score = $state(0);
    let selectedAnswer = $state<string | null>(null);
    let isAnswered = $state(false);

    // Question setup
    type VerbKey = 'speak' | 'eat' | 'learn';
    type TenseKey = 'present' | 'past';

    interface Question {
        verb: VerbKey;
        tense: TenseKey;
        correctAnswer: string;
        options: string[];
    }

    let questions = $state<Question[]>([]);

    function generateGame() {
        const langData = typedVerbData[selectedLangCode];
        if (!langData) return;

        const verbKeys: VerbKey[] = ['speak', 'eat', 'learn'];
        // Generate a question for each of the 3 verbs
        const generated: Question[] = verbKeys.map((verb, index) => {
            // Alternate tenses to cover both
            const tense: TenseKey = index % 2 === 0 ? 'present' : 'past';
            const correctAnswer = langData.verbs[verb][tense];
            
            // Gather all possible answers for this language to create distractors
            const allPossibleAnswers = new Set<string>();
            Object.values(langData.verbs).forEach(v => {
                allPossibleAnswers.add(v.present);
                allPossibleAnswers.add(v.past);
            });

            // Remove the correct answer from distractors pool
            allPossibleAnswers.delete(correctAnswer);

            const distractors = Array.from(allPossibleAnswers);
            
            // Shuffle distractors and select 3
            const selectedDistractors = distractors
                .sort(() => 0.5 - Math.random())
                .slice(0, 3);

            // Fill up with placeholders if we don't have enough distractors (rare)
            while (selectedDistractors.length < 3) {
                selectedDistractors.push("—");
            }

            // Combine correct answer and distractors, then shuffle
            const options = [correctAnswer, ...selectedDistractors].sort(() => 0.5 - Math.random());

            return {
                verb,
                tense,
                correctAnswer,
                options
            };
        });

        questions = generated;
        currentQuestionIndex = 0;
        score = 0;
        selectedAnswer = null;
        isAnswered = false;
        gameState = 'playing';
    }

    function selectOption(option: string) {
        if (isAnswered) return;
        selectedAnswer = option;
        isAnswered = true;
        
        if (option === questions[currentQuestionIndex].correctAnswer) {
            score++;
        }
    }

    function nextQuestion() {
        selectedAnswer = null;
        isAnswered = false;
        
        if (currentQuestionIndex + 1 < questions.length) {
            currentQuestionIndex++;
        } else {
            gameState = 'completed';
        }
    }

    function resetGame() {
        gameState = 'intro';
    }
</script>

<div class="game-card">
    {#if gameState === 'intro'}
        <div class="intro-screen">
            <div class="badge">Static Demo Game</div>
            <h2>Conjugation Practice</h2>
            <p>Select a language below to test your conjugation knowledge before downloading the app.</p>
            
            <div class="select-container">
                <label for="language-select">Target Language</label>
                <select id="language-select" bind:value={selectedLangCode}>
                    {#each languages as lang}
                        <option value={lang.code}>{lang.name}</option>
                    {/each}
                </select>
            </div>
            
            <button class="btn btn-primary" onclick={generateGame}>
                Start Quiz
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="5" y1="12" x2="19" y2="12"></line><polyline points="12 5 19 12 12 19"></polyline></svg>
            </button>
        </div>
    {:else if gameState === 'playing'}
        {@const currentQuestion = questions[currentQuestionIndex]}
        <div class="quiz-screen">
            <div class="quiz-header">
                <span class="progress-indicator">Question {currentQuestionIndex + 1} of {questions.length}</span>
                <span class="score-indicator">Score: {score}</span>
            </div>
            
            <div class="progress-bar-container">
                <div class="progress-bar" style="width: {((currentQuestionIndex + (isAnswered ? 1 : 0)) / questions.length) * 100}%"></div>
            </div>

            <div class="question-box">
                <span class="sub">Conjugate the verb</span>
                <h3>{currentQuestion.verb.toUpperCase()}</h3>
                <span class="tense-tag">{currentQuestion.tense.toUpperCase()} TENSE</span>
                <p class="question-text">
                    What is the correct form in <strong>{typedVerbData[selectedLangCode].language}</strong>?
                </p>
            </div>

            <div class="options-grid">
                {#each currentQuestion.options as option}
                    {@const isCorrect = option === currentQuestion.correctAnswer}
                    {@const isSelected = option === selectedAnswer}
                    <button 
                        class="option-btn" 
                        class:correct={isAnswered && isCorrect}
                        class:incorrect={isAnswered && isSelected && !isCorrect}
                        class:disabled={isAnswered}
                        onclick={() => selectOption(option)}
                        disabled={isAnswered}
                    >
                        <span class="option-text">{option}</span>
                        {#if isAnswered && isCorrect}
                            <svg class="status-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>
                        {:else if isAnswered && isSelected && !isCorrect}
                            <svg class="status-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
                        {/if}
                    </button>
                {/each}
            </div>

            {#if isAnswered}
                <button class="btn btn-primary next-btn" onclick={nextQuestion}>
                    {currentQuestionIndex + 1 === questions.length ? 'Show Results' : 'Next Question'}
                    <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="5" y1="12" x2="19" y2="12"></line><polyline points="12 5 19 12 12 19"></polyline></svg>
                </button>
            {/if}
        </div>
    {:else if gameState === 'completed'}
        <div class="completed-screen">
            <div class="completed-icon">
                {#if score === questions.length}
                    🏆
                {:else}
                    🎉
                {/if}
            </div>
            <h2>Quiz Completed!</h2>
            <div class="score-card">
                <span class="score-num">{score} / {questions.length}</span>
                <p>
                    {#if score === questions.length}
                        Perfect score! You're ready to learn.
                    {:else if score > 0}
                        Great effort! Keep practicing.
                    {:else}
                        Ready to start your language learning journey?
                    {/if}
                </p>
            </div>

            <p class="cta-desc">
                Download the complete Parlez-vous application to unlock interactive roleplaying, custom journaling, visual description games, and native handwriting support.
            </p>

            <div class="completed-actions">
                <a href="https://github.com/n123xyz/parlez-vous/releases" target="_blank" rel="noopener noreferrer" class="btn btn-primary download-apk-btn">
                    Download Android APK
                    <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="7 10 12 15 17 10"></polyline><line x1="12" y1="15" x2="12" y2="3"></line></svg>
                </a>
                
                <div class="desktop-promo">
                    <span>Prefer desktop? Get the <strong>Windows / Linux / macOS</strong> version:</span>
                    <a href="https://github.com/n123xyz/parlez-vous/releases" target="_blank" rel="noopener noreferrer" class="desktop-link">
                        Download Desktop App &rarr;
                    </a>
                </div>

                <button class="btn btn-secondary" onclick={resetGame}>
                    Try Another Language
                </button>
            </div>
        </div>
    {/if}
</div>
