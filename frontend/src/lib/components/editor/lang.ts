import { markdown } from '@codemirror/lang-markdown';
import { python } from '@codemirror/lang-python';
import { javascript } from '@codemirror/lang-javascript';
import { yaml } from '@codemirror/lang-yaml';
import type { LanguageSupport } from '@codemirror/language';

export type LangType = 'markup' | 'scripting' | 'compiled' | 'config';
export type CodeFnReturn = { code: string; cursor: [number, number] | number };
export type CodeFn = (fns: { [key: string]: string[] }) => CodeFnReturn;
export type LangInfo = {
	lang: LanguageSupport;
	type: LangType;
	code?: CodeFn;
	// return [code, [cursorStart, cursorEnd]]
};

export const langs: { [key: string]: LangInfo } = {
	markdown: {
		lang: markdown(),
		type: 'markup'
	},
	python: {
		lang: python(),
		type: 'scripting',
		code: (fns) => {
			const entries = Object.entries(fns);
			const code = entries
				.map(([name, args]) => `def ${name}(${args.join(', ')}):\n\tpass\n`)
				.join('\n');
			const [name, args] = entries[0];
			const pos = 4 + name.length + 1 + args.join(', ').length + 4;
			return {
				code: `\n${code}`,
				cursor: [pos + 1, pos + 5]
			};
		}
	},
	javascript: {
		lang: javascript(),
		type: 'scripting',
		code: (fns) => {
			const entries = Object.entries(fns);
			const code = entries
				.map(([name, args]) => `export function ${name}(${args.join(', ')}) {\n\t\n}\n`)
				.join('\n');
			const [name, args] = entries[0];
			const pos = 16 + name.length + 1 + args.join(', ').length + 5;
			return {
				code: `\n${code}`,
				cursor: pos + 1
			};
		}
	},
	yaml: {
		lang: yaml(),
		type: 'config'
	}
};

export const names: ['markdown', 'python', 'javascript', 'yaml'] = [
	'markdown',
	'python',
	'javascript',
	'yaml'
];
