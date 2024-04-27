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
	event: string;
}
export interface CognitioConfigChangedPayload {
	config: CognitioConfig;
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

export interface CheatsheetInfo {
	title: string;
	path: string;
}

export interface Styling {
	menu?: Record<string, string>;
}

export interface CognitioConfig {
	editor?: string;
	cheatsheets: string[] | CheatsheetInfo[];
	styling?: Styling;
}
