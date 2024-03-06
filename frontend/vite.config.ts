import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import yaml from 'yaml';
import fs from 'fs';
import aliases from './alias.json' assert { type: 'json' };
import path from 'path';

let alias: { [key: string]: string } = {};
for (const [key, value] of Object.entries(aliases)) {
	alias[key] = path.resolve(value);
}

const secrets = yaml.parse(fs.readFileSync('../secrets.yaml').toString());

process.env = {
	PUBLIC_GOOGLE_CLIENT_ID: secrets.google_auth.client_id,
	GOOGLE_CLIENT_SECRET: secrets
};
export default defineConfig({
	plugins: [sveltekit()],
	server: {
		// TODO: port env var
		proxy: {
			'/api': {
				target: 'http://localhost:3000',
				changeOrigin: false,
				secure: false,
				rewrite: (path) => path.replace(/^\/api/, '')
			}
		}
	},
	resolve: {
		alias
	}
});
