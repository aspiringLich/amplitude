import { preprocessMeltUI, sequence } from '@melt-ui/pp';
import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import aliases from './alias.json' assert { type: 'json' };
import path from 'path';

let alias = {};
for (const [key, value] of Object.entries(aliases)) {
	alias[key] = path.resolve(value);
}

function importer(url) {
	for ([a, path] of Object.entries(alias)) {
		if (url.startsWith(alias)) {
			return { file: url.replace(alias, path) };
		}
	}
}

/** @type {import('@sveltejs/kit').Config}*/
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: [
		sequence([
			vitePreprocess({
				importer
			}),
			preprocessMeltUI()
		])
	],
	kit: {
		// adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
		// If your environment is not supported or you settled on a specific environment, switch out the adapter.
		// See https://kit.svelte.dev/docs/adapters for more information about adapters.
		adapter: adapter(),
		alias
	}
};
export default config;
