import { logger } from '$lib/logger';
import { App } from 'sdk';
import { LocalUserScope, type LocalUser, type Scope } from './user';
import type { Action, RecordId, UUID } from 'surrealdb.js';
import type { Space } from './space';

const LOG_TARGET = 'LocalAuthController';

export interface PublicUserInfo {
	id: RecordId<string>;
	name: string;
	phone_number: string;
	member_of_spaces: string[];
	manages_spaces: string[];
	is_center_manager: boolean;
}

export type LocalUserWithPermissionsToResetPassword = Omit<
	PublicUserInfo,
	'member_of_spaces' | 'manages_spaces'
>;

export type PermissionChangesHandler = (action: Action, scope: Scope) => Promise<void>;

export class LocalAuthController {
	constructor(private app: App) {}

	async signIn(phone_number: string, password: string): Promise<void> {
		logger.info(LOG_TARGET, `Signing in user with phone number ${phone_number}`);
		await this.app.db.signin({
			namespace: 'local',
			database: 'local',
			scope: 'local_user',
			phone_number,
			password
		});
	}

	async whoCanResetPasswordFor(
		phone_number: string
	): Promise<LocalUserWithPermissionsToResetPassword[]> {
		logger.info(LOG_TARGET, `Checking who can reset password for ${phone_number}`);

		const users: Map<RecordId<string>, LocalUserWithPermissionsToResetPassword> = new Map();

		logger.info(LOG_TARGET, `Checking center managers`);
		const [managers] = await this.app.rootDb.query<Scope[][]>(
			`SELECT user.id, user.phone_number, user.name FROM scope WHERE scope_name = '${LocalUserScope.ManageCenter}' FETCH user`
		);

		managers.forEach((manager) => {
			const user = manager.user as LocalUserWithPermissionsToResetPassword;
			users.set(user.id, user);
		});

		logger.info(LOG_TARGET, `Checking space managers`);
		const [space_managers] = await this.app.rootDb.query<Scope[][]>(
			`SELECT user.id, user.name, user.phone_number, space FROM scope WHERE scope_name = '${LocalUserScope.ManageSpace}' AND space IN (SELECT space FROM scope WHERE user IN (SELECT id FROM user WHERE phone_number = $phone_number).id).space FETCH user`,
			{ phone_number }
		);

		space_managers.forEach((manager) => {
			const user = manager.user as LocalUserWithPermissionsToResetPassword;

			// Skip if it's the same user
			if (user.phone_number == phone_number) return;

			users.set(user.id, user);
		});

		return Array.from(users.values());
	}

	/**
	 * Lists all users in the local database
	 * @returns {Promise<LocalUser[]>} - List of users
	 */
	async listUsers(): Promise<PublicUserInfo[]> {
		logger.info(LOG_TARGET, `Listing users`);
		const [users] = await this.app.rootDb.query<PublicUserInfo[][]>(
			'SELECT id, name, phone_number FROM user'
		);

		logger.info(LOG_TARGET, `Listing center managers`);
		const [managers] = await this.app.rootDb.query<Scope[][]>(
			`SELECT user FROM scope WHERE scope_name = '${LocalUserScope.ManageCenter}'`
		);

		logger.info(LOG_TARGET, `Listing space managers`);
		const [space_managers] = await this.app.rootDb.query<Scope[][]>(
			`SELECT user, space FROM scope WHERE scope_name = '${LocalUserScope.ManageSpace}' FETCH space`
		);

		logger.info(LOG_TARGET, `Listing space members`);
		const [space_members] = await this.app.rootDb.query<Scope[][]>(
			`SELECT user, space FROM scope WHERE scope_name NOT IN ['${LocalUserScope.ManageCenter}', '${LocalUserScope.ManageSpace}'] AND space != NONE GROUP BY user, space FETCH space;`
		);

		users.map((user) => {
			user.is_center_manager = managers.some(
				(manager) => manager.user.toString() === user.id.toString()
			);

			user.manages_spaces = space_managers
				.filter((manager) => manager.user.toString() === user.id.toString())
				.map((manager) => (manager.space as Space).name);

			user.member_of_spaces = space_members
				.filter((member) => member.user.toString() === user.id.toString())
				.map((member) => (member.space as Space).name);

			// Filter out managed spaces to make sure there are no duplicates
			user.member_of_spaces = user.member_of_spaces.filter(
				(space) => !user.manages_spaces.includes(space)
			);
		});

		return users;
	}

	/**
	 * Must be logged to receive permission changes
	 * @param callback {PermissionChangesHandler} - Callback to be called when a permission change is detected
	 */
	async listenForPermissionChanges(callback: PermissionChangesHandler): Promise<void> {
		const [liveQueryId] = await this.app.db.query<UUID[]>(
			'LIVE SELECT * FROM scope WHERE user = $auth.id'
		);

		await this.app.db.subscribeLive(liveQueryId, async (action, record: Scope) => {
			logger.info(
				LOG_TARGET,
				`Permission change detected: ${action} ${record.scope_name} for space ${record.space}`
			);
			await callback(action, record);
		});
	}
}
