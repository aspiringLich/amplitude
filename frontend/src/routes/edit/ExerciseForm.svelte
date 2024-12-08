<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Button } from '$src/lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import LangSelect from '$src/lib/components/editor/LangSelect.svelte';
	import { CornerUpRight } from 'lucide-svelte';

	import { drafts, selected_draft } from '../create/+page.svelte';
	import { exerciseSchema } from './schema';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { debounce } from '@melt-ui/svelte/internal/helpers';
	import { superForm } from 'sveltekit-superforms';
	import { toast } from 'svelte-sonner';

	let selected = $selected_draft;
	let exercise = $drafts[selected];

	const update = debounce(() => {
		$drafts[selected] = $data;
	}, 500);

	const form = superForm(exercise, {
		validators: zodClient(exerciseSchema),
		onChange: update,
		onError: (e) => toast.error(e.result.error.message),
		onSubmit: (e) => {
			$drafts[selected] = $data;
		},
		onResult(e) {
			let res = e.result;
			switch (res.type) {
				case 'error':
					console.error(res);
					toast.error(res.error.message);
					break;
				case 'success':
					toast.success('Exercise submitted!');
					break;
				case 'failure':
					console.error(res);
					toast.error(`Submission Failiure: ${res.status}`);
					break;
				case 'redirect':
			}
		},
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
		<div class="grid flex-[1_0_auto] grid-cols-[6rem_1fr] gap-x-2 gap-y-1">
			<Form.Field {form} name="description" class="contents">
				<Form.Control let:attrs>
					<Form.Label class="flex items-center">Description</Form.Label>
					<Input {...attrs} type="hidden" bind:value={$data.description} />
				</Form.Control>
				<Button
					on:click={() => select('description')}
					size="sm"
					variant={variant($data, 'description')}
					class="!mt-0"
				>
					<span>Edit</span>
					<CornerUpRight class="ml-2 h-4 w-4" />
				</Button>
				<Form.FieldErrors class="col-span-2 !mt-0" />
			</Form.Field>

			<h6 class="col-span-2 mt-2">Solution</h6>
			
			<h6 class="col-span-2 mt-2">Test Cases</h6>

			<Form.Field {form} name="generator_lang" class="contents">
				<Form.Control let:attrs>
					<Form.Label class="flex items-center">Generator Language</Form.Label>
					<Input {...attrs} type="hidden" bind:value={$data.generator_lang} />
				</Form.Control>
				<LangSelect
					bind:value={$data.generator_lang}
					filter={({ type }) => type == 'scripting'}
					class="!mt-0"
				/>
				<Form.FieldErrors class="col-span-2 !mt-0" />
			</Form.Field>
			<Form.Field {form} name="generator" class="contents">
				<Form.Control let:attrs>
					<Form.Label class="flex items-center">Generator</Form.Label>
					<Input {...attrs} type="hidden" bind:value={$data.generator} />
				</Form.Control>
				<Button
					on:click={() => select('generator')}
					size="sm"
					variant={variant($data, 'generator')}
					class="!mt-0"
				>
					<span>Edit</span>
					<CornerUpRight class="ml-2 h-4 w-4" />
				</Button>
				<Form.FieldErrors class="col-span-2 !mt-0" />
			</Form.Field>
			<Form.Field {form} name="generated_table" class="contents">
				<Form.Control let:attrs>
					<Form.Label class="flex items-center">Test Cases</Form.Label>
					<Input {...attrs} type="hidden" bind:value={$data.generated_table} />
				</Form.Control>
				<Button
					on:click={() => select('table')}
					size="sm"
					variant={variant($data, 'table')}
					class="!mt-0"
				>
					<span>Edit</span>
					<CornerUpRight class="ml-2 h-4 w-4" />
				</Button>
				<Form.FieldErrors class="col-span-2 !mt-0" />
			</Form.Field>
		</div>
	</span>
	<div class="flex flex-row-reverse gap-2 p-6 pt-4">
		<Form.Button>Publish</Form.Button>
	</div>
</form>
