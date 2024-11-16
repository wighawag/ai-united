<script lang="ts">
	import Dropzone from 'svelte-file-dropzone';
	import { files } from './files';
	import { play, battle } from '$lib/battle';
	import Battle from '$lib/battle/Battle.svelte';

	function handleFilesSelect(e: CustomEvent) {
		const { acceptedFiles, fileRejections } = e.detail;
		console.log({ fileRejections });
		files.addFiles(acceptedFiles);
	}
</script>

{#if $files.length < 2}
	<Dropzone accept={['.wasm']} on:drop={handleFilesSelect} />
{:else if !$battle.battle}
	<button onclick={() => play($files[0], $files[1])}>Play</button>
{:else}
	<Battle />
{/if}
