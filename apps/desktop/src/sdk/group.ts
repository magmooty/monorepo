import { logger } from '$lib/logger';
import type { App } from 'sdk';
import type { Space } from './space';
import type { CreatePayload, LinkedObject } from 'common';
import type { AcademicYear } from './academic-year';
import type { RecordId } from 'surrealdb.js';

export enum Day {
	Saturday = 'Saturday',
	Sunday = 'Sunday',
	Monday = 'Monday',
	Tuesday = 'Tuesday',
	Wednesday = 'Wednesday',
	Thursday = 'Thursday',
	Friday = 'Friday'
}

export interface ClassSchedule {
	/**
	 * The order of the class in the group, first class, second, third, etc. This can be used to link classes of different groups together.
	 */
	class_order: number;
	/**
	 * The day of the week the class is scheduled
	 */
	day: Day;
	/**
	 * Defined in minutes from the start of the day
	 */
	start: number;
	/**
	 * Defined in minutes from the start of the day
	 */
	end: number;
}

export type Group = {
	id: RecordId<string>;
	schedule: ClassSchedule[];
	academic_year: LinkedObject<AcademicYear>;
	space: LinkedObject<Space>;
};

const LOG_TARGET = 'GroupController';

export class GroupController {
	constructor(private app: App) {}

	async create(content: CreatePayload<Group>): Promise<Group> {
		logger.info(
			LOG_TARGET,
			`Creating group for academic year ${content.academic_year} for space ${content.space}`
		);
		const [group] = await this.app.db.create<CreatePayload<Group>>('group', content);

		return group;
	}

	async list(
		academic_year: LinkedObject<AcademicYear>,
		space: LinkedObject<Space>
	): Promise<Group[]> {
		logger.info(LOG_TARGET, `Listing groups for academic year ${academic_year} for space ${space}`);
		return await this.app.db.query<Group[]>(
			'SELECT * FROM group WHERE academic_year = $academic_year AND space = $space',
			{
				academic_year,
				space
			}
		);
	}

	async update(id: LinkedObject<Space>, content: Group) {
		logger.info(LOG_TARGET, `Updating group ${id}`);
		await this.app.db.update<Group>(id as RecordId<string>, content);
	}
}
