<script>
	import { Label, Input, Button } from 'flowbite-svelte';
	import { MobilePhoneSolid } from 'flowbite-svelte-icons';
	import { CentralClient } from '../central';
	import { MessagingChannel } from 'common';
	const client = new CentralClient();

	let phoneNumber = '+201069392983';
	let code = '';
	let isCodeSent = false;
	let isSignedIn = false;

	async function sendCode() {
		client.auth.sendSigninCode(phoneNumber, MessagingChannel.WhatsApp).then(() => {
			isCodeSent = true;
		});
	}

	async function signin() {
		await client.connect();
		await client.auth
			.signin(phoneNumber, code)
			.then((res) => {
				console.log('Signed in', res);
				isSignedIn = true;
			})
			.catch(console.log);
	}
</script>

<div class="flex h-screen flex-col items-center justify-center">
	{#if isSignedIn}
		<h1>{phoneNumber} User Is Signed In</h1>
	{:else}
		<div>
			<Label class="mb-2 block text-center">Your phone number</Label>
			<Input type="tel" bind:value={phoneNumber} class="w-100">
				<MobilePhoneSolid slot="left" class="h-5 w-5 text-gray-500 dark:text-gray-400" />
			</Input>
		</div>
		{#if !isCodeSent}
			<Button on:click={sendCode} class="mt-5">Send code</Button>
		{/if}

		{#if isCodeSent}
			<Label class="my-2 block">Code</Label>
			<Input type="number" bind:value={code} class="w-200" />
			<Button on:click={signin} class="mt-5">Sign in</Button>
		{/if}
	{/if}
</div>
