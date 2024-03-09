<script lang="ts">
	import { Button } from '$src/lib/components/ui/button';
	import type { ExerciseDraft } from '$src/routes/create/schema';
	import { Pane, Splitpanes } from 'svelte-splitpanes';
	import { drafts, selected_draft } from '$src/routes/create/+page.svelte';
	import Page from '$src/lib/Page.svelte';

	let exercise = $drafts[$selected_draft];
</script>

<Page class="h-full">
	<Splitpanes class="" theme="custom-theme">
		<Pane size={50} minSize={20} class="p-2 pr-0">
			<div class="card h-full">
                
            </div>
		</Pane>
		<Pane size={50} minSize={20} class="p-2 pl-0">
			<div class="card h-full"></div>
		</Pane>
	</Splitpanes>
</Page>
