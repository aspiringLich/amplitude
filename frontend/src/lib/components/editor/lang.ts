import { markdown } from '@codemirror/lang-markdown';
import { python } from '@codemirror/lang-python';
import { javascript } from '@codemirror/lang-javascript';
import type { LanguageSupport } from '@codemirror/language';

export type LangType = 'markup' | 'scripting' | 'compiled';
export type LangInfo = {
	lang: LanguageSupport;
	type: LangType;
};
export const langs: { [key: string]: LangInfo } = {
	markdown: {
		lang: markdown(),
		type: 'markup',
	},
	python: {
		lang: python(),
		type: 'scripting',
	},
	javascript: {
		lang: javascript(),
		type: 'scripting',
	},
};
export const names: ['markdown'] = ['markdown'];
