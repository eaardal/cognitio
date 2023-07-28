export type CardKind = 'codeBlock' | 'codeTuple' | 'description';

export type CodeBlockCard = {
	kind: CardKind;
	code: string;
};

export type CodeTupleCard = {
	kind: CardKind;
	code: string;
	description: string;
};

export type DescriptionCard = {
	kind: CardKind;
	text: string;
};

export type Card = CodeBlockCard | CodeTupleCard | DescriptionCard;

export function isCodeBlock(card: Card): card is CodeBlockCard {
	return card.kind === 'codeBlock';
}

export function isCodeTuple(card: Card): card is CodeTupleCard {
	return card.kind === 'codeTuple';
}

export function isDescription(card: Card): card is DescriptionCard {
	return card.kind === 'description';
}

export interface Section {
	title: string;
	cards: Card[];
}

export interface Chapter {
	title: string;
	sections: Section[];
}

export interface Subject {
	title: string;
	chapters: Chapter[];
	sections: Section[];
}
