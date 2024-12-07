<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Button } from '$src/lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import LangSelect from '$src/lib/components/editor/LangSelect.svelte';
	import { CornerUpRight } from 'lucide-svelte';

	import { drafts, selected_draft, type ExerciseDraft } from '../create/+page.svelte';
	import { exerciseSchema, type Exercise } from './schema';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { debounce } from '@melt-ui/svelte/internal/helpers';
	import { superForm, type SuperValidated } from 'sveltekit-superforms';
	import { toast } from 'svelte-sonner';
	import { langs, type CodeFn } from '$src/lib/components/editor/lang';
	import type { EditorView } from 'codemirror';

	let selected = $selected_draft;
	let exercise = $drafts[selected];

	const update = debounce(() => {
		$drafts[selected] = $data;
	}, 500);

	const form = superForm(exercise, {
		validators: zodClient(exerciseSchema),
		onChange: update,
		onError: (e) => toast.error(e.result.error.message),
		dataType: 'json'
	});
	const { form: formData, enhance } = form;
	export let data = formData;

	const select = (field: string) => {
		if ($data.selected_field === field) $data.selected_field = undefined;
		else $data.selected_field = field;
	};
	const variant = (data: typeof $data, field: string) => {
		if (data.selected_field === field) return 'secondary';
		else return 'outline';
	};
</script>

<form method="POST" use:enhance class="flex flex-shrink flex-col overflow-scroll">
	<span class="flex flex-shrink flex-col overflow-scroll border-y border-zinc-300 px-6 py-4">
		<Form.Field {form} name="title">
			<Form.Control let:attrs>
				<Form.Label>Title</Form.Label>
				<Input {...attrs} bind:value={$data.title} />
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>
		<div class="grid flex-[1_0_auto] grid-cols-[6rem_1fr] gap-2">
			<Form.Field {form} name="description" class="flex items-center">
				<Form.Control let:attrs>
					<Form.Label>Description</Form.Label>
					<Input {...attrs} type="hidden" bind:value={$data.description} />
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>
			<Button
				on:click={() => select('description')}
				size="sm"
				variant={variant($data, 'description')}
			>
				<span>Edit</span>
				<CornerUpRight class="ml-2 h-4 w-4" />
			</Button>

			<h6 class="col-span-2 mt-2">Test Cases</h6>
			<Form.Field {form} name="generator_lang" class="flex items-center">
				<Form.Control let:attrs>
					<Form.Label>Generator Language</Form.Label>
					<Input {...attrs} type="hidden" bind:value={$data.generator_lang} />
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>
			<LangSelect bind:value={$data.generator_lang} filter={({ type }) => type == 'scripting'} />
			<Form.Field {form} name="generator" class="flex items-center">
				<Form.Control let:attrs>
					<Form.Label>Generator</Form.Label>
					<Input {...attrs} type="hidden" bind:value={$data.generator} />
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>
			<Button on:click={() => select('generator')} size="sm" variant={variant($data, 'generator')}>
				<span>Edit</span>
				<CornerUpRight class="ml-2 h-4 w-4" />
			</Button>
			<Form.Field {form} name="generated_table" class="flex items-center">
				<Form.Control let:attrs>
					<Form.Label>Test Cases</Form.Label>
					<Input {...attrs} type="hidden" bind:value={$data.generated_table} />
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>
			<Button on:click={() => select('table')} size="sm" variant={variant($data, 'table')}>
				<span>Edit</span>
				<CornerUpRight class="ml-2 h-4 w-4" />
			</Button>
		</div>
	</span>
	<div class="flex flex-row-reverse gap-2 p-6 pt-4">
		<Form.Button on:click={() => ($drafts[selected] = $data)}>Submit</Form.Button>
	</div>
</form>
