import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    preprocess: vitePreprocess(),

    kit: {
        adapter: adapter(),
        alias: {
            $styles: 'src/styles',
            $lib: 'src/lib',
            $components: 'src/lib/components',
            $types: 'src/app.d.ts'
        }
    }
};

export default config;
