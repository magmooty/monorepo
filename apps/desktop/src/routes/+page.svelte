<script>
	import { Button } from 'flowbite-svelte';
	import { CentralClient } from 'central';
	import { app } from 'sdk';
	import Login from '../components/Login.svelte';

	app.connect();

	async function testStuff() {
		// await app.manager.clearLocalDatabase();
		// const client = new CentralClient();
		// await client.connect();
		// await client.auth.signin('+201096707442', '69961372');
	}
</script>

<Login />
