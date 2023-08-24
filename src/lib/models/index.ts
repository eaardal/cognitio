export interface Cheatsheet {
	title: string;
	markdown: string;
}

export interface File {
	name: string;
	path: string;
}

export interface Directory {
	name: string;
	path: string;
	files: File[];
	sub_directories: Directory[];
}

export interface FileChangedPayload {
	path: string;
}

export interface MenuItem {
	id: string;
	title: string;
	children: MenuItem[];
}

export interface MenuSection {
	title: string;
	items: MenuItem[];
	tooltipText: string | undefined;
}
