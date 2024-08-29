import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';

export default defineConfig({
	plugins: [sveltekit()],
	test: {
		minWorkers: 1,
		maxWorkers: 1,
		maxConcurrency: 1,
		setupFiles: ['./vitest.setup.ts'],
		include: ['src/**/*.{test,spec}.{js,ts}']
	}
});

import fs from 'fs';
import path from 'path';
import { generateDatabaseSchema } from './src/sdk/schema';

const schema = `pub static LOCAL_SCHEMA: &str = "${generateDatabaseSchema()}";`;

fs.writeFileSync(path.join(__dirname, '../api/src/database/schema.rs'), schema);
