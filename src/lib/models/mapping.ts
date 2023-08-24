import type { File, Directory, MenuItem, MenuSection } from '.';

export function mapFilesToMenuItems(files: File[]): MenuItem[] {
	return files.map((file) => {
		return {
			id: file.path,
			title: file.name.replace('.md', ''),
			tooltipText: file.path,
			children: []
		};
	});
}

export function mapDirectoriesToMenuSections(directories: Directory[]): MenuSection[] {
	return directories.map((directory) => {
		const menuSection: MenuSection = {
			title: directory.name,
			tooltipText: directory.path,
			items: []
		};

		if (directory.files.length > 0) {
			menuSection.items = mapFilesToMenuItems(directory.files);
		}

		if (directory.sub_directories.length > 0) {
			menuSection.items = menuSection.items.concat(
				directory.sub_directories.map((subDirectory) => {
					const subDirectoryMenuItem: MenuItem = {
						id: subDirectory.path,
						title: subDirectory.name,
						children: []
					};

					if (subDirectory.files.length > 0) {
						subDirectoryMenuItem.children = mapFilesToMenuItems(subDirectory.files);
					}

					return subDirectoryMenuItem;
				})
			);
		}

		return menuSection;
	});
}
