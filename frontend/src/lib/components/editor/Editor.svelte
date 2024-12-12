<script lang="ts">
	import { langs } from '$src/lib/components/editor/lang';
	import type { EditorView } from 'codemirror';
	import RawEditor from './RawEditor.svelte';
	import { onDestroy, onMount } from 'svelte';
	import { EditorState } from '@codemirror/state';

	export let lang: keyof typeof langs | undefined = undefined;
	export let initialStateJSON: any | undefined = undefined;
	export { className as class };
	let className: string = '';

	export let view: EditorView = undefined as any;
	
	export let readonly: boolean = false;
	$: editable = !readonly;

	onDestroy(() => {
		view?.destroy();
	});
	
	onMount(() => {
		view.focus();
		if (initialStateJSON) view.setState(EditorState.fromJSON(initialStateJSON));
	});
</script>

<RawEditor
	bind:view
	bind:readonly
	bind:editable
	lang={lang && langs[lang] ? langs[lang].lang : undefined}
	class={className}
	useTab={true}
	on:change
	on:transactions
/>
