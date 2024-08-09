import { logger } from '$lib/logger';
import type { CreatePayload } from 'common';
import type { App } from 'sdk';
import type { RecordId } from 'surrealdb.js';

export type Space = {
	id: RecordId<string>;
	name: string;
};

const LOG_TARGET = 'SpaceController';

export class SpaceController {
	constructor(private app: App) {}

	async listSpaces(): Promise<Space[]> {
		logger.info(LOG_TARGET, `Listing spaces`);
		return await this.app.db.select<Space>('space');
	}

	async createSpace(name: string): Promise<Space> {
		logger.info(LOG_TARGET, `Creating space ${name}`);

		const [space] = await this.app.db.create<CreatePayload<Space>>('space', { name });

		return space;
	}

	async renameSpace(id: RecordId<string>, name: string): Promise<void> {
		logger.info(LOG_TARGET, `Creating space ${name}`);
		await this.app.db.update(id, { name });
	}
}
