import { addMessages, init } from 'svelte-i18n';

import ar from './locales/ar.json';

addMessages('ar', ar);

init({
	fallbackLocale: 'ar',
	initialLocale: 'ar'
});
