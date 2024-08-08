import { logger } from '$lib/logger';
import { App } from 'sdk';
import { LocalUserScope, type LocalUser, type Scope } from './user';
import type { Action, UUID } from 'surrealdb.js';

const LOG_TARGET = 'LocalAuthController';

export interface PublicUserInfo {
	name: string;
	phone_number: string;
	is_center_manager: boolean;
}

export type PermissionChangesHandler = (action: Action, scope: Scope) => Promise<void>;

export class LocalAuthController {
	constructor(private app: App) {}

	/**
	 * Lists all users in the local database
	 * @returns {Promise<LocalUser[]>} - List of users
	 */
	async listUsers(): Promise<LocalUser[]> {
		logger.info(LOG_TARGET, `Listing users`);
		const users = await this.app.rootDb.query<LocalUser[]>('SELECT name, phone_number FROM user');

		// logger.info(LOG_TARGET, `Listing managers`);
		// const scopes = await this.app.rootDb.query<Scope[]>(
		// 	'SELECT user, space, array::group(scope_name) AS scopes FROM scope GROUP BY user, space',
		// 	{ scope_name: LocalUserScope.ManageCenter }
		// );

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
