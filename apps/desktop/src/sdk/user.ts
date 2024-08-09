import type { LinkedObject } from 'common';
import type { Space } from './space';
import { logger } from '$lib/logger';
import type { App } from 'sdk';
import type { RecordId } from 'surrealdb.js';

export enum LocalUserScope {
	ManageCenter = 'manage_center',
	ManageSpace = 'manage_space',
	CreateStudent = 'create_student'
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