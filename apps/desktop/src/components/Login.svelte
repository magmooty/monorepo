<script>
	import { Label, Input, Button } from 'flowbite-svelte';
	import { MobilePhoneSolid } from 'flowbite-svelte-icons';
	import { CentralClient } from '../central';
	import { MessagingChannel } from 'common';
	import { _ } from 'svelte-i18n';
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

<div class="flex h-screen flex-col items-center justify-center text-center">
	{#if isSignedIn}
		<h1>{phoneNumber}{$_('onboarding.isSignedIn')}</h1>
	{:else}
		<div>
			<Label class="mb-2 block text-center">{$_('onboarding.yourPhoneNumber')}</Label>
			<Input type="tel" bind:value={phoneNumber} class="w-100">
				<MobilePhoneSolid slot="left" class="h-5 w-5 text-gray-500 dark:text-gray-400" />
			</Input>
		</div>
		{#if !isCodeSent}
			<Button on:click={sendCode} class="mt-5">{$_('onboarding.sendCode')}</Button>
		{/if}

		{#if isCodeSent}
			<Label class="my-2 block">{$_('onboarding.code')}</Label>
			<Input type="number" bind:value={code} class="w-200" />
			<Button on:click={signin} class="mt-5">{$_('onboarding.signIn')}</Button>
		{/if}
	{/if}
</div>
