<script>
	import Button from 'components/common/button/Button.svelte';
	import Locales from 'components/locales.svelte';
	import { ArrowRightOutline } from 'flowbite-svelte-icons';
</script>

<h1>Home</h1>

<div class="m-4">
	<h1>Default</h1>
	<div class="align-center mt-5 flex gap-4">
		<Button>Default</Button>
		<Button variant="secondary">Secondary</Button>
		<Button variant="transparent">Transparent</Button>
		<Button variant="link">Link</Button>
	</div>
</div>

<div class="m-4">
	<h1>With Icons</h1>
	<div class="align-center mt-6 flex gap-4">
		<Button><ArrowRightOutline class="text-primary-700 h-7 w-7" />Default</Button>
		<Button variant="secondary"
			><ArrowRightOutline class="text-primary-700 h-7 w-7" />Secondary</Button
		>
		<Button variant="transparent">
			<ArrowRightOutline class="text-primary-700 h-7 w-7" />Transparent</Button
		>
		<Button variant="link"><ArrowRightOutline class="text-primary-700 h-7 w-7" />Link</Button>
	</div>
</div>

<div class="m-4">
	<h1>Loading</h1>
	<div class="align-center mt-5 flex gap-4">
		<Button loading>Default</Button>
		<Button loading variant="secondary">Secondary</Button>
		<Button loading variant="transparent">Transparent</Button>
		<Button loading variant="link">Link</Button>
	</div>
</div>

<div class="m-4">
	<h1>Disabled</h1>
	<div class="align-center mt-5 flex gap-4">
		<Button disabled>Default</Button>
		<Button disabled variant="secondary">Secondary</Button>
		<Button disabled variant="transparent">Transparent</Button>
		<Button disabled variant="link">Link</Button>
	</div>
</div>
