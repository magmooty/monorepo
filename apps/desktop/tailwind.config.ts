import flowbitePlugin from 'flowbite/plugin';
import tailwindcssRtl from 'tailwindcss-rtl';

import type { Config } from 'tailwindcss';

export default {
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		'./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'
	],

	theme: {
		extend: {
			colors: {
				primary: '#0B8CD2'
			},
			fontFamily: {
				'noto-kufi': ['"Noto Kufi Arabic"', 'sans-serif']
			}
		}
	},

	plugins: [flowbitePlugin, tailwindcssRtl]
} as Config;
