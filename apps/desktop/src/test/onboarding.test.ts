import { describe, it, beforeAll, vi, expect } from 'vitest';
import { App } from 'sdk';
import type { RecordId } from 'surrealdb.js';
import { Grade, Subject } from 'sdk/academic-year-course';

beforeAll(() => {
	vi.mock('$lib/bindings', () => {
		return {
			setGlobalKey: () => {},
			getRootDatabaseCredentials: () => ({ username: 'root', password: 'root' })
		};
	});
});

describe('SDK', () => {
	let app: App;
	let space: RecordId<string>;
	let academicYear: RecordId<string>;

	beforeAll(() => {
		app = new App('http://127.0.0.1:7002/rpc');
	});

	it('Initialize and connect to database', async () => {
		await app.connect();
	});

	it('Set up the database', async () => {
		await app.manager.initializeCenter({
			center_name: 'Test Center',
			id: '1',
			private_key: 'private',
			public_key: 'public'
		});
	});

	it('Create an admin user', async () => {
		await app.manager.initializeLocalAdmin({
			name: 'Test Admin',
			password: '0000',
			phone_number: '+201096707442'
		});
	});

	it('List all users', async () => {
		const users = await app.auth.listUsers();

		expect(users).toHaveLength(1);

		const [adminUser] = users;

		expect(adminUser.name).toBe('Test Admin');
		expect(adminUser.is_center_manager).toBeTruthy();
		expect(adminUser.phone_number).toBe('+201096707442');
		expect(adminUser.manages_spaces).toHaveLength(0);
		expect(adminUser.member_of_spaces).toHaveLength(0);
	});

	it('Sign in as admin user', async () => {
		await app.auth.signIn('+201096707442', '0000');
	});

	it('Create a space', async () => {
		space = (await app.spaces.createSpace('Test Space')).id;
	});

	it('List all spaces', async () => {
		const spaces = await app.spaces.listSpaces();

		expect(spaces).toHaveLength(1);

		const [testSpace] = spaces;

		expect(testSpace.name).toBe('Test Space');
	});

	it('Create an academic year', async () => {
		academicYear = (await app.academicYears.createAcademicYear(2024, space)).id;
	});

	it('List all academic years', async () => {
		const academicYears = await app.academicYears.listAcademicYears(space);

		expect(academicYears).toHaveLength(1);

		const [testAcademicYear] = academicYears;

		expect(testAcademicYear.year).toBe(2024);
	});

	it('Create an academic year course', async () => {
		await app.academicYearCourses.createAcademicYearCourse({
			academic_year: academicYear,
			grade: Grade.Secondary1,
			subjects: [Subject.English],
			space
		});
	});

	it('List all academic year courses', async () => {
		const academicYearCourses = await app.academicYearCourses.listAcademicYearCourses(academicYear);

		expect(academicYearCourses).toHaveLength(1);

		const [testAcademicYearCourse] = academicYearCourses;

		expect(testAcademicYearCourse.grade).toBe(Grade.Secondary1);
		expect(testAcademicYearCourse.subjects).toEqual([Subject.English]);
	});

	it('Clear local database', async () => {
		await app.manager.clearLocalDatabase();

		const users = await app.auth.listUsers();

		expect(users).toHaveLength(0);
	});
});
