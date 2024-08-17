<script>
	import { CentralClient } from 'central';
	import { app } from 'sdk';
	import { City, Country, Governorate } from 'common';
	import Login from '../components/Login.svelte';
	import Locales from '../components/locales.svelte';

	async function testStuff() {
		await app.manager.clearLocalDatabase();
		const client = new CentralClient();
		await client.connect();
		await client.auth.signin('+201096707442', '45918907');
		const parameters = await client.center.create({
			name: 'Test Center',
			address: {
				line1: '15th St',
				city: City.Damanhour,
				state: Governorate.Beheira,
				country: Country.Egypt
			}
		});
		await app.manager.initializeCenter(parameters);
		await app.manager.initializeLocalAdmin({
			name: 'Ziad',
			password: '123456',
			phone_number: '+201096707442'
		});
	}
</script>

<Login />
<Locales />
