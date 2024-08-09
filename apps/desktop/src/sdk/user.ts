import type { LinkedObject } from 'common';
import type { Space } from './space';

export enum LocalUserScope {
	ManageCenter = 'manage_center',
	ManageSpace = 'manage_space',
	ManageAcademicYears = 'manage_academic_years',
	ManageAcademicYearCourses = 'manage_academic_year_courses',
	ManageGroups = 'manage_groups',
}

export type Scope = {
	scope_name: LocalUserScope;
	space?: LinkedObject<Space>;
	user: LinkedObject<LocalUser>;
};

export type LocalUser = {
	name: string;
	phone_number: string;
	password?: string;
};
