<script lang="ts">
	import { Button, Spinner } from 'flowbite-svelte';
	export let variant: 'primary' | 'secondary' | 'link' | 'transparent' = 'primary';
	export let loading: boolean = false;

	let classList;

	const isSecondary = variant === 'secondary';

	if (variant === 'primary') {
		classList = `hover:bg-primary bg-primary text-white`;
	} else if (variant === 'secondary') {
		classList = `text-black hover:text-back`;
	} else if (variant === 'link') {
		classList = `text-primary hover:text-primary-600 underline border-none outline-none`;
	} else if (variant === 'transparent') {
		classList = `text-primary bg-transparent border-none`;
	}
</script>

<Button class={classList} on:click outline={isSecondary} {...$$restProps}>
	{#if loading}
		<Spinner class="me-3" size="4" color="white" />
	{/if}
	<slot />
</Button>
