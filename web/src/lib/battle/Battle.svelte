<script lang="ts">
	import { Canvas } from '@threlte/core';

	import Scene from './Scene.svelte';
	import Settings from './Settings.svelte';

	import { battle } from '$lib/battle';

	let autoRotate: boolean = false;
	let enableDamping: boolean = true;
	let rotateSpeed: number = 1;
	let zoomToCursor: boolean = false;
	let zoomSpeed: number = 1;
	let minPolarAngle: number = 0;
	let maxPolarAngle: number = Math.PI;
	let enableZoom: boolean = true;
</script>

<div class="canvas">
	<Canvas>
		<Scene
			{enableDamping}
			{autoRotate}
			{rotateSpeed}
			{zoomToCursor}
			{zoomSpeed}
			{minPolarAngle}
			{maxPolarAngle}
			{enableZoom}
		/>
	</Canvas>
</div>

<Settings
	bind:enableDamping
	bind:autoRotate
	bind:rotateSpeed
	bind:zoomToCursor
	bind:zoomSpeed
	bind:minPolarAngle
	bind:maxPolarAngle
	bind:enableZoom
/>

<div class="overlay">
	<!-- <p>{$battle.step}</p>
	<p>{JSON.stringify($battle.bot1)}</p>
	<p>{JSON.stringify($battle.bot2)}</p>
	<p>{JSON.stringify($battle.ball)}</p> -->

	{#if $battle.winner}
		<p>Winner: {$battle.winner}</p>
	{/if}
</div>

<style>
	.canvas {
		position: relative;
		height: 100%;
		width: 100%;
		background-color: rgb(14, 22, 37);
	}

	.overlay {
		background-color: black;
		color: white;
		bottom: 0;
		right: 0;
		z-index: 1;
		position: absolute;
	}
</style>
