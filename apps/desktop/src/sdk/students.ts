import { App } from 'sdk';
import { Surreal } from 'surrealdb.js';
import * as yup from 'yup';
import { phoneNumberShape } from 'sdk/common';

const studentSchema = yup.object({
	name: yup.string().required(),
	phone_numbers: yup.array(phoneNumberShape).required()
});

export type Student = yup.InferType<typeof studentSchema>;

export class StudentsController {
	db: Surreal;

	constructor(app: App) {
		this.db = app.db;
	}

	async create(student: Student) {
		studentSchema.validateSync(student);
		await this.db.create('student', student);
	}
}
