<script lang="ts">
	import * as Select from '$lib/components/ui/select';
	import { langs, type LangInfo } from '$src/lib/components/editor/lang';

	export let value: string | undefined = undefined;
	export let filter: (lang: LangInfo) => boolean = () => true;

	const onSelectedChange = (s: unknown) => {
		value = (s as any).value;
	};
</script>

<Select.Root {onSelectedChange}>
	<Select.Trigger class="w-[180px]">
		<Select.Value placeholder="Select Language" />
	</Select.Trigger>
	<Select.Content>
		{#each Object.entries(langs) as [lang, info]}
			{#if filter(info)}
				<Select.Item value={lang}>{lang}</Select.Item>
			{/if}
		{/each}
	</Select.Content>
</Select.Root>
