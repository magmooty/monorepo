import { App } from 'sdk';
import { getRootDatabaseCredentials, setGlobalKey } from '$lib/bindings';
import { logger } from '$lib/logger';
import type { InfoForRoot } from './common';
import { LocalUserScope, type LocalUser, type Scope } from './user';
import type { RecordId } from 'surrealdb.js';

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

	private async defineDatabase() {
		logger.info(LOG_TARGET, `Defining local namespace`);
		await this.app.rootDb.use({ namespace: 'local', database: 'local' });

		const CenterManagerScope = `(SELECT * FROM scope WHERE user = $auth.id AND scope_name = '${LocalUserScope.ManageCenter}')`;
		const SpaceManagerScope = `space IN (SELECT space FROM scope WHERE user = $auth.id AND scope_name = '${LocalUserScope.ManageSpace}' GROUP BY space)`;
		const SpaceMemberScope = `space IN (SELECT space FROM scope WHERE user = $auth.id GROUP BY space)`;

		const SyncEventCreator = (table: string) => `
			DEFINE EVENT ${table}_syncer ON TABLE ${table} THEN (
				CREATE sync CONTENT { record_id: $after.id, event: $event, content: $after, created_at: time::now() }
			);
		`;

		const CustomScope = (scope: LocalUserScope) =>
			`space IN (SELECT space FROM scope WHERE user = $auth.id AND scope_name = '${scope}' GROUP BY space)`;

		await this.app.rootDb.query(`
			DEFINE TABLE user SCHEMAFULL
				PERMISSIONS
					FOR SELECT FULL, // All users can see other users
					FOR UPDATE WHERE id = $auth.id OR ${CenterManagerScope}, // User can update themselves
					FOR CREATE, DELETE WHERE (SELECT * FROM scope WHERE user = $auth.id AND (scope_name = '${LocalUserScope.ManageCenter}' OR scope_name = 'manage_space')); // Center manager can create other users, A space owner can create users
			DEFINE FIELD name ON TABLE user TYPE string;
			DEFINE FIELD phone_number ON TABLE user TYPE string;
			DEFINE FIELD password ON TABLE user TYPE string PERMISSIONS FOR SELECT NONE;
			
			DEFINE TABLE scope SCHEMAFULL
				PERMISSIONS
					FOR SELECT FULL, // All users can see other users' scopes
					FOR CREATE, UPDATE, DELETE WHERE ${CenterManagerScope} OR (SELECT * FROM scope WHERE user = $auth.id AND scope_name = 'manage_space' AND space = $input.space); // Center manager can modify other users' scopes, A space owner can modify users' scopes if the scopes are for an owned space
			DEFINE FIELD user ON TABLE scope TYPE record<user>;
			DEFINE FIELD space ON TABLE scope TYPE option<record<space>>;
			DEFINE FIELD scope_name ON TABLE scope TYPE string;

			DEFINE SCOPE local_user SESSION 24h
        SIGNIN ( SELECT * FROM user WHERE phone_number = $phone_number AND crypto::argon2::compare(password, $password) );

			DEFINE TABLE space SCHEMAFULL
				PERMISSIONS
					FOR SELECT FULL,
					FOR CREATE, DELETE WHERE ${CenterManagerScope},
					FOR UPDATE WHERE ${CenterManagerScope} OR id IN (SELECT space FROM scope WHERE user = $auth.id AND scope_name = 'manage_space');
			DEFINE FIELD name ON TABLE space TYPE string;

			DEFINE TABLE academic_year SCHEMAFULL
				PERMISSIONS
					FOR SELECT WHERE ${CenterManagerScope} OR ${SpaceMemberScope},
					FOR CREATE, UPDATE, DELETE WHERE ${CenterManagerScope} OR ${SpaceManagerScope} OR ${CustomScope(LocalUserScope.ManageAcademicYears)};
			DEFINE FIELD year ON TABLE academic_year TYPE number;
			DEFINE FIELD space ON TABLE academic_year TYPE record<space>;

			DEFINE TABLE academic_year_course SCHEMAFULL
				PERMISSIONS
					FOR SELECT WHERE ${CenterManagerScope} OR ${SpaceMemberScope},
					FOR CREATE, UPDATE, DELETE WHERE ${CenterManagerScope} OR ${SpaceManagerScope} OR ${CustomScope(LocalUserScope.ManageAcademicYearCourses)};
			DEFINE FIELD grade ON TABLE academic_year_course TYPE string;
			DEFINE FIELD subjects ON TABLE academic_year_course TYPE array<string>;
			DEFINE FIELD academic_year ON TABLE academic_year_course TYPE record<academic_year>;
			DEFINE FIELD space ON TABLE academic_year_course TYPE record<space>;

			DEFINE TABLE group SCHEMAFULL
				PERMISSIONS
					FOR SELECT WHERE ${CenterManagerScope} OR ${SpaceMemberScope},
					FOR CREATE, UPDATE, DELETE WHERE ${CenterManagerScope} OR ${SpaceManagerScope} OR ${CustomScope(LocalUserScope.ManageGroups)};
			DEFINE FIELD schedule ON TABLE group FLEXIBLE TYPE array<object>;
			DEFINE FIELD academic_year ON TABLE group TYPE record<academic_year>;
			DEFINE FIELD space ON TABLE group TYPE record<space>;

			DEFINE ANALYZER name_analyzer TOKENIZERS blank FILTERS edgengram(2,10);

			DEFINE TABLE student SCHEMALESS
				PERMISSIONS
					FOR SELECT WHERE ${CenterManagerScope} OR ${SpaceMemberScope},
					FOR CREATE, UPDATE, DELETE WHERE ${CenterManagerScope} OR ${SpaceManagerScope} OR ${CustomScope(LocalUserScope.ManageStudents)};
			DEFINE FIELD name ON TABLE student TYPE string;
			DEFINE FIELD _name ON TABLE student TYPE string;

			DEFINE INDEX student_name_index ON student FIELDS _name SEARCH ANALYZER name_analyzer BM25;

			DEFINE EVENT student_enrollment_syncer ON TABLE student WHEN $before.name != $after.name THEN (
				UPDATE enrollment SET name = $after.name, _name = $after.name WHERE student = $after.id
			);

			DEFINE TABLE enrollment SCHEMAFULL
				PERMISSIONS
					FOR SELECT WHERE ${CenterManagerScope} OR ${SpaceMemberScope},
					FOR CREATE, UPDATE, DELETE WHERE ${CenterManagerScope} OR ${SpaceManagerScope} OR ${CustomScope(LocalUserScope.ManageStudents)};
			DEFINE FIELD name ON TABLE enrollment TYPE string; 
			DEFINE FIELD _name ON TABLE enrollment TYPE string; 
			DEFINE FIELD student ON TABLE enrollment TYPE record<student>; 
			DEFINE FIELD default_group ON TABLE enrollment TYPE record<group>;
			DEFINE FIELD academic_year ON TABLE enrollment TYPE record<academic_year>;
			DEFINE FIELD course ON TABLE enrollment TYPE record<academic_year_course>;
			DEFINE FIELD space ON TABLE enrollment TYPE record<space>;

			DEFINE INDEX enrollment_student_name_index ON enrollment FIELDS _name SEARCH ANALYZER name_analyzer BM25;

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
		`);
	}
}
