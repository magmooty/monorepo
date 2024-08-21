import { App } from 'sdk';
import { LocalUserScope } from 'sdk/user';
import { RecordId } from 'surrealdb.js';
import { beforeAll, describe, expect, it, vi } from 'vitest';

beforeAll(() => {
	vi.mock('$lib/bindings', () => {
		return {
			setGlobalKey: () => {},
			getRootDatabaseCredentials: () => ({ username: 'root', password: 'root' })
		};
	});
});

describe('Auth', () => {
	let app: App;
	let space: RecordId<string>;
	let spaceManagerUser: RecordId<string>;

	beforeAll(async () => {
		app = new App('http://127.0.0.1:7002/rpc');

		await app.connect();

		await app.manager.initializeCenter({
			center_name: 'Test Center',
			id: new RecordId('center', 'test-center'),
			private_key: 'private',
			public_key: 'public'
		});

		await app.manager.initializeLocalAdmin({
			name: 'Test Admin',
			password: '0000',
			phone_number: '+201096707442'
		});

		await app.auth.signIn('+201096707442', '0000');

		space = (await app.spaces.create('Test Space')).id;
	});

	it('Can signin as admin user', async () => {
		await app.auth.signIn('+201096707442', '0000');
	});

	it('Can create a new user', async () => {
		spaceManagerUser = (
			await app.users.create({
				name: 'Test User',
				phone_number: '+201151002051',
				password: '0000'
			})
		).id;
	});

	it('Can create a new scope for new user', async () => {
		await app.users.scopes.create({
			scope_name: LocalUserScope.ManageSpace,
			user: spaceManagerUser,
			space
		});
	});

	it('Can list scopes and manage space scope exists', async () => {
		const scopes = await app.users.scopes.list(spaceManagerUser);

		expect(scopes).toHaveLength(1);
		expect(scopes).toEqual(
			expect.arrayContaining([
				expect.objectContaining({
					scope_name: LocalUserScope.ManageSpace,
					space
				})
			])
		);
	});

	it('Lists all users', async () => {
		const users = await app.auth.listUsers();

		expect(users).toHaveLength(2);

		expect(users).toEqual(
			expect.arrayContaining([
				expect.objectContaining({ phone_number: '+201096707442', is_center_manager: true }),
				expect.objectContaining({
					phone_number: '+201151002051',
					is_center_manager: false,
					manages_spaces: ['Test Space']
				})
			])
		);
	});

	it('Can log in a new user (space manager)', async () => {
		await app.auth.signIn('+201151002051', '0000');
	});

	it('Who can reset password for space manager', async () => {
		const users = await app.auth.whoCanResetPasswordFor('+201151002051');

		expect(users).toHaveLength(1);
		expect(users).toEqual(
			expect.arrayContaining([expect.objectContaining({ phone_number: '+201096707442' })])
		);
	});

	it('Can create secretary user', async () => {
		const secretaryUser = (
			await app.users.create({
				name: 'Test Secretary',
				phone_number: '+201151002052',
				password: '0000'
			})
		).id;

		await app.users.scopes.create({
			scope_name: LocalUserScope.ManageStudents,
			user: secretaryUser,
			space
		});
	});

	it('Can log in a new user (secretary)', async () => {
		await app.auth.signIn('+201151002052', '0000');
	});

	it('Who can reset password for secretary', async () => {
		const users = await app.auth.whoCanResetPasswordFor('+201151002052');

		const queryResult = await app.db.query(
			`SELECT * FROM scope`,
			{ phone_number: '+201151002052' }
		);

		console.log(queryResult);

		expect(users).toHaveLength(2);
		expect(users).toEqual(
			expect.arrayContaining([
				expect.objectContaining({ phone_number: '+201096707442' }),
				expect.objectContaining({ phone_number: '+201151002051' })
			])
		);
	});
});
