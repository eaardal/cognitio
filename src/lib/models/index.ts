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
