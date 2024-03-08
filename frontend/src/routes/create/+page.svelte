<script lang="ts" context="module">
	import { writable, type Writable } from 'svelte/store';
	import { local_store } from '$src/lib/local_store';
	import type { Exercise } from '$src/routes/create/schema';

	export type State = {
		open?: string;
	};

	export const state: Writable<State> = local_store<State>('create/state', {});
	export const ex_drafts: Writable<Exercise[]> = writable([]);
</script>

<script lang="ts">
	import Page from '$lib/Page.svelte';
	import ChooseDraft from '$src/routes/create/ChooseDraft.svelte';

	let open = $state.open;
</script>

<Page center>
	{#if !open}
		<ChooseDraft />
	{:else}
		gaming
	{/if}
</Page>
