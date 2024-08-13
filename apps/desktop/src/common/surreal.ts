import { RecordId } from 'surrealdb.js';

export function isSurrealConnectionError(error: Error): boolean {
	return error.message.includes('Failed to retrieve remote version');
}

export function parseRecordId(id: string): RecordId<string> {
	try {
		const [table, record] = id.split(':');
		return new RecordId(table, record);
	} catch {
		throw new Error(`Failed to parse record id ${id}`);
	}
}
