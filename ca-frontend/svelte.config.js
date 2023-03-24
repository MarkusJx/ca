import adapter from '@sveltejs/adapter-node';
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import("@sveltejs/kit").Config} */
const config = {
	preprocess: vitePreprocess(),
	kit: {
		adapter: adapter(),
	},
	vite: {
		ssr: {
			noExternal: [/^@smui(?:-extra)?\//],
		},
	},
};

export default config;
