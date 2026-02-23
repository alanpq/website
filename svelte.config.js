import { mdsvex } from 'mdsvex';
import gfm from 'remark-gfm';
import rehypeSlug from 'rehype-slug';
import remarkToc from 'remark-toc';
import remarkFootnotes from 'remark-footnotes';

import adapter from '@sveltejs/adapter-static';

import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: { adapter: adapter() },
	preprocess: [
		mdsvex({
			layout: join(__dirname, './src/layouts/blog-post.layout.svelte'),
			remarkPlugins: [
				[remarkToc, { tight: true }],
				[remarkFootnotes, {}]
			],
			rehypePlugins: [rehypeSlug],
			highlight: {
				alias: {
					rs: 'rust'
				}
			},
			extensions: ['.svx', '.md']
		})
	],
	extensions: ['.svelte', '.svx', '.md']
};

export default config;
