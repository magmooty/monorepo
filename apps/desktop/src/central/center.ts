import type { CentralClient } from 'central';
import type Surreal from 'surrealdb.js';
import type { Address, LinkedObject } from '../common';
import type { User } from './auth';
import { generateKeyPair, setGlobalKey } from '$lib/bindings';
import { logger } from '$lib/logger';

export interface Center {
	name: string;
	address: Address;
	public_key: string;
	owner: LinkedObject<User>;
}

const LOG_TARGET = 'CentralCenterController';

export class CentralCenterController {
	db: Surreal;

	constructor(private client: CentralClient) {
		this.db = client.db;
	}

	async listCenters(): Promise<Center[]> {
		const userId = this.client.auth.userId();

		logger.info(LOG_TARGET, `Listing centers for user ${userId}`);
		return await this.db.query('SELECT * FROM center WHERE owner = $owner', {
			owner: userId
		});
	}

	async createCenter(center: Omit<Center, 'owner' | 'public_key'>): Promise<void> {
		const userId = this.client.auth.userId();

		logger.info(LOG_TARGET, `Creating a new center for user ${userId}`);

		logger.info(LOG_TARGET, `Generating a new key pair`);
		const { public_key, private_key } = await generateKeyPair();

		logger.info(LOG_TARGET, `Inserting new center into the database`);
		await this.db.create('center', {
			...center,
			owner: this.client.auth.userId(),
			public_key: public_key
		});

		logger.info(LOG_TARGET, `Setting local global key: private_key`);
		await setGlobalKey('private_key', private_key);

		logger.info(LOG_TARGET, `Center created`);
	}
}
