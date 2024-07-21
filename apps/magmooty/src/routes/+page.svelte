<script>
	import { Button } from 'flowbite-svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	let info = '';

	async function fetchInfo() {
		const response = await invoke('query', {
			path: '/whatsapp/info',
			body: {}
		});
		info = response;
	}

	async function startConnection() {
		const response = await invoke('query', {
			path: '/whatsapp/start_connection',
			body: {}
		});
		console.log(response);
		info = response;
	}

	async function sendMessage() {
		const response = await invoke('query', {
			path: '/whatsapp/send_message',
			body: {
				phone_number: '+201070671580',
				message: 'Hello world!'
			}
		});
		console.log(response);
		info = response;
	}
</script>

{info}
<Button on:click={fetchInfo}>Fetch info</Button>
<Button on:click={startConnection}>Start connection</Button>
<Button on:click={sendMessage}>Send message</Button>
