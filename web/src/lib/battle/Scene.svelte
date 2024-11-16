<script lang="ts">
	import { T } from '@threlte/core';
	import { Gizmo, OrbitControls } from '@threlte/extras';

	import { BoxGeometry, MeshStandardMaterial, SphereGeometry } from 'three';

	import { battle } from '$lib/battle';

	export let autoRotate: boolean;
	export let enableDamping: boolean;
	export let rotateSpeed: number;
	export let zoomToCursor: boolean;
	export let zoomSpeed: number;
	export let minPolarAngle: number;
	export let maxPolarAngle: number;
	export let enableZoom: boolean;
</script>

<T.PerspectiveCamera makeDefault position={[10, 5, 10]} lookAt.y={0.5}>
	<OrbitControls
		{enableDamping}
		{autoRotate}
		{rotateSpeed}
		{zoomToCursor}
		{zoomSpeed}
		{minPolarAngle}
		{maxPolarAngle}
		{enableZoom}
	/>
</T.PerspectiveCamera>

<Gizmo horizontalPlacement="left" paddingX={20} paddingY={20} />

<T.DirectionalLight position.y={10} position.z={10} />
<T.AmbientLight intensity={0.3} />

<T.GridHelper args={[10, 10]} />

<!-- bot 1 -->
<T.Mesh
	position.y={$battle.bot1.y}
	position.x={$battle.bot1.x}
	position.z={$battle.bot1.z}
	geometry={new SphereGeometry(0.5)}
	material={new MeshStandardMaterial()}
/>

<!-- bot 2 -->
<T.Mesh
	position.y={$battle.bot2.y}
	position.x={$battle.bot2.x}
	position.z={$battle.bot1.z}
	geometry={new SphereGeometry(0.5)}
	material={new MeshStandardMaterial()}
/>

<!-- ball -->
<T.Mesh
	position.y={$battle.ball.y}
	position.x={$battle.ball.x}
	position.z={$battle.bot1.z}
	geometry={new SphereGeometry(0.5)}
	material={new MeshStandardMaterial()}
/>
