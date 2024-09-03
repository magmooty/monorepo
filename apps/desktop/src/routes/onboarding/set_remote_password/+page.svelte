<script>
	import SetSignUpRemotePassword from 'components/SetSignUpRemotePassword.svelte';
	import { _ } from 'svelte-i18n';
</script>

<div class="flex h-screen w-screen flex-col items-center justify-center gap-5">
	<h1 class="text-4xl text-primary">{$_('onboarding.welcomeHeader')}</h1>
	<h2>{$_('onboarding.setRemotePasswordSubHeader')}</h2>

	<div class="mt-5">
		<SetSignUpRemotePassword />
	</div>
</div>
