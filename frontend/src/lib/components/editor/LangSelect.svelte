<script lang="ts">
    import * as Select from "$lib/components/ui/select";
    import { langs } from "$src/lib/components/editor/lang";

    export let value: string | undefined = undefined;

    const onSelectedChange = (s: unknown) => {
      value = (s as any).value;
    };
</script>
 
<Select.Root {onSelectedChange}>
  <Select.Trigger class="w-[180px]">
    <Select.Value placeholder="Select Language" />
  </Select.Trigger>
  <Select.Content>
    {#each Object.entries(langs) as [lang, {}]}
        <Select.Item value={lang}>{lang}</Select.Item>
    {/each}
  </Select.Content>
</Select.Root>

