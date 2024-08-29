import { App } from 'sdk';
import { getRootDatabaseCredentials, setGlobalKey } from '$lib/bindings';
import { logger } from '$lib/logger';
import type { InfoForRoot } from './common';
import { type LocalUser, type Scope } from './user';
import type { RecordId } from 'surrealdb.js';
import { generateDatabaseSchema } from './schema';
import { LocalUserScope } from './static-types';

export interface InitializeCenterParameters {
	id: RecordId<string>;
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

		logger.info(LOG_TARGET, `Setting local global key: center_id`);
		await setGlobalKey('center_id', parameters.id.toString());

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
		const [[user]] = await this.app.rootDb.query<LocalUser[][]>(
			`CREATE user CONTENT { name: $name, phone_number: $phone_number, password: crypto::argon2::generate($password) }`,
			{
				name: admin.name,
				phone_number: admin.phone_number,
				password: admin.password
			}
		);

		await this.app.rootDb.create<Scope>('scope', {
			scope_name: LocalUserScope.ManageCenter,
			user: user.id
		});

		logger.info(LOG_TARGET, `Local admin user created`);
	}

	/**
	 * Reset the password for a local user.
	 * **CAUTION**: This is root access and should only be used for local admin user if signed in.
	 * @param phoneNumber The phone number of the user
	 * @param password The new password
	 */
	async resetLocalUserPassword(phoneNumber: string, password: string) {
		logger.info(
			LOG_TARGET,
			`Resetting password for local user through root manager ${phoneNumber}`
		);
		await this.app.rootDb.query(
			`UPDATE user SET password = crypto::argon2::generate($password) WHERE phone_number = $phone_number`,
			{
				phone_number: phoneNumber,
				password: password
			}
		);

		logger.info(LOG_TARGET, `Password reset for local user through root manager ${phoneNumber}`);
	}

	async clearLocalDatabase() {
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

	private generateDatabaseSchema() {
		logger.info(LOG_TARGET, `Generating local database schema`);

		const SyncEventCreator = (table: string) => `
			DEFINE EVENT ${table}_syncer ON TABLE ${table} THEN (
				CREATE sync CONTENT { record_id: $after.id, event: $event, content: $after, created_at: time::now() }
			);
		`;

		const schema = generateDatabaseSchema();

		const query = `
			${schema}
			
			DEFINE TABLE sync SCHEMALESS PERMISSIONS FOR CREATE FULL, FOR SELECT FULL, FOR UPDATE NONE, FOR DELETE NONE;
			DEFINE FIELD pushed ON TABLE sync TYPE bool DEFAULT false;

			${SyncEventCreator('user')}
			${SyncEventCreator('scope')}
			${SyncEventCreator('space')}
			${SyncEventCreator('academic_year')}
			${SyncEventCreator('academic_year_course')}
			${SyncEventCreator('group')}
			${SyncEventCreator('student')}
			${SyncEventCreator('enrollment')}
		`;

		return { schema, query };
	}

	async defineDatabase() {
		logger.debug(LOG_TARGET, `Defining local database namespace`);
		const { query } = this.generateDatabaseSchema();
		await this.app.rootDb.use({ namespace: 'local', database: 'local' });
		await this.app.rootDb.query(query);
	}
}
