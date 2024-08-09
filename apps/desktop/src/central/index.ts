import Surreal from 'surrealdb.js';
import { CentralAuthController } from './auth';
import { CentralCenterController } from './center';
import { isSurrealConnectionError } from 'common/surreal';

export enum ConnectError {
	DatabaseConnectionError = 'database_connection_error'
}

export class CentralClient {
	apiBaseUrl: string;
	remoteSurrealUrl: string;
	db: Surreal;

	auth: CentralAuthController;
	center: CentralCenterController;

	constructor(private testing = false) {
		if (!import.meta.env.VITE_CENTRAL_API_BASE_URL) {
			throw new Error('VITE_CENTRAL_API_BASE_URL is not set in environment variables');
		}

		this.apiBaseUrl = import.meta.env.VITE_CENTRAL_API_BASE_URL;

		if (!import.meta.env.VITE_REMOTE_SURREAL_URL) {
			throw new Error('VITE_REMOTE_SURREAL_URL is not set in environment variables');
		}

		this.remoteSurrealUrl = import.meta.env.VITE_REMOTE_SURREAL_URL;

		this.db = new Surreal();

		this.auth = new CentralAuthController(this);
		this.center = new CentralCenterController(this);
	}

	async isDatabaseConnected() {
		return this.db
			.ping()
			.then(() => true)
			.catch((error) => {
				if (isSurrealConnectionError(error)) {
					return false;
				}

				throw error;
			});
	}

	/**
	 * @throws {ConnectError} if the function fails
	 */
	async connect(): Promise<void> {
		if (this.testing) {
			this.remoteSurrealUrl = 'http://127.0.0.1:7001/rpc';
		}

		await this.db.connect(this.remoteSurrealUrl).catch((error) => {
			if (isSurrealConnectionError(error)) {
				throw new Error(ConnectError.DatabaseConnectionError);
			}

			throw error;
		});

		await this.db.use({ namespace: 'magmooty', database: 'magmooty' }).catch((error) => {
			if (isSurrealConnectionError(error)) {
				throw new Error(ConnectError.DatabaseConnectionError);
			}

			throw error;
		});
	}
}
