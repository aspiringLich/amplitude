<script lang="ts">
	import { Button } from '$src/lib/components/ui/button';
	import type { ExerciseDraft } from '$src/routes/create/schema';
	import { Pane, Splitpanes } from 'svelte-splitpanes';
	import { drafts, selected_draft } from '$src/routes/create/+page.svelte';
	import Page from '$src/lib/Page.svelte';

	let exercise = $drafts[$selected_draft];
</script>

<Page center class="max-w-4xl grow">
	<Splitpanes class="h-min w-full items-stretch" theme="custom-theme">
		<Pane size={50} minSize={20} class="!h-auto p-2 pr-0">
			<div class="card prose max-w-full">
				<section>yep</section>
			</div>
		</Pane>
		<Pane size={50} minSize={20} class="!h-auto p-2 pl-0">
			<div class="card h-full"></div>
		</Pane>
	</Splitpanes>
</Page>
