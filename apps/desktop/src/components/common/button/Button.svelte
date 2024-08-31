<script lang="ts">
	import { Button, Spinner } from 'flowbite-svelte';
	export let variant: 'primary' | 'secondary' | 'link' | 'transparent' = 'primary';
	export let loading: boolean = false;

	const primaryClassList = `hover:bg-primary bg-primary text-white`;
	const secondaryClassList = `text-black hover:text-back`;
	const linkClassList = `text-primary hover:text-primary-600 underline border-none outline-none`;
	const transparentClassList = `text-primary bg-transparent border-none`;

	const isSecondary = variant === 'secondary';

	function getVariantClasses() {
		switch (variant) {
			case 'primary':
				return primaryClassList;
			case 'secondary':
				return secondaryClassList;
			case 'link':
				return linkClassList;
			case 'transparent':
				return transparentClassList;
			default:
				return primaryClassList;
		}
	}
</script>

<Button class={getVariantClasses()} on:click outline={isSecondary} {...$$restProps}>
	{#if loading}
		<Spinner class="me-3" size="4" color="white" />
	{/if}
	<slot />
</Button>
