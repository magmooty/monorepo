import { Surreal } from 'surrealdb.js';
import { StudentsController } from './students';

export class App {
	public db: Surreal;
	public students: StudentsController;

	constructor() {
		this.db = new Surreal();
		this.students = new StudentsController(this);
	}

	/**
	 * Connect to the SurrealDB server
	 */
	async connect() {
		await this.db.connect('http://127.0.0.1:8000/rpc');
		await this.db.use({ namespace: 'test', database: 'test' });
	}
}

export const app: App = new App();
