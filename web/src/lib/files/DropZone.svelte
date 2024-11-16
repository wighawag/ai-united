<script lang="ts">
	import Dropzone from 'svelte-file-dropzone';
	import { files } from './files';
	import { play, battle } from '$lib/battle';

	function handleFilesSelect(e: CustomEvent) {
		const { acceptedFiles, fileRejections } = e.detail;
		console.log({ fileRejections });
		files.addFiles(acceptedFiles);
	}
</script>

{#if $files.length < 2}
	<Dropzone accept={['.wasm']} on:drop={handleFilesSelect} />
{:else}
	<button onclick={() => play($files[0], $files[1])}>Play</button>
	<p>{$battle.step}</p>
{/if}
