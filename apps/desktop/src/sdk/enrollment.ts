import type { CreatePayload, LinkedObject } from 'common';
import { App } from 'sdk';
import { RecordId, Surreal } from 'surrealdb.js';
import type { Student } from './student';
import type { Space } from './space';
import type { AcademicYear } from './academic-year';
import type { Group } from './group';
import { nameFilter } from './common';
import type { AcademicYearCourse } from './academic-year-course';

export enum StudentPhoneNumberUse {
	Parent = 'parent',
	Student = 'student',
	Home = 'home',
	Other = 'other'
}

export interface StudentPhoneNumber {
	number: string;
	use: StudentPhoneNumberUse;
}

export type Enrollment = {
	id: RecordId<string>;
	name: string;
	_name: string;
	student: LinkedObject<Student>;
	default_group: LinkedObject<Group>;
	academic_year: LinkedObject<AcademicYear>;
	course: LinkedObject<AcademicYearCourse>;
	space: LinkedObject<Space>;
};

export type EnrollmentCreatePayload = Omit<CreatePayload<Enrollment>, 'name' | '_name'>;

export class EnrollmentsController {
	db: Surreal;

	constructor(private app: App) {
		this.db = app.rootDb;
	}

	async create(content: EnrollmentCreatePayload): Promise<Enrollment> {
		const student = await this.app.students.get(content.student as RecordId<string>);

		const [enrollment] = await this.db.create<CreatePayload<Enrollment>>('enrollment', {
			...content,
			name: student.name,
			_name: student._name
		});

		return enrollment;
	}

	async list(course: RecordId<string>, offset = 0, limit = 10): Promise<Enrollment[]> {
		const [enrollments] = await this.db.query<Enrollment[][]>(
			'SELECT * FROM enrollment WHERE course = $course LIMIT $limit START $offset',
			{ course, offset, limit }
		);

		return enrollments;
	}

	async search(query: string, limit = 10): Promise<Enrollment[]> {
		const [enrollments] = await this.db.query<Enrollment[][]>(
			'SELECT * FROM enrollment WHERE _name @@ $query LIMIT $limit FETCH student',
			{ query: nameFilter(query), limit }
		);

		return enrollments;
	}
}
