<script>
	import { Button } from 'flowbite-svelte';
	import { CentralClient } from 'central';
	import { app } from 'sdk';

	app.connect();

	async function testStuff() {
		// await app.manager.clearLocalDatabase();
		// const client = new CentralClient();
		// await client.connect();
		// await client.auth.signin('+201096707442', '69961372');
	}
</script>

<Button on:click={testStuff}>Click me</Button>
