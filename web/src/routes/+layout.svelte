<script lang="ts">
	import '../app.css';
	// let { children } = $props();
</script>

<!-- {@render children()} -->
<!-- required due to wasm issue : https://github.com/sveltejs/svelte/issues/13155-->
<slot />
