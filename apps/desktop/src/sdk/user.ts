import type { LinkedObject } from 'common';

export enum LocalUserScope {
	ManageCenter = 'manage_center',
	CreateStudent = 'create_student'
}

export type Scope = {
	scope_name: LocalUserScope;
	space?: LinkedObject;
	user: LinkedObject;
};

export type LocalUser = {
	name: string;
	phone_number: string;
	password: string;
};
