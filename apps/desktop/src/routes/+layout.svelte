<script>
	import { closeSplashScreen } from '$lib/bindings';
	import '../app.css';
	import { app } from 'sdk';
	import '../i18n';

	// Initialization logic goes here
	app.connect().then(closeSplashScreen);
</script>

<slot></slot>
