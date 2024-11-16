<script lang="ts">
	import { dummy_test } from '$lib';

	function test() {
		dummy_test();
	}
</script>

<button onclick={() => test()}>Dummy Test</button>
