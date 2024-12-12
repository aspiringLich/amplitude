<script lang="ts">
	import * as Command from '$lib/components/ui/command';
	import * as Popover from '$lib/components/ui/popover';
	import { Input } from '$src/lib/components/ui/input';
	import { cn } from '$src/lib/utils';
	import { Check } from 'lucide-svelte';
	import { createEventDispatcher, tick } from 'svelte';

	export let value: string | undefined = undefined;
	export let value2: string | undefined = undefined;
    export let validate_value2: (value2: string) => boolean = () => true;
	export let values: string[];
	export let placeholder: string = 'Search';
	export let empty: string = 'No Values Found';

	const dispatch = createEventDispatcher<{ select: [string, string] }>();

	let open = false;

	let closeAndFocusTrigger = (triggerId: string) => {
		open = false;
		tick().then(() => {
			document.getElementById(triggerId)?.focus();
		});
	};
</script>

<Popover.Root bind:open let:ids>
	<Popover.Trigger asChild let:builder>
		<slot {builder} {open} />
	</Popover.Trigger>
	<Popover.Content class="w-[200px] p-0">
		<Command.Root>
			<Input
				placeholder="Variable Name"
				bind:value={value2}
				class="h-9 rounded-none border-0 border-b !ring-0"
			/>
			<Command.Input {placeholder} class="h-9" />
			<Command.Empty class="h-full w-full italic">{empty}</Command.Empty>
			<Command.Group>
				{#each values as item}
					<Command.Item
						value={item}
						onSelect={(currentValue) => {
							if (currentValue && value2 && validate_value2(value2)) {
								closeAndFocusTrigger(ids.trigger);
								dispatch('select', [currentValue, value2]);
								value = undefined;
								value2 = undefined;
							}
						}}
					>
						<Check class={cn('mr-2 h-4 w-4', value !== item && 'text-transparent')} />
						{item}
					</Command.Item>
				{/each}
			</Command.Group>
		</Command.Root>
	</Popover.Content>
</Popover.Root>
