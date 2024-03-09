<script lang="ts">
	import { Button } from '$src/lib/components/ui/button';
	import type { ExerciseDraft } from '$src/routes/create/schema';
	import { Pane, Splitpanes } from 'svelte-splitpanes';
	import { drafts, selected_draft } from '$src/routes/create/+page.svelte';
	import Page from '$src/lib/Page.svelte';

	let exercise = $drafts[$selected_draft];
</script>

<Page class="h-full">
	<Splitpanes class="h-full w-full" theme="custom-theme">
		<Pane size={50} minSize={20}>
			<a href="/create">
				<Button>Back</Button>
			</a>
		</Pane>
		<Pane size={50} minSize={20}>hell yeah</Pane>
	</Splitpanes>
</Page>
