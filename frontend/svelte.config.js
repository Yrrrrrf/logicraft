// import adapter from '@sveltejs/adapter-auto';
import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
	// 	// adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
	// 	// If your environment is not supported or you settled on a specific environment, switch out the adapter.
	// 	// See https://kit.svelte.dev/docs/adapters for more information about adapters.
		
	// 	// ? Replace adapter-auto with adapter-static to generate a static site.
	// 	// This allows you to host your site on any static hosting service.
		// adapter: adapter()
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			fallback: undefined,
			precompress: false,
			strict: true
		})
	}
};

export default config;
