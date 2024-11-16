<script lang="ts">
	import Dropzone from 'svelte-file-dropzone';
	import { files } from './files';
	import { play, battle } from '$lib/battle';
	import Battle from '$lib/battle/Battle.svelte';
	import { Button } from '$lib/components/ui/button';

	function handleFilesSelect1(e: CustomEvent) {
		const { acceptedFiles, fileRejections } = e.detail;
		console.log({ fileRejections });
		files.addFile(0, acceptedFiles[0]);
	}

	function handleFilesSelect2(e: CustomEvent) {
		const { acceptedFiles, fileRejections } = e.detail;
		console.log({ fileRejections });
		files.addFile(1, acceptedFiles[0]);
	}
</script>

{#if !$files.one || !$files.two}
	<div class="dropzone">
		{#if !$files.one}
			<Dropzone accept={['.wasm']} on:drop={handleFilesSelect1}
				><p class="drop-text">Chose bot 1</p></Dropzone
			>
		{:else}
			<div class="half">Select bot 2 now</div>
		{/if}

		{#if !$files.two}
			<Dropzone accept={['.wasm']} on:drop={handleFilesSelect2}
				><p class="drop-text">Chose the bot 2</p></Dropzone
			>
		{:else}
			<div class="half">Select bot 1 now</div>
		{/if}
	</div>
{:else if !$battle.battle}
	<div class="play">
		<Button class="w-64 text-xl font-black" onclick={() => play($files.one!, $files.two!)}
			>Play</Button
		>
	</div>
{:else}
	<Battle />
{/if}

<style>
	div :global(.dropzone) {
		background-color: #101010;
		border: 3px solid hsl(24.6 95% 53.1%);
	}
	.drop-text {
		color: white;
	}
	.half {
		width: 50%;
	}
	.dropzone {
		display: flex;
	}
	.play {
		font-size: 3rem;

		width: 100%;
		height: 100%;
		display: flex;
		justify-content: center;
		align-items: center;
	}
</style>
