<script lang="ts">
	import * as Select from '$lib/components/ui/select';
	import { langs, type LangInfo } from '$src/lib/components/editor/lang';
	import type { Selected } from 'bits-ui';

	export let value: string | undefined = undefined;
	export let filter: (lang: LangInfo) => boolean = () => true;

	let selected: Selected<string> = { value: value || '' };
</script>

<Select.Root {selected} onSelectedChange={(s) => s && (value = s.value)}>
	<Select.Trigger class="w-[180px]">
		<Select.Value placeholder="Select Language" />
	</Select.Trigger>
	<Select.Content>
		{#each Object.entries(langs) as [lang, info]}
			{#if filter(info)}
				<Select.Item value={lang} label={lang}>{lang}</Select.Item>
			{/if}
		{/each}
	</Select.Content>
</Select.Root>
