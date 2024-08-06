import Surreal from 'surrealdb.js';

export class CentralClient {
	apiBaseUrl: string;
	remoteSurrealUrl: string;

	db: Surreal;

	constructor() {
		if (!import.meta.env.VITE_CENTRAL_API_BASE_URL) {
			throw new Error('VITE_CENTRAL_API_BASE_URL is not set in environment variables');
		}

		this.apiBaseUrl = import.meta.env.VITE_CENTRAL_API_BASE_URL;

		if (!import.meta.env.VITE_REMOTE_SURREAL_URL) {
			throw new Error('VITE_REMOTE_SURREAL_URL is not set in environment variables');
		}

		this.remoteSurrealUrl = import.meta.env.VITE_REMOTE_SURREAL_URL;

		this.db = new Surreal();
	}

	async isDatabaseConnected() {
		return this.db.ping();
	}

	async connect(): Promise<void> {
		await this.db.connect(this.remoteSurrealUrl);
		await this.db.use({ namespace: 'magmooty', database: 'magmooty' });
	}
}
