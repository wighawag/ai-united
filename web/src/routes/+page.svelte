<script lang="ts">
	import { dummy_test } from '$lib';
	import DropZone from '$lib/files/DropZone.svelte';

	function test() {
		dummy_test();
	}
</script>

<button onclick={() => test()}>Dummy Test</button>

<DropZone></DropZone>
