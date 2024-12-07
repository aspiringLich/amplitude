<script lang="ts">
	import { Button as ButtonPrimitive } from 'bits-ui';
	import { type Events, type Props, buttonVariants } from './index.js';
	import { cn } from '$lib/utils.js';
	import * as Tooltip from '$src/lib/components/ui/tooltip';

	type $$Props = Props;
	type $$Events = Events;

	let className: $$Props['class'] = undefined;
	export let variant: $$Props['variant'] = 'default';
	export let size: $$Props['size'] = 'default';
	export let builders: $$Props['builders'] = [];
	export let tooltip: $$Props['tooltip'] = undefined;
	export { className as class };
</script>

{#if tooltip}
	<Tooltip.Root>
		<Tooltip.Trigger tabindex={-1} let:builder>
			<svelte:self
				{...$$props}
				tooltip={undefined}
				builders={builders ? [...builders, builder] : [builder]}
				on:click
				on:keydown
				aria-label={tooltip}
			>
				<slot />
			</svelte:self>
		</Tooltip.Trigger>
		<Tooltip.Content>{tooltip}</Tooltip.Content>
	</Tooltip.Root>
{:else}
	<ButtonPrimitive.Root
		{builders}
		class={cn(buttonVariants({ variant, size, className }))}
		type="button"
		{...$$restProps}
		on:click
		on:keydown
	>
		<slot />
	</ButtonPrimitive.Root>
{/if}
