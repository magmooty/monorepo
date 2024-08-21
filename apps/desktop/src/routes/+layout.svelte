<script>
	import { closeSplashScreen } from '$lib/bindings';
	import '../app.css';
	import { app } from 'sdk';
	import '../i18n';

	let connected = false;
	// Initialization logic goes here
	app.connect().then(closeSplashScreen);
</script>

<slot></slot>
