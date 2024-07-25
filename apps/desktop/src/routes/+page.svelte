<script>
	import { Button } from 'flowbite-svelte';
	import { whatsappGetInfo, whatsappSendMessage, whatsappStartConnection } from '$lib/whatsapp';
	import { app } from 'sdk';
	// import { PhoneNumberUse } from 'sdk/common';

	let info = '';

	async function createStudent() {
		await app.connect();
		// await app.students.create({
		// 	name: 'Ziad',
		// 	phone_numbers: [
		// 		{
		// 			use: PhoneNumberUse.Student,
		// 			number: '+201070671580'
		// 		}
		// 	]
		// });
	}

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
<Button on:click={createStudent}>Create student</Button>
<Button on:click={fetchInfo}>Fetch info</Button>
<Button on:click={startConnection}>Start connection</Button>
<Button on:click={sendMessage}>Send message</Button>
