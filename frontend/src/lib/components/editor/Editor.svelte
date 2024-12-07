<script lang="ts">
	import { langs } from '$src/lib/components/editor/lang';
	import type { EditorView } from 'codemirror';
	import RawEditor from './RawEditor.svelte';
	import { onDestroy } from 'svelte';

	export let value: string | null | undefined;
	export let lang: keyof typeof langs | undefined = undefined;
	export { className as class };
	let className: string = '';

	export let view: EditorView = undefined as any;
	
	export let readonly: boolean = false;
	$: editable = !readonly;

	export let onChange: (value: string) => void = () => {};
	export let onLangChange: (lang: keyof typeof langs | undefined) => void = () => {};

	$: onLangChange(lang);

	onDestroy(() => {
		view?.destroy();
	});
</script>

<RawEditor
	bind:value
	bind:view
	bind:readonly
	bind:editable
	lang={lang && langs[lang] ? langs[lang].lang : undefined}
	class={className}
	useTab={true}
	on:change={(e) => onChange(e.detail)}
/>
