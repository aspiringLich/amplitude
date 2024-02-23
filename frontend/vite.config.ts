import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import yaml from 'yaml';
import fs from 'fs';

const secrets = yaml.parse(fs.readFileSync('../secrets.yaml').toString());

process.env = {
	"PUBLIC_GOOGLE_CLIENT_ID": secrets.google_auth.client_id,
	"GOOGLE_CLIENT_SECRET": secrets,
};
export default defineConfig({
	plugins: [sveltekit()]
});

