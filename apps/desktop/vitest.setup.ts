import { afterAll, beforeAll } from 'vitest';
import { spawn, ChildProcess, ChildProcessWithoutNullStreams } from 'child_process';

function runSurrealInstance(port: number): Promise<ChildProcessWithoutNullStreams> {
	return new Promise((resolve, reject) => {
		const process = spawn('env', [
			'surreal',
			'start',
			'memory',
			'--username',
			'root',
			'--pass',
			'root',
			'--bind',
			`127.0.0.1:${port}`
		]);

		const handleOutput = (data: Buffer) => {
			if (data.toString().includes('Started web server')) {
				resolve(process);
			}

			if (data.toString().includes('ERROR')) {
				process.kill();
				reject();
			}
		};

		process.stdout.on('data', handleOutput);
		process.stderr.on('data', handleOutput);
	});
}

const instances: ChildProcessWithoutNullStreams[] = [];

beforeAll(async () => {
	if (process.env.SKIP_SURREAL) {
		return;
	}
	instances.push(await runSurrealInstance(7002));
});

afterAll(() => {
	instances.forEach((instance: ChildProcess) => {
		instance.kill();
	});
});
