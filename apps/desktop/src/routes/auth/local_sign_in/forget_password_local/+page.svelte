<script lang="ts">
	import { _ } from 'svelte-i18n';
	import UserSelector from 'components/LocalSignIn/UserSelector.svelte';
	import Button from 'components/common/button/Button.svelte';
	import { app } from 'sdk';
	import type { LocalUserWithPermissionsToResetPassword } from 'sdk/auth';
	let users: LocalUserWithPermissionsToResetPassword[] = [];
	let phoneNumber: string = '1234567890';
	async function fetchUsersWhoCanRestPass(phoneNumber: string) {
		try {
			const fetchedUsers = await app.auth.whoCanResetPasswordFor(phoneNumber);
			users = fetchedUsers;
		} catch (error) {
			//TODO: Handle error
			console.log('Failed to fetch users:', error);
		}
	}
	fetchUsersWhoCanRestPass(phoneNumber);
</script>

<div class=" flex h-screen w-full flex-col items-center justify-center">
	<h1 class="text-primary mb-12 w-full text-center text-2xl">
		{$_('localOnboarding.whoCanResetYourPassword')}
	</h1>
	<div class="w-max-[247px]">
		<UserSelector {users} />
		<Button class="w-full">{$_('common.back')}</Button>
	</div>
</div>
