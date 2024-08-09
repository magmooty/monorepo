import { logger } from '$lib/logger';
import type { App } from 'sdk';
import type { Space } from './space';
import type { CreatePayload, LinkedObject } from 'common';
import type { RecordId } from 'surrealdb.js';

export type AcademicYear = {
	id: RecordId<string>;
	year: number;
	space: LinkedObject<Space>;
};

const LOG_TARGET = 'AcademicYearController';

export class AcademicYearController {
	constructor(private app: App) {}

	async createAcademicYear(year: number, space: RecordId<string>): Promise<AcademicYear> {
		logger.info(LOG_TARGET, `Creating academic year ${year} for space ${space}`);

		const [academicYear] = await this.app.db.create<CreatePayload<AcademicYear>>('academic_year', {
			year,
			space
		});

		return academicYear;
	}

	async listAcademicYears(space: LinkedObject<Space>): Promise<AcademicYear[]> {
		logger.info(LOG_TARGET, `Listing academic years for space ${space}`);
		const [academicYears] = await this.app.db.query<AcademicYear[][]>(
			'SELECT year FROM academic_year WHERE space = $space',
			{ space }
		);

		return academicYears;
	}
}
