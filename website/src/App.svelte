<script lang="ts">
    import './styles/app.1.css';
    import './styles/app.2.css';
    import ConjugationGame from './components/ConjugationGame.svelte';
    import LanguageCarousel from './components/LanguageCarousel.svelte';
    import AboutSection from './components/AboutSection.svelte';
    import DocsSection from './components/DocsSection.svelte';
    import SupportSection from './components/SupportSection.svelte';

    type Tab = 'welcome' | 'about' | 'docs' | 'support';
    let activeTab = $state<Tab>('welcome');

    function setTab(tab: Tab) {
        activeTab = tab;
        // Reset scroll position
        window.scrollTo({ top: 0, behavior: 'smooth' });
    }

    const videoUrl = `${import.meta.env.BASE_URL}parlezvous.mp4`;

    // Shared state for game interaction
    let selectedLangCode = $state('fr');
    let startTrigger = $state(0);
    let isMuted = $state(true);

    function selectLanguageAndStart(code: string) {
        selectedLangCode = code;
        startTrigger += 1;
        
        // Smooth scroll up to the conjugation game card
        const gameElement = document.querySelector('.game-card');
        if (gameElement) {
            gameElement.scrollIntoView({ behavior: 'smooth', block: 'center' });
        }
    }
</script>

<div class="app-layout">
    <!-- Header Navigation -->
    <header class="app-header">
        <div class="logo-container" onclick={() => setTab('welcome')} role="button" tabindex="0" onkeydown={(e) => e.key === 'Enter' && setTab('welcome')}>
            <div class="logo-icon">P</div>
            <span class="logo-text">Parlez<span class="yellow-text">Vous</span></span>
        </div>
        <nav class="nav-links">
            <button class="nav-btn" class:active={activeTab === 'welcome'} onclick={() => setTab('welcome')}>Welcome</button>
            <button class="nav-btn" class:active={activeTab === 'about'} onclick={() => setTab('about')}>About</button>
            <button class="nav-btn" class:active={activeTab === 'docs'} onclick={() => setTab('docs')}>Docs</button>
            <button class="nav-btn" class:active={activeTab === 'support'} onclick={() => setTab('support')}>Support</button>
        </nav>
        <div class="header-actions">
            <a href="https://github.com/n123xyz/parlez-vous" target="_blank" rel="noopener noreferrer" class="github-icon-link" aria-label="GitHub Repository">
                <svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path></svg>
            </a>
        </div>
    </header>

    <!-- Main Content Area -->
    <main class="main-content">
        {#if activeTab === 'welcome'}
            <div class="welcome-container">
                <!-- Two-Panel Section -->
                <div class="two-panel-grid">
                    <!-- Left Panel: Phone Mockup / Demo Placeholder -->
                    <div class="panel panel-left">
                        <div class="phone-mockup">
                            <div class="phone-speaker"></div>
                            <div class="phone-screen">
                                <!-- svelte-ignore a11y_media_has_caption -->
                                <video class="phone-video" src={videoUrl} autoplay loop playsinline bind:muted={isMuted} controls></video>
                                <div class="screen-content overlay-content">
                                    <button class="unmute-overlay-btn" onclick={(e) => { e.stopPropagation(); isMuted = !isMuted; }} aria-label={isMuted ? "Unmute video" : "Mute video"}>
                                        {#if isMuted}
                                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="1" y1="1" x2="23" y2="23"></line><path d="M9 9v6a3 3 0 0 0 3 3h1.586l4.707 4.707A1 1 0 0 0 20 22V4a1 1 0 0 0-1.707-.707L13.586 8H12a3 3 0 0 0-3 3z"></path></svg>
                                            <span class="unmute-text">Tap to Unmute</span>
                                        {:else}
                                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon><path d="M19.07 4.93a10 10 0 0 1 0 14.14M15.54 8.46a5 5 0 0 1 0 7.07"></path></svg>
                                        {/if}
                                    </button>

                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- Right Panel: Interactive Conjugation Game -->
                    <div class="panel panel-right">
                        <div class="panel-game-container">
                            <ConjugationGame bind:selectedLangCode bind:startTrigger />
                        </div>
                    </div>
                </div>

                <!-- Carousel at the bottom of Welcome tab -->
                <div class="languages-section">
                    <span class="sec-badge">Supported Languages</span>
                    <h3>Practice in 30+ Languages</h3>
                    <LanguageCarousel onSelect={selectLanguageAndStart} />
                </div>
            </div>
        {:else if activeTab === 'about'}
            <AboutSection />
        {:else if activeTab === 'docs'}
            <DocsSection />
        {:else if activeTab === 'support'}
            <SupportSection />
        {/if}
    </main>

    <!-- Footer -->
    <footer class="app-footer">
        <p>&copy; 2026 Parlez-vous. Built with Svelte, Tauri, and Rust.</p>
    </footer>
</div>
