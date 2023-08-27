export function pathToHash(path: string): string {
	const val = path.replace(/\//g, '_');
	return encodeURIComponent(val);
}

export function hashToPath(hash: string): string {
	const dec = decodeURIComponent(hash);
	return dec.replace(/_/g, '/');
}

export function hashIncludesPath(path: string): boolean {
	return window.location.hash.includes(pathToHash(path));
}
