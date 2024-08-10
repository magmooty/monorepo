import { describe, it, beforeAll, vi, expect, afterAll } from 'vitest';
import { App } from 'sdk';
import type { RecordId } from 'surrealdb.js';
import { Grade, Subject } from 'sdk/academic-year-course';
import { StudentPhoneNumberUse } from 'sdk/student';

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

	beforeAll(async () => {
		app = new App('http://127.0.0.1:7002/rpc');

		await app.connect();

		await app.manager.initializeCenter({
			center_name: 'Test Center',
			id: '1',
			private_key: 'private',
			public_key: 'public'
		});

		await app.manager.initializeLocalAdmin({
			name: 'Test Admin',
			password: '0000',
			phone_number: '+201096707442'
		});

		await app.auth.signIn('+201096707442', '0000');

		space = (await app.spaces.createSpace('Test Space')).id;

		academicYear = (await app.academicYears.createAcademicYear(2024, space)).id;

		await app.academicYearCourses.createAcademicYearCourse({
			academic_year: academicYear,
			grade: Grade.Secondary1,
			subjects: [Subject.English],
			space
		});
	});

	it('Create student', async () => {
		await app.students.create({
			name: 'زياد طارق محمد الزرقا',
			phone_numbers: [{ number: '+201096707442', use: StudentPhoneNumberUse.Student }]
		});

		await app.students.create({
			name: 'عبدالرحمن ختعن',
			phone_numbers: [{ number: '+201151002051', use: StudentPhoneNumberUse.Student }]
		});

		await app.students.create({
			name: 'مهاب أكرم صبري',
			phone_numbers: [{ number: '+201070671580', use: StudentPhoneNumberUse.Student }]
		});
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
			expect.not.arrayContaining([expect.objectContaining({ name: 'زياد طارق محمد الزرقا' })])
		);
	});
});
