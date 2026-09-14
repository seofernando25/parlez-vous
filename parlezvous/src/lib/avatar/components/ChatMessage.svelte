<script lang="ts">
    import { marked } from 'marked';
    import type { AvatarChatMessage } from '../types';
    let { message }: { message: AvatarChatMessage } = $props();
    const cleanAssistant = (text: string) => text
        .replace(/<start_of_turn>\s*model\s*/g, '')
        .replace(/<start_of_turn>|<end_of_turn>/g, '')
        .replace(/[<\[](?:anim:[a-zA-Z0-9_-]+|laugh|breath|sigh|surprise|sad|cough|cry|whisper|yell|gasp|sneeze|sniff)[>\]]/g, '');
</script>

{#if message.role !== 'system'}
    <div class="message-row {message.role === 'user' ? 'user' : 'assistant'}">
        <div class="message-bubble">
            {#if message.role === 'assistant'}
                <div class="message-markdown">{@html marked.parse(cleanAssistant(message.content))}</div>
            {:else}
                <p>{message.content}</p>
                {#if message.audioBase64}<audio src="data:audio/wav;base64,{message.audioBase64}" controls></audio>{/if}
            {/if}
        </div>
        {#if message.role === 'user' && message.correction}
            <div class="correction"><span>Correction</span><p>{message.correction}</p></div>
        {/if}
    </div>
{/if}

<style>
    .message-row { display:flex; flex-direction:column; align-items:flex-start; }
    .message-row.user { align-items:flex-end; }
    .message-bubble { max-width:85%; border:1px solid var(--pv-border); border-radius:1rem; background:var(--pv-surface-raised); color:var(--pv-foreground); padding:.65rem .85rem; }
    .user .message-bubble { border-color:transparent; background:var(--pv-accent); color:var(--pv-on-accent); }
    .message-bubble p { margin:0; font-size:.88rem; line-height:1.5; }
    .message-bubble audio { width:min(16rem,100%); height:2rem; margin-top:.5rem; opacity:.85; }
    .message-markdown { font-size:.88rem; line-height:1.55; }
    .message-markdown :global(p) { margin:.25rem 0; }
    .message-markdown :global(p:first-child) { margin-top:0; }.message-markdown :global(p:last-child) { margin-bottom:0; }
    .correction { max-width:80%; margin:.35rem .3rem 0; border-left:2px solid var(--pv-success); padding-left:.55rem; }
    .correction span { display:block; color:var(--pv-success); font-size:.62rem; font-weight:800; letter-spacing:.06em; text-transform:uppercase; }
    .correction p { margin:.15rem 0 0; color:var(--pv-muted); font-size:.75rem; font-style:italic; line-height:1.4; }
</style>
