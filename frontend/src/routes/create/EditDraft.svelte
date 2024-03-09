<script lang="ts">
	import { Button } from '$src/lib/components/ui/button';
	import { state } from '$src/routes/create/+page.svelte';
	import type { ExerciseDraft } from '$src/routes/create/schema';

	export let draft: ExerciseDraft;
</script>

<Button on:click={() => ($state.open = undefined)}>Back</Button>
