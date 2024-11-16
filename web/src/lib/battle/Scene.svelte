<script lang="ts">
	import { T } from '@threlte/core';
	import { Gizmo, Grid, OrbitControls } from '@threlte/extras';

	import {
		BoxGeometry,
		Color,
		CylinderGeometry,
		Mesh,
		MeshStandardMaterial,
		SphereGeometry
	} from 'three';

	import { battle } from '$lib/battle';

	export let autoRotate: boolean;
	export let enableDamping: boolean;
	export let rotateSpeed: number;
	export let zoomToCursor: boolean;
	export let zoomSpeed: number;
	export let minPolarAngle: number;
	export let maxPolarAngle: number;
	export let enableZoom: boolean;

	const gridSize = 20.0;
	const gridColor = Color.NAMES.gray;
	const gridSectionColor = Color.NAMES.gray;
	const gridSectionThickness = undefined;
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

<!-- <T.GridHelper args={[20, 20]} plane={'xy'} /> -->

<Grid
	plane="xz"
	gridSize={[gridSize, gridSize]}
	cellColor={gridColor}
	sectionColor={gridSectionColor}
	sectionThickness={gridSectionThickness}
	backgroundOpacity={0.0}
	type={'grid'}
/>

<Grid
	plane="xz"
	gridSize={[gridSize, gridSize]}
	cellColor={gridColor}
	sectionColor={gridSectionColor}
	sectionThickness={gridSectionThickness}
	backgroundOpacity={0.0}
	type={'grid'}
	position={[0.0, gridSize, 0.0]}
/>

<Grid
	plane="xy"
	gridSize={[gridSize, gridSize]}
	cellColor={gridColor}
	sectionColor={gridSectionColor}
	sectionThickness={gridSectionThickness}
	backgroundOpacity={0.0}
	type={'grid'}
	position={[0.0, gridSize / 2, -gridSize / 2]}
/>

<Grid
	plane="xy"
	gridSize={[gridSize, gridSize]}
	cellColor={gridColor}
	sectionColor={gridSectionColor}
	sectionThickness={gridSectionThickness}
	backgroundOpacity={0.0}
	type={'grid'}
	position={[0.0, gridSize / 2, gridSize / 2]}
/>

<Grid
	plane="zy"
	gridSize={[gridSize, gridSize]}
	cellColor={gridColor}
	sectionColor={gridSectionColor}
	sectionThickness={gridSectionThickness}
	backgroundOpacity={0.0}
	type={'grid'}
	position={[-gridSize / 2, gridSize / 2, 0.0]}
/>

<Grid
	plane="zy"
	gridSize={[gridSize, gridSize]}
	cellColor={gridColor}
	sectionColor={gridSectionColor}
	sectionThickness={gridSectionThickness}
	backgroundOpacity={0.0}
	type={'grid'}
	position={[gridSize / 2, gridSize / 2, 0.0]}
/>

<!-- 
<T.Mesh position.y={-0.01} rotation.x={-Math.PI / 2}>
	<T.CircleGeometry args={[10, 64]} />
	<T.ShaderMaterial
		uniforms={{
			uSize: { value: 10.0 },
			uLineWidth: { value: 0.1 },
			uLineColor: { value: new Color(0x888888) },
			uGridColor: { value: new Color(0xcccccc) }
		}}
		vertexShader={`
        varying vec2 vUv;
        void main() {
          vUv = uv;
          gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
        }
      `}
		fragmentShader={`
        uniform float uSize;
        uniform float uLineWidth;
        uniform vec3 uLineColor;
        uniform vec3 uGridColor;
        varying vec2 vUv;
        void main() {
          vec2 coord = vUv * uSize;
          vec2 grid = abs(fract(coord - 0.5) - 0.5) / fwidth(coord);
          float line = min(grid.x, grid.y);
          float color = 1.0 - min(line, 1.0);
          gl_FragColor = vec4(mix(uGridColor, uLineColor, color), 1.0);
        }
      `}
	/>
</T.Mesh> -->

<!-- bot 1 -->
<T.Mesh
	position.y={$battle.bot1.y}
	position.x={$battle.bot1.x}
	position.z={$battle.bot1.z}
	geometry={new SphereGeometry(0.5)}
	material={new MeshStandardMaterial({
		color: Color.NAMES.blueviolet
	})}
/>

<!-- bot 2 -->
<T.Mesh
	position.y={$battle.bot2.y}
	position.x={$battle.bot2.x}
	position.z={$battle.bot2.z}
	geometry={new SphereGeometry(0.5)}
	material={new MeshStandardMaterial({
		color: Color.NAMES.firebrick
	})}
/>

<!-- ball -->
<T.Mesh
	position.y={$battle.ball.y}
	position.x={$battle.ball.x}
	position.z={$battle.ball.z}
	geometry={new SphereGeometry(0.5)}
	material={new MeshStandardMaterial({
		color: Color.NAMES.white
	})}
/>

<!-- <T.Mesh
	position.y={2.5}
	position.x={-9.5}
	position.z={9.5}
	geometry={new CylinderGeometry(0, 0.5, 5)}
	material={new MeshStandardMaterial({
		color: Color.NAMES.blueviolet
	})}
/>

<T.Mesh
	position.y={2.5}
	position.x={9.5}
	position.z={-9.5}
	geometry={new CylinderGeometry(0, 0.5, 5)}
	material={new MeshStandardMaterial({
		color: Color.NAMES.firebrick
	})}
/> -->

<T.Mesh
	position.y={2.5}
	position.x={-10.4}
	position.z={0}
	geometry={new BoxGeometry(1.0, 5.0, 5.0)}
	material={new MeshStandardMaterial({
		color: Color.NAMES.blueviolet
	})}
/>

<T.Mesh
	position.y={2.5}
	position.x={10.4}
	position.z={0}
	geometry={new BoxGeometry(1.0, 5.0, 5.0)}
	material={new MeshStandardMaterial({
		color: Color.NAMES.firebrick
	})}
/>
