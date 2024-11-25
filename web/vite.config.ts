import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
	plugins: [sveltekit(), wasm(), topLevelAwait()],

	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	}

	// server: {
	// 	fs: {
	// 		// Allow serving files from one level up to the project root
	// 		allow: ['..']
	// 	}
	// }
});
