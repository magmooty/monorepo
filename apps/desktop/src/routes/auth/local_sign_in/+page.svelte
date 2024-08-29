<script>
	import LocalSignInLayout from 'components/localSignIn/LocalSignInLayout.svelte';
	import UserSelector from 'components/localSignIn/UserSelector.svelte';
</script>

<LocalSignInLayout>
	<UserSelector slot="user-selector" />
</LocalSignInLayout>
