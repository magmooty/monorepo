<script>
	import { BottomNav, BottomNavItem } from 'flowbite-svelte';
	import Locales from './locales.svelte';
</script>

<div id="dev-nav">
	<BottomNav position="absolute" navType="application" classInner="grid-cols-5">
		<BottomNavItem appBtnPosition="left">
			<a href="/">Home</a>
		</BottomNavItem>
		<BottomNavItem appBtnPosition="middle">
			<a href="/auth/local_sign_in">localSignIn</a>
		</BottomNavItem>

		<BottomNavItem appBtnPosition="middle">
			<a href="/auth/remote_sign_in">remoteSignIn</a>
		</BottomNavItem>

		<BottomNavItem appBtnPosition="middle">
			<a href="/auth/local_sign_in/forget_password_local">ForgetPassLocal</a>
		</BottomNavItem>

		<BottomNavItem appBtnPosition="right">
			<a href="/dashboard">Dashboard</a>
		</BottomNavItem>
		<Locales />
	</BottomNav>
</div>

<style>
	#dev-nav {
		opacity: 0.5;
	}
</style>
