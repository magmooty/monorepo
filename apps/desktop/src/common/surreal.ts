export function isSurrealConnectionError(error: Error): boolean {
	return error.message.includes('Failed to retrieve remote version');
}
