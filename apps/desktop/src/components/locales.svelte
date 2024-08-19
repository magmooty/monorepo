<script lang="ts">
	import { _, locale } from 'svelte-i18n';
	import { Button } from 'flowbite-svelte';
	let currentLocale: string | null | undefined;

	const toggleLocale = () => {
		if (currentLocale === 'en') {
			locale.set('ar');
			toggleDirection('ar');
		} else {
			locale.set('en');
			toggleDirection('en');
		}
	};

	const toggleDirection = (locale: string) => {
		const htmlElement = document.documentElement;
		if (locale === 'ar') {
			htmlElement.setAttribute('dir', 'rtl');
		} else {
			htmlElement.setAttribute('dir', 'ltr');
		}
	};

	locale.subscribe((locale) => (currentLocale = locale));
</script>

<Button id="lang_selector" on:click={toggleLocale}>
	{#if currentLocale === 'en'}
		العربية
	{:else}
		English
	{/if}
</Button>
