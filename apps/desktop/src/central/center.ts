import type { CentralClient } from 'central';
import type Surreal from 'surrealdb.js';
import type { Address, LinkedObject } from '../common';
import type { RemoteUser } from './auth';
import { generateKeyPair } from '$lib/bindings';
import { logger } from '$lib/logger';
import type { InitializeCenterParameters } from 'sdk/manager';

export interface Center {
	name: string;
	address: Address;
	public_key: string;
	owner: LinkedObject<RemoteUser>;
}

const LOG_TARGET = 'CentralCenterController';

export class CentralCenterController {
	db: Surreal;

	constructor(private client: CentralClient) {
		this.db = client.db;
	}

	async list(): Promise<Center[]> {
		const userId = this.client.auth.userId();

		logger.info(LOG_TARGET, `Listing centers for user ${userId}`);
		return await this.db.query('SELECT * FROM center WHERE owner = $owner', {
			owner: userId
		});
	}

	async create(payload: Omit<Center, 'owner' | 'public_key'>): Promise<InitializeCenterParameters> {
		const userId = this.client.auth.userId();

		logger.info(LOG_TARGET, `Creating a new center for user ${userId}`);

		logger.info(LOG_TARGET, `Generating a new key pair`);
		const { public_key, private_key } = await generateKeyPair();

		logger.info(LOG_TARGET, `Inserting new center into the database`);
		const [record] = await this.db.create('center', {
			...payload,
			owner: this.client.auth.userId(),
			public_key
		});

		logger.info(LOG_TARGET, `Center created ${record}`);
		return { id: record.id, center_name: payload.name, public_key, private_key };
	}
}
