<script lang="ts" context="module">
	import { writable, type Writable } from 'svelte/store';
	import { local_store } from '$src/lib/local_store';
	import type { ExerciseDraft } from '$src/routes/create/schema';

	export type State = {
		open?: number;
	};

	export const state: Writable<State> = local_store<State>('create/state', {});
	export const drafts: Writable<ExerciseDraft[]> = local_store('create/drafts', []);
</script>

<script lang="ts">
	import Page from '$lib/Page.svelte';
	import ChooseDraft from './ChooseDraft.svelte';
	import EditDraft from './EditDraft.svelte';
	import { refresh } from '$src/routes/+layout.svelte';

	let open = $state.open;
	state.subscribe((s) => {
		if (s.open !== open) refresh();
	});
</script>

<Page center>
	{#if open === undefined}
		<ChooseDraft />
	{:else}
		<EditDraft draft={$drafts[open]} />
	{/if}
</Page>
