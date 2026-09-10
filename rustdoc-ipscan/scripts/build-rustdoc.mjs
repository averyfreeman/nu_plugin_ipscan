import { cpSync, mkdirSync, rmSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';

const siteDirectory = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const repositoryDirectory = resolve(siteDirectory, '..');
const manifestPath = resolve(repositoryDirectory, 'Cargo.toml');
const generatedDirectory = resolve(repositoryDirectory, 'target', 'doc');
const publicDirectory = resolve(siteDirectory, 'public', 'rustdoc');

const result = spawnSync(
	process.platform === 'win32' ? 'cargo.exe' : 'cargo',
	['doc', '--manifest-path', manifestPath, '--workspace', '--no-deps', '--document-private-items', '--locked'],
	{ cwd: repositoryDirectory, stdio: 'inherit' },
);

if (result.error) {
	throw result.error;
}

if (result.status !== 0) {
	process.exit(result.status ?? 1);
}

rmSync(publicDirectory, { recursive: true, force: true });
mkdirSync(resolve(siteDirectory, 'public'), { recursive: true });
mkdirSync(publicDirectory, { recursive: true });
for (const crate of ['ipscan', 'ipscan_lib', 'nu_plugin_ipscan']) {
	const source = resolve(generatedDirectory, crate);
	const destination = resolve(publicDirectory, crate);
	cpSync(source, destination, { recursive: true });
}
