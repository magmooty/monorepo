<script lang="ts">
	import LocalSignInLayout from 'components/localSignIn/LocalSignInLayout.svelte';
	import UserSelector from 'components/localSignIn/UserSelector.svelte';
	import { app } from 'sdk';
	import type { PublicUserInfo } from 'sdk/auth';
	let users: PublicUserInfo[] = [];
	async function fetchUsers() {
		try {
			const fetchedUsers = await app.auth.listUsers();
			users = fetchedUsers;
		} catch (error) {
			//TODO: Handle error
			console.error('Failed to fetch users:', error);
		}
	}
	fetchUsers();
</script>

<LocalSignInLayout>
	<UserSelector {users} slot="user-selector" />
</LocalSignInLayout>
