<script lang="ts">
    import { Trash2, Volume2, VolumeX, Compass, BookOpen, PenLine, Mic, Send, LoaderCircle, Upload } from 'lucide-svelte';
    import type { AvatarChatController } from '../chat.svelte';
    import type { HandwritingController } from '../handwriting/controller.svelte';
    import type { TextbookController } from '../textbooks.svelte';
    import type { VoiceCaptureController } from '../voice.svelte';
    import type { ViewMode } from '../types';
    import ChatMessage from './ChatMessage.svelte';
    import HandwritingPanel from './HandwritingPanel.svelte';

    let { chat, voice, handwriting, textbooks, viewMode, mapFollowMode, muteTts, onToggleMap, onToggleMute }: {
        chat: AvatarChatController; voice: VoiceCaptureController; handwriting: HandwritingController; textbooks: TextbookController;
        viewMode: ViewMode; mapFollowMode: boolean; muteTts: boolean;
        onToggleMap: () => void; onToggleMute: () => void;
    } = $props();
    let textbookMenuOpen = $state(false);

    function toggleHandwriting() {
        handwriting.show = !handwriting.show;
        if (!handwriting.show) return;
        chat.inputRef?.blur();
        setTimeout(() => { handwriting.initCanvas(); chat.scrollContainer?.scrollTo({ top: chat.scrollContainer.scrollHeight, behavior: 'smooth' }); }, 50);
    }

    function sendFromKeyboard(event: KeyboardEvent) {
        if (event.key !== 'Enter' || event.shiftKey) return;
        event.preventDefault();
        if (!chat.isChatting && (chat.input.trim() || voice.isActive)) void chat.send();
    }
</script>

