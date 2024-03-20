<script lang="ts">
	import { langs } from '$src/lib/components/editor/lang';
	import RawEditor from './RawEditor.svelte';

	export let value:  string | null | undefined;
	export let lang: keyof typeof langs | undefined = undefined; 
</script>

<RawEditor bind:value lang={lang && langs[lang] ? langs[lang].lang : undefined} />