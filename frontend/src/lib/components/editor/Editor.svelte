<script lang="ts">
	import RawEditor from './RawEditor.svelte';

	export let value: string | null | undefined;
	export let lang: string | undefined = undefined;

	import { markdown } from '@codemirror/lang-markdown';

	$: lang_support = {
		markdown: markdown,
	}[lang || ''];
</script>

<RawEditor bind:value lang={lang_support ? lang_support() : undefined} />