<script lang="ts">
	import { langs } from '$src/lib/components/editor/lang';
	import type { EditorView } from 'codemirror';
	import RawEditor from './RawEditor.svelte';

	export let value: string | null | undefined;
	export let lang: keyof typeof langs | undefined = undefined;
	export let view: EditorView;
	
	export let onChange: (value: string) => void = () => {};
	export let onLangChange: (lang: keyof typeof langs | undefined) => void = () => {};
	
	$: onLangChange(lang);
</script>

<RawEditor
	bind:value
	bind:view
	lang={lang && langs[lang] ? langs[lang].lang : undefined}
	on:change={(e) => onChange(e.detail)}
/>
