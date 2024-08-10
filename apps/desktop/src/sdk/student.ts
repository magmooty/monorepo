import { App } from 'sdk';
import { RecordId, Surreal } from 'surrealdb.js';
import { nameFilter } from './common';
import type { CreatePayload } from 'common';

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

export type Student = {
	id: RecordId<string>;
	name: string;
	/**
	 * Normalized name for search
	 */
	_name: string;
	phone_numbers: StudentPhoneNumber[];
};

export type StudentCreatePayload = Omit<CreatePayload<Student>, '_name'>;

export class StudentsController {
	db: Surreal;

	constructor(app: App) {
		this.db = app.rootDb;
	}

	async create(content: StudentCreatePayload): Promise<Student> {
		const [student] = await this.db.create<CreatePayload<Student>>('student', {
			...content,
			name: content.name,
			_name: nameFilter(content.name, true)
		});

		return student;
	}

	async get(id: RecordId<string>): Promise<Student> {
		const student = await this.db.select<Student>(id);

		return student;
	}

	async list(offset = 0, limit = 10): Promise<Student[]> {
		const [students] = await this.db.query<Student[][]>(
			'SELECT * FROM student LIMIT $limit START $offset',
			{ offset, limit }
		);

		return students;
	}

	async search(query: string, limit = 10): Promise<Student[]> {
		const [students] = await this.db.query<Student[][]>(
			'SELECT * FROM student WHERE _name @@ $query LIMIT $limit',
			{ query: nameFilter(query, true), limit }
		);

		return students;
	}

	async rename(id: RecordId<string>, name: string): Promise<Student> {
		const [[student]] = await this.db.query<Student[][]>(
			'UPDATE student SET name = $name, _name = $_name WHERE id = $id',
			{
				name,
				_name: nameFilter(name, true),
				id
			}
		);

		return student as Student;
	}
}
