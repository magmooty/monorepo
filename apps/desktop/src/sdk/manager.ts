import { App } from 'sdk';
import { getRootDatabaseCredentials, setGlobalKey } from '$lib/bindings';
import { logger } from '$lib/logger';
import type { InfoForRoot } from './common';
import { LocalUserScope, type LocalUser, type Scope } from './user';

export interface InitializeCenterParameters {
	id: string;
	center_name: string;
	public_key: string;
	private_key: string;
}

export interface InitializeLocalAdminParameters {
	name: string;
	phone_number: string;
	password: string;
}

const LOG_TARGET = 'LocalDatabaseManager';

export class LocalDatabaseManager {
	constructor(private app: App) {}

	async initializeCenter(parameters: InitializeCenterParameters): Promise<void> {
		logger.info(LOG_TARGET, `Switching center to ${parameters.center_name}`);

		logger.info(LOG_TARGET, `Clearing local database`);
		await this.clearLocalDatabase();

		logger.info(LOG_TARGET, `Defining local database`);
		await this.defineDatabase();

		logger.info(LOG_TARGET, `Setting local global key: center_name`);
		await setGlobalKey('center_name', parameters.center_name);

		logger.info(LOG_TARGET, `Setting local global key: private_key`);
		await setGlobalKey('private_key', parameters.private_key);

		logger.info(LOG_TARGET, `Setting local global key: public_key`);
		await setGlobalKey('public_key', parameters.public_key);

		logger.info(LOG_TARGET, `Setting local global key: public_key`);
		await setGlobalKey('instance_type', 'master');

		logger.error(LOG_TARGET, `Downloading data from the Central API is not implemented yet`);
	}

	async initializeLocalAdmin(admin: InitializeLocalAdminParameters) {
		logger.info(LOG_TARGET, `Creating local admin user ${admin.phone_number}`);
		const [user] = await this.app.rootDb.create<LocalUser>('user', {
			name: admin.name,
			phone_number: admin.phone_number,
			password: admin.password
		});

		await this.app.rootDb.create<Scope>('scope', {
			scope_name: LocalUserScope.ManageCenter,
			user: user.id
		});
	}

	private async clearLocalDatabase() {
		logger.info(LOG_TARGET, `Fetching all namespaces`);
		const [rootInfo] = await this.app.rootDb.query<[InfoForRoot]>('INFO FOR ROOT;');

		const namespaces = Object.keys(rootInfo.namespaces);
		logger.warn(LOG_TARGET, `Deleting ${namespaces.length} namespaces ${namespaces.join(', ')}`);

		for (const namespace of namespaces) {
			logger.warn(LOG_TARGET, `Deleting namespace ${namespace}`);
			await this.app.rootDb.query(`REMOVE NAMESPACE IF EXISTS ${namespace}`);
		}

		logger.warn(LOG_TARGET, `Deleted all existing namespaces in local database`);

		const users = Object.keys(rootInfo.users);
		logger.warn(LOG_TARGET, `Deleting ${users.length} users ${users.join(', ')}`);

		const rootUser = (await getRootDatabaseCredentials()).username;

		for (const user of users) {
			if (user === rootUser) {
				logger.info(LOG_TARGET, `Skipping deletion of root user`);
				continue;
			}

			logger.warn(LOG_TARGET, `Deleting user ${user}`);
			await this.app.rootDb.query(`REMOVE USER IF EXISTS ${user}`);
		}

		logger.warn(LOG_TARGET, `Deleted all existing users in local database`);

		logger.warn(LOG_TARGET, `Local database cleared`);
	}

	private async defineDatabase() {
		logger.info(LOG_TARGET, `Defining local namespace`);
		await this.app.rootDb.use({ namespace: 'local', database: 'local' });

		await this.app.rootDb.query(`
			DEFINE TABLE user SCHEMAFULL
				PERMISSIONS
					FOR SELECT FULL, // All users can see other users
					FOR UPDATE WHERE id = $auth.id, // User can update themselves
					FOR CREATE, DELETE WHERE (SELECT * FROM scope WHERE user = $auth.id AND (scope_name = '${LocalUserScope.ManageCenter}' OR scope_name = 'manage_space')); // Center manager can create other users, A space owner can create users
			DEFINE FIELD name ON TABLE user TYPE string;
			DEFINE FIELD phone_number ON TABLE user TYPE string;
			DEFINE FIELD password ON TABLE user TYPE string PERMISSIONS FOR SELECT NONE;
			
			DEFINE TABLE scope SCHEMAFULL
				PERMISSIONS
					FOR SELECT FULL, // All users can see other users' scopes
					FOR CREATE, UPDATE, DELETE WHERE (SELECT * FROM scope WHERE user = $auth.id AND scope_name = '${LocalUserScope.ManageCenter}') OR (SELECT * FROM scope WHERE user = $auth.id AND scope_name = 'manage_space' AND space = $input.space); // Center manager can modify other users' scopes, A space owner can modify users' scopes if the scopes are for an owned space
			DEFINE FIELD user ON TABLE scope TYPE record<user>;
			DEFINE FIELD space ON TABLE scope TYPE option<record<space>>;
			DEFINE FIELD scope_name ON TABLE scope TYPE string;

			DEFINE TABLE space SCHEMAFULL
				PERMISSIONS
					FOR SELECT FULL,
					FOR CREATE WHERE (SELECT * FROM scope WHERE user = $auth.id AND scope_name = '${LocalUserScope.ManageCenter}'),
					FOR UPDATE WHERE (SELECT * FROM scope WHERE user = $auth.id AND scope_name = '${LocalUserScope.ManageCenter}') OR id IN (SELECT space FROM scope WHERE user = $auth.id AND scope_name = 'manage_space');
			DEFINE FIELD name ON TABLE space TYPE string;

			DEFINE SCOPE local_user SESSION 24h
        SIGNIN ( SELECT * FROM user WHERE phone_number = $phone_number AND password = $password );
			
			DEFINE TABLE tutor SCHEMAFULL;
			DEFINE FIELD name ON TABLE tutor TYPE string;
			DEFINE FIELD phone_number ON TABLE tutor TYPE string;
			DEFINE FIELD subjects ON TABLE tutor TYPE array<string>;

			DEFINE TABLE student SCHEMALESS;
			DEFINE FIELD name ON TABLE student string;
			DEFINE FIELD phone_number ON TABLE student string;
		`);
	}
}
