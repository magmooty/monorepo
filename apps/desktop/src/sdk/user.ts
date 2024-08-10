import type { LinkedObject } from 'common';
import type { Space } from './space';
import type { RecordId } from 'surrealdb.js';

export enum LocalUserScope {
	ManageCenter = 'manage_center',
	ManageSpace = 'manage_space',
	ManageAcademicYears = 'manage_academic_years',
	ManageAcademicYearCourses = 'manage_academic_year_courses',
	ManageGroups = 'manage_groups',
	ManageStudents = 'manage_students'
}

export type Scope = {
	scope_name: LocalUserScope;
	space?: LinkedObject<Space>;
	user: LinkedObject<LocalUser>;
};

export type LocalUser = {
	id: RecordId<string>;
	name: string;
	phone_number: string;
	password?: string;
};
