import { Surreal } from 'surrealdb.js';
import { StudentsController } from './students';
import { LocalDatabaseManager } from './manager';
import { getRootDatabaseCredentials } from '$lib/bindings';
import { isSurrealConnectionError } from 'common/surreal';
import { logger } from '$lib/logger';
import { LocalAuthController } from './auth';
import { SpaceController } from './space';

const LOG_TARGET = 'sdk';

export class App {
	public rootDb: Surreal;
	public db: Surreal;

	public auth: LocalAuthController;
	public manager: LocalDatabaseManager;
	public students: StudentsController;
	public spaces: SpaceController;

	constructor() {
		this.rootDb = new Surreal();
		this.db = new Surreal();
		this.auth = new LocalAuthController(this);
		this.manager = new LocalDatabaseManager(this);
		this.students = new StudentsController(this);
		this.spaces = new SpaceController(this);
	}

	/**
	 * Connect to the local SurrealDB server
	 */
	async connect() {
		// Connect to Root
		await this.rootDb.connect('http://127.0.0.1:5004/rpc').catch((err) => {
			if (isSurrealConnectionError(err)) {
				// This is a critical error, we have to report it
				logger.error(LOG_TARGET, 'Failed to connect for root to local SurrealDB server');
			}

			throw err;
		});

		const rootCredentials = await getRootDatabaseCredentials();

		await this.rootDb.use({ namespace: 'local', database: 'local' });
		await this.rootDb.signin(rootCredentials);

		// Connect to user
		await this.db.connect('http://127.0.0.1:5004/rpc').catch((err) => {
			if (isSurrealConnectionError(err)) {
				// This is a critical error, we have to report it
				logger.error(LOG_TARGET, 'Failed to connect for user to local SurrealDB server');
			}

			throw err;
		});

		await this.db.use({ namespace: 'local', database: 'local' });
	}
}

export const app: App = new App();
