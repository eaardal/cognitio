import type { Directory, File } from '$lib/models';
import { orderBy } from 'lodash';

export function findDirectoryWithPath(
	directories: Directory[],
	path: string
): Directory | undefined {
	let directory: Directory | undefined;

	for (const dir of directories) {
		if (dir.path === path) {
			directory = dir;
			break;
		}

		if (dir.sub_directories.length > 0) {
			for (const subDirectory of dir.sub_directories) {
				if (subDirectory.path === path) {
					directory = subDirectory;
					break;
				}
			}
		}
	}

	return directory;
}

export function findFileWithPath(directories: Directory[], path: string): File | undefined {
	let file: File | undefined;

	for (const dir of directories) {
		if (dir.files.length > 0) {
			for (const f of dir.files) {
				if (f.path === path) {
					file = f;
					break;
				}
			}
		}

		if (dir.sub_directories.length > 0) {
			for (const subDirectory of dir.sub_directories) {
				if (subDirectory.files.length > 0) {
					for (const f of subDirectory.files) {
						if (f.path === path) {
							file = f;
							break;
						}
					}
				}
			}
		}
	}

	return file;
}

export function orderAllDirectoriesByName(directories: Directory[]): Directory[] {
	const orderedDirectories = orderBy(directories, ['name'], ['asc']);
	for (const dir of orderedDirectories) {
		dir.sub_directories = orderBy(dir.sub_directories, ['name'], ['asc']);
	}
	return orderedDirectories;
}

export function findFirstDirectoryWithCheatsheets(directories: Directory[]): Directory | undefined {
	let directoryToLoad: Directory | undefined;

	for (const directory of directories) {
		if (directory.files.length > 0) {
			directoryToLoad = directory;
			break;
		}

		if (directory.sub_directories.length > 0) {
			directoryToLoad = findFirstDirectoryWithCheatsheets(directory.sub_directories);
			if (directoryToLoad) {
				break;
			}
		}
	}

	return directoryToLoad;
}
