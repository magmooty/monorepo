<script>
	import { Button } from 'flowbite-svelte';

	async function testStuff() {
		console.log('testStuff');
	}
</script>

<Button on:click={testStuff}>Click me</Button>
