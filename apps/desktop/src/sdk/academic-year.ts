import { logger } from '$lib/logger';
import type { App } from 'sdk';
import type { Space } from './space';
import type { LinkedObject } from 'common';

export type AcademicYear = {
	year: number;
	space: LinkedObject<Space>;
};

const LOG_TARGET = 'AcademicYearController';

export class AcademicYearController {
	constructor(private app: App) {}

	async createAcademicYear(year: number, space: Space) {
		logger.info(LOG_TARGET, `Creating academic year ${year} for space ${space}`);
		await this.app.db.create<AcademicYear>('academic_year', { year, space });
	}

	async listAcademicYears(space: LinkedObject<Space>): Promise<AcademicYear[]> {
		logger.info(LOG_TARGET, `Listing academic years for space ${space}`);
		return await this.app.db.query<AcademicYear[]>(
			'SELECT year FROM academic_year WHERE space = $space',
			{ space }
		);
	}
}
