<script lang="ts">
    let { theme, onSkip, onAccept }: {
        theme: { id: string; name: string };
        onSkip: () => void;
        onAccept: (theme: { id: string; name: string }) => void;
    } = $props();
</script>

<div class="review-overlay">
    <div class="review-dialog" role="dialog" aria-modal="true" aria-labelledby="review-title">
        <h2 id="review-title">Review {theme.name}?</h2>
        <p>A short refresher before continuing.</p>
        <div class="review-actions"><button onclick={onSkip}>Not now</button><button class="primary" onclick={() => onAccept(theme)}>Review</button></div>
    </div>
</div>

<style>
    .review-overlay { position:absolute; inset:0; z-index:50; display:grid; place-items:center; background:color-mix(in oklch,var(--pv-canvas) 62%,transparent); padding:1rem; backdrop-filter:blur(10px); }
    .review-dialog { width:min(100%,22rem); border:1px solid var(--pv-border); border-radius:1rem; background:var(--pv-surface); padding:1.25rem; box-shadow:var(--pv-shadow); }
    .review-dialog h2 { margin:0; color:var(--pv-foreground); font-size:1.05rem; font-weight:800; }
    .review-dialog p { margin:.4rem 0 1.2rem; color:var(--pv-muted); font-size:.82rem; }
    .review-actions { display:flex; justify-content:flex-end; gap:.45rem; }
    .review-actions button { border:0; border-radius:.7rem; background:var(--pv-surface-raised); color:var(--pv-muted); padding:.6rem .8rem; font:inherit; font-size:.78rem; font-weight:750; cursor:pointer; }
    .review-actions button.primary { background:var(--pv-accent); color:var(--pv-on-accent); }
</style>
