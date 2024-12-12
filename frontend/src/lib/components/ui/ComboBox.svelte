<script lang="ts">
	import * as Command from '$lib/components/ui/command';
	import * as Popover from '$lib/components/ui/popover';
	import { cn } from '$src/lib/utils';
	import { Check } from 'lucide-svelte';
	import { createEventDispatcher, tick } from 'svelte';

	export let value: string | undefined = undefined;
	export let values: string[];
	export let check: boolean = true;
	export let placeholder: string = 'Search';
	export let empty: string = 'No Values Found';

	const dispatch = createEventDispatcher<{ select: string }>();

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
			<Command.Input {placeholder} class="h-9" />
			<Command.Empty class="h-full w-full italic">{empty}</Command.Empty>
			<Command.Group>
				{#each values as item}
					<Command.Item
						value={item}
						onSelect={(currentValue) => {
							value = currentValue;
							closeAndFocusTrigger(ids.trigger);
							dispatch('select', value);
						}}
					>
						{#if check}
							<Check class={cn('mr-2 h-4 w-4', value !== item && 'text-transparent')} />
						{/if}
						{item}
					</Command.Item>
				{/each}
			</Command.Group>
		</Command.Root>
	</Popover.Content>
</Popover.Root>
