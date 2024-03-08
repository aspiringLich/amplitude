<script lang="ts">
	import type { HTMLAttributes } from 'svelte/elements';
	import { cn } from '$lib/utils';

	type $$Props = HTMLAttributes<HTMLDivElement>;

	let className: $$Props['class'] = undefined;
	export { className as class };
</script>

<div
	class={cn('card bg-card text-card-foreground rounded-lg border shadow-sm', className)}
	{...$$restProps}
>
	<slot />
</div>

<style>
	.card > :global(header) {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
		padding: 1.5rem;
	}

	.card > :global(header > h1) {
		font-size: 1.25rem;
		font-weight: 600;
		line-height: 1;
		letter-spacing: -0.0125em;
	}

	.card > :global(header > span) {
		color: var(--text-muted);
		font-size: 0.875rem;
	}

	.card > :global(section) {
		padding: 1.5rem;
		padding-top: 0;
	}

	.card > :global(footer) {
		display: flex;
		align-items: center;
		padding: 1.5rem;
		padding-top: 0;
	}
	/* .card :global(header) {
		@apply flex flex-col space-y-1.5 p-6;
	}

	.card :global(title) {
		@apply text-lg font-semibold leading-none tracking-tight;
	}

	.card :global(desc) {
		@apply text-muted-foreground text-sm;
	}

	.card :global(section) {
		@apply p-6 pt-0;
	}

	.card :global(footer) {
		@apply flex items-center p-6 pt-0;
	} */
</style>
