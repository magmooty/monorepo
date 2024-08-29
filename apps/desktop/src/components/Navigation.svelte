<script>
	import { BottomNav, BottomNavItem } from 'flowbite-svelte';
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
		<BottomNavItem appBtnPosition="right">
			<a href="/dashboard">Dashboard</a>
		</BottomNavItem>
	</BottomNav>
</div>

<style>
	#dev-nav {
		opacity: 0.5;
	}
</style>
