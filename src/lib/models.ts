export type CardKind = 'codeBlock' | 'codeTuple' | 'description';

export type CodeBlockCardContent = {
	kind: CardKind;
	code: string;
};

export type CodeTupleCardContent = {
	kind: CardKind;
	code: string;
	description: string;
};

export type DescriptionCardContent = {
	kind: CardKind;
	text: string;
};

export type CardContent = CodeBlockCardContent | CodeTupleCardContent | DescriptionCardContent;

export function isCodeBlock(content: CardContent): content is CodeBlockCardContent {
	return content.kind === 'codeBlock';
}

export function isCodeTuple(content: CardContent): content is CodeTupleCardContent {
	return content.kind === 'codeTuple';
}

export function isDescription(content: CardContent): content is DescriptionCardContent {
	return content.kind === 'description';
}

export interface Card {
	contents: CardContent[];
}

export interface Section {
	title: string;
	cards: Card[];
}

export interface Chapter {
	title: string;
	sections: Section[];
}

export interface Cheatsheet {
	title: string;
	chapters: Chapter[];
	sections: Section[];
}

export interface File {
	name: string;
	path: string;
}

export interface Directory {
	name: string;
	path: string;
	files: File[];
}
