<script>
	import SetSignUpRemotePassword from 'components/SetSignUpRemotePassword.svelte';
	import { _ } from 'svelte-i18n';
	import { Card } from 'flowbite-svelte';
</script>

<div class="flex h-screen w-screen flex-col items-center justify-center gap-5">
	<h1 class="text-primary text-4xl">{$_('onboarding.welcomeHeader')}</h1>
	<h2>{$_('onboarding.setRemotePasswordSubHeader')}</h2>

	<div class="mt-5">
		<Card>
			<SetSignUpRemotePassword />
		</Card>
	</div>
</div>
