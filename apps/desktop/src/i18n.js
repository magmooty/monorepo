import { addMessages, init, getLocaleFromNavigator } from 'svelte-i18n';

import en from './locales/en.json';
import ar from './locales/ar.json';

addMessages('en', en);
addMessages('ar', ar);

init({
	fallbackLocale: 'en',
	initialLocale: getLocaleFromNavigator()
});
