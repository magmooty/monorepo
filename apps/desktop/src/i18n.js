import { addMessages, init } from 'svelte-i18n';

import ar from './locale/ar.json';
import en from './locale/en.json';

addMessages('ar', ar);
addMessages('en', en);

init({
	fallbackLocale: 'ar',
	initialLocale: 'ar'
});
