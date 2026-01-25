import { mdsvex } from 'mdsvex';
import adapter from '@sveltejs/adapter-static';

import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: { adapter: adapter() },
	preprocess: [mdsvex({
        layout: join(__dirname, './src/layouts/blog-post.layout.svelte'),
        highlight: {
            alias: {
                "rs": "rust",
            },
        },
    })],
	extensions: ['.svelte', '.svx']
};

export default config;
