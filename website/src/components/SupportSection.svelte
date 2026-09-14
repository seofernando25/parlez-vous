<script lang="ts">
    import './styles/support-section.1.css';
    import './styles/support-section.2.css';
</script>

<div class="support-container">
    <div class="support-header">
        <span class="eyebrow">Get Involved</span>
        <h2>Support & Contribute</h2>
        <p class="subtitle">Parlez-vous? is powered by open source contributors and community support.</p>
    </div>

    <div class="support-grid">
        <!-- Developer Contribution Card -->
        <div class="support-card glassmorphic">
            <div class="card-icon">💻</div>
            <h3>For Developers</h3>
            <p>
                Whether you want to fix bugs, optimize on-device inference, improve Svelte interfaces, or extend database migrations, we welcome contributions!
            </p>
            
            <div class="guide-list">
                <div class="guide-item">
                    <span class="num">1</span>
                    <div class="text">
                        <strong>Fork & Clone</strong>
                        <span>Head over to our GitHub repository and pull the source branch.</span>
                    </div>
                </div>
                <div class="guide-item">
                    <span class="num">2</span>
                    <div class="text">
                        <strong>Tauri Setup</strong>
                        <span>Follow the compilation guide in our <a href="https://github.com/n123xyz/parlez-vous/blob/main/docs/installation.md" target="_blank" rel="noopener noreferrer">docs</a> to set up Rust, Bun, and Android Studio NDK paths.</span>
                    </div>
                </div>
                <div class="guide-item">
                    <span class="num">3</span>
                    <div class="text">
                        <strong>Submit a PR</strong>
                        <span>Create a feature branch, run local tests, and open a Pull Request. We review contributions weekly.</span>
                    </div>
                </div>
            </div>

            <a href="https://github.com/n123xyz/parlez-vous" target="_blank" rel="noopener noreferrer" class="btn btn-github">
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path></svg>
                View GitHub Repository
            </a>
        </div>

        <!-- Financial Support Card -->
        <div class="support-card glassmorphic highlighted-card">
            <div class="card-icon">☕</div>
            <h3>Buy Us a Coffee</h3>
            <p>
                Our project runs entirely on zero ads and zero paywalls. 100% of the funds go towards testing Android hardware, purchasing API keys for external evaluation, and general development infrastructure.
            </p>
            
            <p class="donation-desc">
                If <b>Parlez-vous?</b> has helped you learn a language, saved you subscription fees, or made your development workflows easier, please consider supporting the project.
            </p>

            <div class="kofi-section">
                <!-- A sleek Ko-fi widget lookalike -->
                <a href="https://ko-fi.com/parlezvous" target="_blank" rel="noopener noreferrer" class="kofi-button">
                    <img src="https://storage.ko-fi.com/cdn/cup-border.png" alt="Cup icon" class="kofi-cup" />
                    Support us on Ko-fi
                </a>
            </div>
            
            <div class="other-ways">
                <h4>Other ways to support:</h4>
                <ul>
                    <li>Give us a ⭐ on GitHub</li>
                    <li>Share the app with other language learners</li>
                    <li>Provide feedback by opening issues</li>
                </ul>
            </div>
        </div>
    </div>
</div>
