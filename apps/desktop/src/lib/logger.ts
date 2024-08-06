export class Logger {
	debug(target: string, message: string) {
		console.debug(`${target}: ${message}`);
	}

	info(target: string, message: string) {
		console.info(`${target}: ${message}`);
	}

	warn(target: string, message: string) {
		console.warn(`${target}: ${message}`);
	}

	error(target: string, message: string) {
		console.error(`${target}: ${message}`);
	}
}

export const logger = new Logger();