<div class="chat-panel {handwriting.show ? 'handwriting-open' : ''} {viewMode === 'avatar' ? 'hidden' : ''} {viewMode === 'chat' ? 'chat-only' : ''}">
    <div class="chat-toolbar">
        <div class="toolbar-spacer"></div>
        <div class="toolbar-actions">
            <button class="tool" onclick={() => chat.clear()} title="Clear chat" aria-label="Clear chat"><Trash2 size={16} /></button>
            <button class="tool" class:active={muteTts} onclick={onToggleMute} title={muteTts ? 'Enable speech' : 'Mute speech'} aria-label={muteTts ? 'Enable speech' : 'Mute speech'}>{#if muteTts}<VolumeX size={16} />{:else}<Volume2 size={16} />{/if}</button>
            <button class="tool" class:active={mapFollowMode} onclick={onToggleMap} title="Curriculum steering" aria-label="Curriculum steering"><Compass size={16} /></button>
            <div class="textbook-menu">
                <button class="tool" class:active={Boolean(textbooks.active)} onclick={() => textbookMenuOpen = !textbookMenuOpen} title="Textbooks" aria-label="Textbooks">{#if textbooks.isIngesting}<LoaderCircle size={16} class="animate-spin" />{:else}<BookOpen size={16} />{/if}</button>
                {#if textbookMenuOpen}
                    <div class="textbook-popover">
                        <button onclick={() => { textbooks.select(null); textbookMenuOpen = false; }}>None</button>
                        {#each textbooks.books as book}<button class:active={textbooks.active === book} onclick={() => { textbooks.select(book); textbookMenuOpen = false; }} title={book}>{book}</button>{/each}
                        <button class="upload" onclick={() => { textbooks.upload(); textbookMenuOpen = false; }}><Upload size={15} /> Upload</button>
                    </div>
                {/if}
            </div>
        </div>
    </div>

    <div class="messages" bind:this={chat.scrollContainer}>
        {#if chat.history.length === 0}<div class="chat-empty">…</div>{:else}<div class="mt-auto"></div>{/if}
        {#each chat.history as message}<ChatMessage {message} />{/each}
        {#if chat.isChatting}<div class="typing"><span></span><span></span><span></span></div>{/if}
    </div>

    <div class="composer">
        {#if voice.isActive}<span class="voice-state">{voice.state === 'speaking' ? 'Listening' : 'Processing'}</span>{/if}
        <form onsubmit={(event) => { event.preventDefault(); void chat.send(); }}>
            {#if handwriting.isSupported}<button type="button" class="tool composer-tool" class:active={handwriting.show} onclick={toggleHandwriting} title="Handwriting" aria-label="Handwriting"><PenLine size={18} /></button>{/if}
            <button type="button" class="tool composer-tool" class:active={voice.isActive} onclick={voice.toggle} title="Voice" aria-label="Voice"><Mic size={18} /></button>
            <textarea bind:this={chat.inputRef} bind:value={chat.input} placeholder={voice.isActive ? 'Speak…' : 'Message…'} rows="1" oninput={(event) => { event.currentTarget.style.height='auto'; event.currentTarget.style.height=`${event.currentTarget.scrollHeight}px`; }} onkeydown={sendFromKeyboard}></textarea>
            <button type="submit" class="send" disabled={chat.isChatting || (!chat.input.trim() && !voice.isActive)} title="Send" aria-label="Send"><Send size={18} /></button>
        </form>
    </div>

    {#if handwriting.show}<HandwritingPanel {handwriting} onClose={() => handwriting.show = false} />{/if}
</div>

<style>
    .chat-panel { position:relative; display:flex; min-width:0; min-height:0; flex:1; flex-direction:column; overflow:hidden; border:1px solid var(--pv-border); border-radius:1rem; background:var(--pv-surface); }
    .chat-panel.hidden { display:none; }
    .chat-toolbar { display:flex; flex:0 0 auto; align-items:center; justify-content:space-between; gap:.5rem; padding:.55rem; border-bottom:1px solid var(--pv-border); }
    .toolbar-spacer { flex:1; }
    .toolbar-actions { display:flex; align-items:center; gap:.15rem; }
    .tool { display:grid; width:2.2rem; height:2.2rem; place-items:center; border:0; border-radius:.65rem; background:transparent; color:var(--pv-subtle); cursor:pointer; }
    .tool:hover { background:var(--pv-surface-raised); color:var(--pv-foreground); }
    .tool.active { background:var(--pv-accent-soft); color:var(--pv-accent-strong); }
    .textbook-menu { position:relative; }
    .textbook-popover { position:absolute; z-index:20; top:calc(100% + .4rem); right:0; width:13rem; max-height:16rem; overflow:auto; border:1px solid var(--pv-border); border-radius:.8rem; background:var(--pv-surface); padding:.35rem; box-shadow:var(--pv-shadow); }
    .textbook-popover button { display:flex; width:100%; align-items:center; gap:.45rem; overflow:hidden; border:0; border-radius:.55rem; background:transparent; color:var(--pv-muted); padding:.55rem .6rem; text-align:left; font:inherit; font-size:.76rem; text-overflow:ellipsis; white-space:nowrap; cursor:pointer; }
    .textbook-popover button:hover,.textbook-popover button.active { background:var(--pv-surface-raised); color:var(--pv-foreground); }
    .textbook-popover .upload { border-top:1px solid var(--pv-border); border-radius:0; margin-top:.25rem; padding-top:.7rem; color:var(--pv-accent-strong); }
    .messages { display:flex; min-height:0; flex:1; flex-direction:column; gap:.8rem; overflow-y:auto; padding:1rem; }
    .chat-empty { display:grid; flex:1; place-items:center; color:var(--pv-subtle); font-size:.78rem; }
    .typing { display:flex; width:max-content; gap:.25rem; border:1px solid var(--pv-border); border-radius:999px; padding:.55rem .75rem; }
    .typing span { width:.35rem; height:.35rem; border-radius:999px; background:var(--pv-subtle); animation:bounce 1s infinite alternate; }
    .typing span:nth-child(2) { animation-delay:.15s; }.typing span:nth-child(3) { animation-delay:.3s; }
    .composer { flex:0 0 auto; border-top:1px solid var(--pv-border); padding:.55rem; }
    .composer form { display:flex; align-items:flex-end; gap:.3rem; }
    .composer textarea { min-width:0; max-height:7rem; flex:1; resize:none; border:0; background:transparent; color:var(--pv-foreground); padding:.55rem .45rem; font:inherit; font-size:.9rem; line-height:1.4; outline:none; }
    .composer-tool { flex:0 0 auto; }
    .send { display:grid; width:2.3rem; height:2.3rem; flex:0 0 2.3rem; place-items:center; border:0; border-radius:.7rem; background:var(--pv-accent); color:var(--pv-on-accent); cursor:pointer; }
    .send:disabled { opacity:.3; }
    .voice-state { display:block; padding:0 .4rem .35rem; color:var(--pv-subtle); font-size:.68rem; font-weight:750; text-transform:uppercase; letter-spacing:.06em; }
    @keyframes bounce { to { transform:translateY(-3px); } }
    @media (min-width:768px) { .chat-panel { width:56%; flex:1 1 56%; border-radius:0; border-block:0; border-right:0; } .chat-panel.chat-only { width:100%; flex-basis:100%; border-left:0; } .messages { padding:1.1rem; } }
</style>
