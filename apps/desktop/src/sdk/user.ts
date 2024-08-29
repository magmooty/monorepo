import type { CreatePayload, LinkedObject } from 'common';
import type { Space } from './space';
import type { RecordId } from 'surrealdb.js';
import type { App } from 'sdk';
import type { LocalUserScope } from './static-types';

export type Scope = {
	scope_name: LocalUserScope;
	space?: LinkedObject<Space>;
	user: LinkedObject<LocalUser>;
};

export type LocalUser = {
	id: RecordId<string>;
	name: string;
	phone_number: string;
	password?: string;
};

class ScopeController {
	constructor(private app: App) {}

	async create(scope: CreatePayload<Scope>): Promise<Scope> {
		const [createdScope] = await this.app.db.create<CreatePayload<Scope>>('scope', scope);

		return createdScope;
	}

	async list(user: RecordId<string>): Promise<Scope[]> {
		const [scopes] = await this.app.db.query<Scope[][]>(`SELECT * FROM scope WHERE user = $user`, {
			user
		});

		return scopes;
	}
}

export class UserController {
	public scopes: ScopeController;

	constructor(private app: App) {
		this.scopes = new ScopeController(app);
	}

	async create(user: CreatePayload<LocalUser>): Promise<LocalUser> {
		const [[createdUser]] = await this.app.rootDb.query<LocalUser[][]>(
			`CREATE user CONTENT { name: $name, phone_number: $phone_number, password: crypto::argon2::generate($password) }`,
			user
		);

		return createdUser;
	}
}
