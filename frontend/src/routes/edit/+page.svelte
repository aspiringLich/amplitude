<script lang="ts">
	import { drafts, selected_draft } from '../create/+page.svelte';
	import Page from '$src/lib/Page.svelte';
	import * as Form from '$lib/components/ui/form';
	import { Pane, Splitpanes } from 'svelte-splitpanes';
	import { Button } from '$src/lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';

	import { exerciseSchema } from './schema';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { debounce } from '@melt-ui/svelte/internal/helpers';
	import { superForm } from 'sveltekit-superforms';

	let selected = $selected_draft;
	let exercise = $drafts[selected];

	const update = () => {
		$drafts[selected] = $data;
	};

	const form = superForm(exercise, {
		validators: zodClient(exerciseSchema),
		onChange: debounce(update, 500)
	});
	const { form: data, enhance } = form;

	let selected_field: string | null = null;
</script>

<Page center class="max-w-4xl grow">
	<Splitpanes class="h-min w-full items-stretch" theme="custom-theme">
		<Pane size={50} minSize={20} class="!h-auto pl-2">
			<div class="card prose max-w-full">
				<header>
					<h1>Edit Exercise</h1>
					<span>Edit the exercise details below.</span>
				</header>
				<section>
					<form method="POST" use:enhance>
						<Form.Field {form} name="title">
							<Form.Control let:attrs>
								<Form.Label>Title</Form.Label>
								<Input {...attrs} bind:value={$data.title} />
							</Form.Control>
							<Form.FieldErrors />
						</Form.Field>
					</form>
				</section>
			</div>
		</Pane>
		<Pane size={50} minSize={20} class="!h-auto pr-2">
			<div class="card h-full">
				{#if selected_field}
					<!--  -->
				{:else}
					<div
						class="text-muted-foreground h-full w-full rounded-lg bg-zinc-100
						flex items-center justify-center italic select-none"
					>
						<span>No Field Selected</span>
					</div>
				{/if}
			</div>
		</Pane>
	</Splitpanes>
</Page>