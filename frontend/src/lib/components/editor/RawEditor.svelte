<!-- taken from https://github.com/touchifyapp/svelte-codemirror-editor -->

<script lang="ts" context="module">
	export type ThemeSpec = Record<string, StyleSpec>;
	export type StyleSpec = {
		[propOrSelector: string]: string | number | StyleSpec | null;
	};
</script>

<script lang="ts">
	import { createEventDispatcher, onDestroy, onMount } from 'svelte';
	import { basicSetup } from 'codemirror';
	import {
		EditorView,
		highlightSpecialChars,
		keymap,
		lineNumbers,
		placeholder as placeholderExt
	} from '@codemirror/view';
	import {
		EditorState,
		StateEffect,
		type Extension,
		EditorSelection,
		Transaction
	} from '@codemirror/state';
	import { indentWithTab } from '@codemirror/commands';
	import { indentUnit, type LanguageSupport } from '@codemirror/language';

	let classes = '';
	export { classes as class };
	let value: string | null | undefined = '';
	export let initialState: EditorState | undefined = undefined;

	export let basic = true;
	export let theme: Extension | null | undefined = undefined;
	export let lang: LanguageSupport | undefined = undefined;
	export let extensions: Extension[] = [];

	export let useTab = true;
	export let tabSize = 4;

	export let styles: ThemeSpec | null | undefined = undefined;
	export let lineWrapping = false;
	export let editable = true;
	export let readonly = false;
	export let placeholder: string | HTMLElement | null | undefined = undefined;

	const dispatch_change = createEventDispatcher<{ change: string }>();
	const dispatch_transactions = createEventDispatcher<{ transactions: readonly Transaction[] }>();

	export let view: EditorView = undefined as any;

	let element: HTMLDivElement;

	let update_from_prop = false;
	let update_from_state = false;
	let first_config = true;
	let first_update = true;

	import { debounce } from '@melt-ui/svelte/internal/helpers';
	import Editor from '$src/lib/components/editor/Editor.svelte';

	$: state_extensions = [
		...get_base_extensions(
			basic,
			useTab,
			tabSize,
			lineWrapping,
			placeholder,
			editable,
			readonly,
			lang
		),
		...get_theme(theme, styles),
		...extensions
	];

	// $: view && update(value);
	$: view && state_extensions && reconfigure();

	onMount(() => {
		view = create_editor_view();
		element.querySelector('.cm-content')?.setAttribute('tabindex', '-1');
	});
	onDestroy(() => view?.destroy());

	function create_editor_view(): EditorView {
		const on_change = debounce(handle_change, 300);

		return new EditorView({
			parent: element,
			state: create_editor_state(initialState || value),
			dispatchTransactions(transactions: readonly Transaction[]) {
				view.update(transactions);
				dispatch_transactions('transactions', transactions);

				if (!update_from_prop && transactions.some((t) => t.docChanged)) {
					on_change();
				}
			}
		});
	}

	function reconfigure(): void {
		if (first_config) {
			first_config = false;
			return;
		}

		view.dispatch({
			effects: StateEffect.reconfigure.of(state_extensions)
		});
	}

	function update(value: string | null | undefined): void {
		if (first_update) {
			first_update = false;
			return;
		}

		if (update_from_state) {
			update_from_state = false;
			return;
		}

		update_from_prop = true;

		view.setState(create_editor_state(value));

		update_from_prop = false;
	}

	function handle_change(): void {
		const new_value = view.state.doc.toString();
		if (new_value === value) return;

		update_from_state = true;

		value = new_value;
		dispatch_change('change', value ?? '');
	}

	function create_editor_state(value: string | any | null | undefined): EditorState {
		if (typeof value == 'string') {
			return EditorState.create({
				doc: value ?? undefined,
				extensions: state_extensions
			});
		} else if (value) {
			return EditorState.fromJSON(value, { extensions: state_extensions });
		}
		return EditorState.create({ extensions: state_extensions });
	}

	function get_base_extensions(
		basic: boolean,
		useTab: boolean,
		tabSize: number,
		lineWrapping: boolean,
		placeholder: string | HTMLElement | null | undefined,
		editable: boolean,
		readonly: boolean,
		lang: LanguageSupport | null | undefined
	): Extension[] {
		const extensions: Extension[] = [
			indentUnit.of(useTab ? '\t' : ' '.repeat(tabSize)),
			EditorView.editable.of(editable),
			EditorState.readOnly.of(readonly)
		];

		if (placeholder) extensions.push(placeholderExt(placeholder));
		if (lang) extensions.push(lang);
		if (lineWrapping) extensions.push(EditorView.lineWrapping);

		if (readonly) {
			extensions.push(lineNumbers());
			extensions.push(highlightSpecialChars());
			return extensions;
		}

		if (basic) extensions.push(basicSetup);
		if (useTab) extensions.push(keymap.of([indentWithTab]));

		return extensions;
	}

	function get_theme(
		theme: Extension | null | undefined,
		styles: ThemeSpec | null | undefined
	): Extension[] {
		const extensions: Extension[] = [];
		if (styles) extensions.push(EditorView.theme(styles));
		if (theme) extensions.push(theme);
		return extensions;
	}
</script>

<div class="codemirror-wrapper {classes}" bind:this={element} />

<style>
	/*! purgecss start ignore */
	.codemirror-wrapper {
		height: 100%;
	}
	.codemirror-wrapper :global(.cm-focused) {
		outline: none;
	}

	.codemirror-wrapper :global(.cm-editor) {
		height: 100%;
		max-height: none;
	}

	.codemirror-wrapper :global(.cm-gutter) {
		user-select: none;
		border-right-width: 0;
	}
</style>
