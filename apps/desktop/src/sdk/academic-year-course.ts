import { logger } from '$lib/logger';
import type { CreatePayload, LinkedObject } from 'common';
import type { App } from 'sdk';
import type { AcademicYear } from './academic-year';
import type { RecordId } from 'surrealdb.js';

export type Space = {
	name: string;
};

export enum Grade {
	Elementary1 = 'E1',
	Elementary2 = 'E2',
	Elementary3 = 'E3',
	Elementary4 = 'E4',
	Elementary5 = 'E5',
	Elementary6 = 'E6',
	Secondary1 = 'S1',
	Secondary2 = 'S2',
	Secondary3 = 'S3',
	HighSchool1 = 'HS1',
	HighSchool2 = 'HS2',
	HighSchool3 = 'HS3'
}

export enum Subject {
	// Elementary
	Arabic = 'arabic',
	Religion = 'religion',
	Math = 'math',
	IT = 'it',
	SocialStudies = 'social_studies',
	English = 'english',
	German = 'german',
	Spanish = 'spanish',
	French = 'french',
	Science = 'science',

	// Secondary
	Chinese = 'chinese',
	Italian = 'italian',

	// High School
	Economics = 'economics',
	Statistics = 'statistics',
	Biology = 'biology',
	Statics = 'statics',
	History = 'history',
	Calculus = 'calculus',
	Integral = 'integral',
	Algebra = 'algebra',
	PureMathematics = 'pure_mathematics',
	SolidGeometry = 'solid_geometry',
	Geography = 'geography',
	Geology = 'geology',
	Dynamics = 'dynamics',
	Philosophy = 'philosophy',
	Physics = 'physics',
	Chemistry = 'chemistry',
	Mechanics = 'mechanics',
	Geometry = 'geometry',
	Trigonometry = 'trigonometry',
	Sociology = 'sociology',
	Psychology = 'psychology'
}

const LOG_TARGET = 'AcademicYearCourseController';

export type AcademicYearCourse = {
	id: RecordId<string>;
	grade: Grade;
	subjects: Subject[];
	academic_year: LinkedObject<AcademicYear>;
	space: LinkedObject<Space>;
};

export class AcademicYearCourseController {
	constructor(private app: App) {}

	async create(
		content: CreatePayload<AcademicYearCourse>
	): Promise<AcademicYearCourse> {
		logger.info(LOG_TARGET, `Creating academic year course`);
		const [academicYearCourse] = await this.app.db.create<CreatePayload<AcademicYearCourse>>(
			'academic_year_course',
			content
		);

		return academicYearCourse;
	}

	async list(academicYear: LinkedObject<AcademicYear>) {
		logger.info(LOG_TARGET, `Listing academic year courses for academic year ${academicYear}`);
		const [academicYearCourses] = await this.app.db.query<AcademicYearCourse[][]>(
			'SELECT * FROM academic_year_course WHERE academic_year = $academicYear',
			{
				academicYear
			}
		);

		return academicYearCourses;
	}
}
