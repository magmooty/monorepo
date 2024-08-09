import { logger } from '$lib/logger';
import type { App } from 'sdk';
import type { RecordId } from 'surrealdb.js';

export type Space = {
	name: string;
};

const LOG_TARGET = 'SpaceController';

export class SpaceController {
	constructor(private app: App) {}

	async listSpaces(): Promise<Space[]> {
		logger.info(LOG_TARGET, `Listing spaces`);
		return await this.app.db.query<Space[]>('SELECT name FROM space');
	}

	async createSpace(name: string): Promise<void> {
		logger.info(LOG_TARGET, `Creating space ${name}`);
		await this.app.db.create<Space>('space', { name });
	}

	async renameSpace(id: RecordId<string>, name: string): Promise<void> {
		logger.info(LOG_TARGET, `Creating space ${name}`);
		await this.app.db.update(id, { name });
	}
}
