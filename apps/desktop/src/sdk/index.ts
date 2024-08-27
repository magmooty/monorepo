import { Surreal } from 'surrealdb.js';
import { StudentsController } from './student';
import { LocalDatabaseManager } from './manager';
import { getRootDatabaseCredentials } from '$lib/bindings';
import { isSurrealConnectionError } from 'common/surreal';
import { logger } from '$lib/logger';
import { LocalAuthController } from './auth';
import { SpaceController } from './space';
import { AcademicYearCourseController } from './academic-year-course';
import { AcademicYearController } from './academic-year';
import { EnrollmentsController } from './enrollment';
import { GroupController } from './group';
import { UserController } from './user';
import { AppEventHandler } from './events';

const LOG_TARGET = 'sdk';

export class App {
	public rootDb: Surreal;
	public db: Surreal;

	public auth: LocalAuthController;
	public manager: LocalDatabaseManager;
	public students: StudentsController;
	public spaces: SpaceController;
	public academicYears: AcademicYearController;
	public academicYearCourses: AcademicYearCourseController;
	public groups: GroupController;
	public enrollments: EnrollmentsController;
	public users: UserController;
	public events: AppEventHandler;

	private surrealDbUrl = 'http://127.0.0.1:5004/rpc';

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	constructor(private testingUrl?: any) {
		this.db = new Surreal();
		this.rootDb = new Surreal();

		this.auth = new LocalAuthController(this);
		this.manager = new LocalDatabaseManager(this);
		this.students = new StudentsController(this);
		this.spaces = new SpaceController(this);
		this.academicYears = new AcademicYearController(this);
		this.academicYearCourses = new AcademicYearCourseController(this);
		this.groups = new GroupController(this);
		this.enrollments = new EnrollmentsController(this);
		this.users = new UserController(this);
		this.events = new AppEventHandler();
	}

	/**
	 * Connect to the local SurrealDB server
	 */
	async connect() {
		if (this.testingUrl) {
			this.surrealDbUrl = this.testingUrl;
		}

		// Connect to Root
		await this.rootDb.connect(this.surrealDbUrl).catch((err) => {
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
		await this.db.connect(this.surrealDbUrl).catch((err) => {
			if (isSurrealConnectionError(err)) {
				// This is a critical error, we have to report it
				logger.error(LOG_TARGET, 'Failed to connect for user to local SurrealDB server');
			}

			throw err;
		});

		await this.db.use({ namespace: 'local', database: 'local' });
	}
}

export let app: App = new App();

// Hot reloading cleanup logic
if (import.meta.hot) {
	import.meta.hot.accept();
	import.meta.hot.dispose(() => {
		logger.debug('CLEANUP', 'Cleaning up app');
		app.events.destroy();
		(app as unknown) = null;
	});
}
