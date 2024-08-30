import { LocalUserScope } from './static-types';

export function generateDatabaseSchema() {
	const CenterManagerScope = `(SELECT * FROM scope WHERE user = $auth.id AND scope_name = '${LocalUserScope.ManageCenter}')`;
	const SpaceManagerScope = `space IN (SELECT space FROM scope WHERE user = $auth.id AND scope_name = '${LocalUserScope.ManageSpace}' GROUP BY space).space`;
	const SpaceMemberScope = `space IN (SELECT space FROM scope WHERE user = $auth.id GROUP BY space).space`;

	const CustomScope = (scope: LocalUserScope) =>
		`space IN (SELECT space FROM scope WHERE user = $auth.id AND scope_name = '${scope}' GROUP BY space)`;

	return `
			DEFINE TABLE user SCHEMAFULL
				PERMISSIONS
					FOR SELECT FULL, // All users can see other users
					FOR UPDATE WHERE id = $auth.id OR ${CenterManagerScope}, // User can update themselves
					FOR CREATE, DELETE WHERE (SELECT * FROM scope WHERE user = $auth.id AND (scope_name = '${LocalUserScope.ManageCenter}' OR scope_name = 'manage_space')); // Center manager can create other users, A space owner can create users
			DEFINE FIELD name ON TABLE user TYPE string;
			DEFINE FIELD phone_number ON TABLE user TYPE string;
			DEFINE FIELD password ON TABLE user TYPE string PERMISSIONS FOR SELECT NONE;

			DEFINE INDEX user_phone_number_index ON TABLE user COLUMNS phone_number UNIQUE;
			
			DEFINE TABLE scope SCHEMAFULL
				PERMISSIONS
					FOR SELECT FULL, // All users can see other users' scopes
					FOR CREATE, UPDATE, DELETE WHERE ${CenterManagerScope} OR (SELECT id FROM scope WHERE user = $auth.id AND scope_name = 'manage_space' AND space = $this.space); // Center manager can modify other users' scopes, A space owner can modify users' scopes if the scopes are for an owned space
			DEFINE FIELD user ON TABLE scope TYPE record<user>;
			DEFINE FIELD space ON TABLE scope TYPE option<record<space>>;
			DEFINE FIELD scope_name ON TABLE scope TYPE string;

			DEFINE SCOPE local_user SESSION 24h
        SIGNIN ( SELECT * FROM user WHERE phone_number = $phone_number AND crypto::argon2::compare(password, $password) );

			DEFINE TABLE space SCHEMAFULL
				PERMISSIONS
					FOR SELECT FULL,
					FOR CREATE, DELETE WHERE ${CenterManagerScope},
					FOR UPDATE WHERE ${CenterManagerScope} OR id IN (SELECT space FROM scope WHERE user = $auth.id AND scope_name = 'manage_space');
			DEFINE FIELD name ON TABLE space TYPE string;

			DEFINE TABLE academic_year SCHEMAFULL
				PERMISSIONS
					FOR SELECT WHERE ${CenterManagerScope} OR ${SpaceMemberScope},
					FOR CREATE, UPDATE, DELETE WHERE ${CenterManagerScope} OR ${SpaceManagerScope} OR ${CustomScope(LocalUserScope.ManageAcademicYears)};
			DEFINE FIELD year ON TABLE academic_year TYPE number;
			DEFINE FIELD space ON TABLE academic_year TYPE record<space>;

			DEFINE TABLE academic_year_course SCHEMAFULL
				PERMISSIONS
					FOR SELECT WHERE ${CenterManagerScope} OR ${SpaceMemberScope},
					FOR CREATE, UPDATE, DELETE WHERE ${CenterManagerScope} OR ${SpaceManagerScope} OR ${CustomScope(LocalUserScope.ManageAcademicYearCourses)};
			DEFINE FIELD grade ON TABLE academic_year_course TYPE string;
			DEFINE FIELD subjects ON TABLE academic_year_course TYPE array<string>;
			DEFINE FIELD academic_year ON TABLE academic_year_course TYPE record<academic_year>;
			DEFINE FIELD space ON TABLE academic_year_course TYPE record<space>;

			DEFINE TABLE group SCHEMAFULL
				PERMISSIONS
					FOR SELECT WHERE ${CenterManagerScope} OR ${SpaceMemberScope},
					FOR CREATE, UPDATE, DELETE WHERE ${CenterManagerScope} OR ${SpaceManagerScope} OR ${CustomScope(LocalUserScope.ManageGroups)};
			DEFINE FIELD schedule ON TABLE group FLEXIBLE TYPE array<object>;
			DEFINE FIELD academic_year ON TABLE group TYPE record<academic_year>;
			DEFINE FIELD course ON TABLE group TYPE record<academic_year_course>;
			DEFINE FIELD space ON TABLE group TYPE record<space>;

			DEFINE ANALYZER name_analyzer TOKENIZERS blank FILTERS edgengram(2,10);

			DEFINE TABLE student SCHEMAFULL
				PERMISSIONS
					FOR SELECT WHERE ${CenterManagerScope} OR ${SpaceMemberScope},
					FOR CREATE, UPDATE, DELETE WHERE ${CenterManagerScope} OR ${SpaceManagerScope} OR ${CustomScope(LocalUserScope.ManageStudents)};
			DEFINE FIELD name ON TABLE student TYPE string;
			DEFINE FIELD _name ON TABLE student TYPE string;
			DEFINE FIELD phone_numbers ON TABLE student FLEXIBLE TYPE array<object>;

			DEFINE INDEX student_name_index ON student FIELDS _name SEARCH ANALYZER name_analyzer BM25;

			DEFINE EVENT student_enrollment_syncer ON TABLE student WHEN $before.name != $after.name THEN (
				UPDATE enrollment SET name = $after.name, _name = $after.name WHERE student = $after.id
			);

			DEFINE TABLE enrollment SCHEMAFULL
				PERMISSIONS
					FOR SELECT WHERE ${CenterManagerScope} OR ${SpaceMemberScope},
					FOR CREATE, UPDATE, DELETE WHERE ${CenterManagerScope} OR ${SpaceManagerScope} OR ${CustomScope(LocalUserScope.ManageStudents)};
			DEFINE FIELD name ON TABLE enrollment TYPE string; 
			DEFINE FIELD _name ON TABLE enrollment TYPE string; 
			DEFINE FIELD student ON TABLE enrollment TYPE record<student>; 
			DEFINE FIELD default_group ON TABLE enrollment TYPE record<group>;
			DEFINE FIELD academic_year ON TABLE enrollment TYPE record<academic_year>;
			DEFINE FIELD course ON TABLE enrollment TYPE record<academic_year_course>;
			DEFINE FIELD space ON TABLE enrollment TYPE record<space>;

			DEFINE INDEX enrollment_student_name_index ON enrollment FIELDS _name SEARCH ANALYZER name_analyzer BM25;
		`;
}
