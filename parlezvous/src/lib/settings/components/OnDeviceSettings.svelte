<script lang="ts">
    import type { SettingsController } from '../controller.svelte';
    import DownloadAssetCard from './DownloadAssetCard.svelte';
    let { controller }: { controller: SettingsController } = $props();
</script>
{#if controller.isAndroid}
<div class="mt-2 border-t border-border pt-2">
    <DownloadAssetCard title="Language model" ready={controller.litertExists} downloading={controller.litertDownloading} progress={controller.litertProgress} onDownload={() => controller.downloadLitert()} onPurge={() => controller.purgeLitert()} onVerify={() => controller.checkLitert()} />
    <DownloadAssetCard title="Speech model" ready={controller.supertonicReady} downloading={controller.supertonicDownloading} progress={controller.supertonicProgress} onDownload={() => controller.downloadSupertonic()} onPurge={() => controller.purgeSupertonic()} onVerify={() => controller.checkSupertonic()} />
    <DownloadAssetCard title="Tokenizer" ready={controller.tokenizerExists} downloading={controller.tokenizerDownloading} onDownload={() => controller.downloadTokenizer()} onPurge={() => controller.purgeTokenizer()} onVerify={() => controller.checkTokenizer()} />
</div>
{/if}
