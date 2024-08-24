<script>
	import { closeSplashScreen } from '$lib/bindings';
	import '../app.css';
	import { app } from 'sdk';
	import '../i18n';
	import Navigation from '../components/Navigation.svelte';

	let connected = false;

	// Initialization logic goes here
	app.connect().then(() => {
		closeSplashScreen();
		connected = true;
	});
</script>

{#if connected}
	<Navigation />
	<slot></slot>
{/if}
