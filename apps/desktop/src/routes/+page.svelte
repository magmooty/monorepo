<script>
	import { Button } from 'flowbite-svelte';
	import { CentralClient } from 'central';

	async function testStuff() {
		const client = new CentralClient();
		await client.connect();
		await client.auth.signin('+201096707442', '69961372');
	}
</script>

<Button on:click={testStuff}>Click me</Button>
