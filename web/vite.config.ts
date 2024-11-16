import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';

export default defineConfig({
	plugins: [sveltekit()],

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
