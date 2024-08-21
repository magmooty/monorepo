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
				// flowbite-svelte
				primary: {
					50: '#FFF5F2',
					100: '#FFF1EE',
					200: '#FFE4DE',
					300: '#FFD5CC',
					400: '#FFBCAD',
					500: '#FE795D',
					600: '#EF562F',
					700: '#EB4F27',
					800: '#CC4522',
					900: '#A5371B',
					"blue-50": "#0B8CD266",
				}

			}, fontFamily: {
				'noto-kufi': ['"Noto Kufi Arabic"', 'sans-serif'], // Add the custom font family here
			},
		}
	},

	plugins: [flowbitePlugin, tailwindcssRtl]
} as Config;
