<script lang="ts">
	import { _, locale } from 'svelte-i18n';
	import { Button } from 'flowbite-svelte';
	let currentLocale: string | null | undefined;

	const toggleLocale = () => {
		if (currentLocale === 'en') {
			locale.set('ar');
		} else {
			locale.set('en');
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
