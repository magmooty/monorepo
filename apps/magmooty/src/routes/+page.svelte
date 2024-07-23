<script>
	import { Button } from 'flowbite-svelte';
	import { whatsappGetInfo, whatsappSendMessage, whatsappStartConnection } from '$lib/bindings';

	let info = '';

	async function fetchInfo() {
		try {
			const response = await whatsappGetInfo();
			info = JSON.stringify(response);
		} catch (error) {
			console.error(error);
		}
	}

	async function startConnection() {
		try {
			const response = await whatsappStartConnection();
			info = JSON.stringify(response);
		} catch (error) {
			console.error(error);
		}
	}

	async function sendMessage() {
		try {
			const response = await whatsappSendMessage({
				phone_number: '+201070671580',
				message: 'Hello world!'
			});
			info = JSON.stringify(response);
		} catch (error) {
			console.error(error);
		}
	}
</script>

{info}
<Button on:click={fetchInfo}>Fetch info</Button>
<Button on:click={startConnection}>Start connection</Button>
<Button on:click={sendMessage}>Send message</Button>
