import { describe, it, beforeAll, vi, expect } from 'vitest';
import { App } from 'sdk';
import { RecordId } from 'surrealdb.js';
import { Grade, Subject } from 'sdk/academic-year-course';
import { StudentPhoneNumberUse } from 'sdk/student';
import { Day } from 'sdk/group';

beforeAll(() => {
	vi.mock('$lib/bindings', () => {
		return {
			setGlobalKey: () => {},
			getRootDatabaseCredentials: () => ({ username: 'root', password: 'root' })
		};
	});
});

describe('Students', () => {
	let app: App;
	let space: RecordId<string>;
	let academicYear: RecordId<string>;
	let academicYearCourse: RecordId<string>;
	let group: RecordId<string>;

	const students: RecordId<string>[] = [];

	beforeAll(async () => {
		app = new App('http://127.0.0.1:7002/rpc');

		await app.connect();

		await app.manager.initializeCenter({
			center_name: 'Test Center',
			id: new RecordId('center', 'test-center'),
			private_key: 'private',
			public_key: 'public'
		});

		await app.manager.initializeLocalAdmin({
			name: 'Test Admin',
			password: '0000',
			phone_number: '+201096707442'
		});

		await app.auth.signIn('+201096707442', '0000');

		space = (await app.spaces.create('Test Space')).id;

		academicYear = (await app.academicYears.create(2024, space)).id;

		academicYearCourse = (
			await app.academicYearCourses.create({
				academic_year: academicYear,
				grade: Grade.Secondary1,
				subjects: [Subject.English],
				space
			})
		).id;
	});

	it('Create student', async () => {
		const ziad = await app.students.create({
			name: 'زياد طارق محمد الزرقا',
			phone_numbers: [{ number: '+201096707442', use: StudentPhoneNumberUse.Student }]
		});

		const khatan = await app.students.create({
			name: 'عبدالرحمن ختعن',
			phone_numbers: [{ number: '+201151002051', use: StudentPhoneNumberUse.Student }]
		});

		const mohab = await app.students.create({
			name: 'مهاب أكرم صبري',
			phone_numbers: [{ number: '+201070671580', use: StudentPhoneNumberUse.Student }]
		});

		students.push(ziad.id, khatan.id, mohab.id);
	});

	it('Get student', async () => {
		const student = await app.students.get(students[0]);

		expect(student).toEqual(expect.objectContaining({ name: 'زياد طارق محمد الزرقا' }));
	});

	it('List students', async () => {
		const students = await app.students.list();

		expect(students).toHaveLength(3);
		expect(students).toEqual(
			expect.arrayContaining([expect.objectContaining({ name: 'زياد طارق محمد الزرقا' })])
		);
	});

	it('Normalize student names', async () => {
		const students = await app.students.list();

		expect(students).toHaveLength(3);
		expect(students).toEqual(
			expect.arrayContaining([
				expect.objectContaining({ name: 'مهاب أكرم صبري' }),
				expect.objectContaining({ name: 'عبدالرحمن ختعن' })
			])
		);
	});

	it('Search students by first name', async () => {
		const students = await app.students.search('مهاب');

		expect(students).toEqual(
			expect.arrayContaining([expect.objectContaining({ name: 'مهاب أكرم صبري' })])
		);
	});

	it('Search students by normalized names', async () => {
		let students = await app.students.search('عبد');

		expect(students).toEqual(
			expect.arrayContaining([expect.objectContaining({ name: 'عبدالرحمن ختعن' })])
		);

		students = await app.students.search('عبد الرحمن');

		expect(students).toEqual(
			expect.arrayContaining([expect.objectContaining({ name: 'عبدالرحمن ختعن' })])
		);

		students = await app.students.search('عبدال');

		expect(students).toEqual(
			expect.arrayContaining([expect.objectContaining({ name: 'عبدالرحمن ختعن' })])
		);
	});

	it('Search students by last name', async () => {
		const students = await app.students.search('صبرى');

		expect(students).toEqual(
			expect.arrayContaining([expect.objectContaining({ name: 'مهاب أكرم صبري' })])
		);
	});

	it('Search students normalizes query', async () => {
		const students = await app.students.search('صبري');

		expect(students).toEqual(
			expect.arrayContaining([expect.objectContaining({ name: 'مهاب أكرم صبري' })])
		);
	});

	it('Search students autocomplete', async () => {
		const students = await app.students.search('زيا');

		expect(students).toEqual(
			expect.arrayContaining([expect.objectContaining({ name: 'زياد طارق محمد الزرقا' })])
		);
	});

	it('Search students unsupported edge cases for name filtering', async () => {
		let students = await app.students.search('عبدا');

		expect(students).toEqual(
			expect.not.arrayContaining([expect.objectContaining({ name: 'عبدالرحمن ختعن' })])
		);

		students = await app.students.search('ز');

		expect(students).toEqual(
			expect.not.arrayContaining([
				expect.objectContaining({
					name: 'زياد طارق محمد الزرقا',
					phone_numbers: expect.arrayContaining([
						expect.objectContaining({ number: '+201096707442', use: StudentPhoneNumberUse.Student })
					])
				})
			])
		);
	});

	it('Rename student', async () => {
		const student = await app.students.rename(students[0], 'زياد طارق');

		expect(student).toEqual(
			expect.objectContaining({
				name: 'زياد طارق',
				phone_numbers: expect.arrayContaining([
					expect.objectContaining({ number: '+201096707442', use: StudentPhoneNumberUse.Student })
				])
			})
		);

		await app.students.rename(students[0], 'زياد طارق محمد الزرقا');
	});

	it('Create group', async () => {
		group = (
			await app.groups.create({
				academic_year: academicYear,
				course: academicYearCourse,
				schedule: [
					{ class_order: 1, day: Day.Saturday, start: 480, end: 540 },
					{ class_order: 2, day: Day.Tuesday, start: 480, end: 540 }
				],
				space
			})
		).id;
	});

	it('Create enrollment', async () => {
		await app.enrollments.create({
			space,
			course: academicYearCourse,
			academic_year: academicYear,
			student: students[0],
			default_group: group
		});
	});

	it('List enrollments', async () => {
		const enrollments = await app.enrollments.list(academicYearCourse);

		expect(enrollments).toHaveLength(1);
		expect(enrollments).toEqual(
			expect.arrayContaining([expect.objectContaining({ student: students[0] })])
		);
	});

	it('Enrollment name gets copied from student', async () => {
		const enrollments = await app.enrollments.list(academicYearCourse);

		expect(enrollments).toHaveLength(1);
		expect(enrollments).toEqual(
			expect.arrayContaining([expect.objectContaining({ name: 'زياد طارق محمد الزرقا' })])
		);
	});

	it('Enrollment name auto updates from student', async () => {
		await app.students.rename(students[0], 'زياد طارق');

		const enrollments = await app.enrollments.list(academicYearCourse);

		expect(enrollments).toHaveLength(1);
		expect(enrollments).toEqual(
			expect.arrayContaining([expect.objectContaining({ name: 'زياد طارق' })])
		);
	});

	it('Search enrollments', async () => {
		const enrollments = await app.enrollments.search('زياد');

		expect(enrollments).toEqual(
			expect.arrayContaining([expect.objectContaining({ name: 'زياد طارق' })])
		);
	});
});
