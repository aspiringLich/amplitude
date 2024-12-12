<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Button } from '$src/lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { ChevronsUpDown, CornerUpRight, Plus, X } from 'lucide-svelte';

	import { type ExerciseDraft } from '../create/+page.svelte';
	import { type SuperForm } from 'sveltekit-superforms';
	import type { Writable } from 'svelte/store';
	import ComboBox from '$src/lib/components/ui/ComboBox.svelte';
	import { valid_types } from '$src/routes/api/schema';
	import Badge from '$src/lib/components/ui/badge/badge.svelte';
	import { createEventDispatcher } from 'svelte';
	import ComboBox2 from '$src/lib/components/ui/ComboBox2.svelte';
	import { identifierSchema } from '$src/routes/api/exec/schema';

	export let data: Writable<ExerciseDraft>;
	export let form: SuperForm<ExerciseDraft, any>;

	const dispatch = createEventDispatcher();

	const select = (field: string | undefined) => {
		if ($data.selected_field === field) $data.selected_field = undefined;
		else $data.selected_field = field;
	};
	const variant = (data: typeof $data, field: string) => {
		if (data.selected_field === field) return 'secondary';
		else return 'outline';
	};

	$: solution_enabled = $data.args?.length && $data.output && update;
	$: if (!solution_enabled && $data.selected_field == 'solution') select(undefined);

	// NOTE: make sure this stays consistent on +page.svelte
	$: solution_editor_enabled =
		$data.args?.length &&
		$data.output &&
		$data.function_name?.length &&
		$data.solution_lang !== undefined;
	$: if (
		!solution_editor_enabled &&
		['generator', 'table'].indexOf($data.selected_field as string) != -1
	)
		select(undefined);

	let types: string[] = valid_types(undefined);
	let update = 1;
	let input_type_value: any;
</script>

<div class="flex-pass border-y border-zinc-300 px-6 py-4">
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
		<Form.Field {form} name="args" class="col-span-2">
			<Form.Control let:attrs>
				<Form.Label class="flex items-center">Arguments</Form.Label>
				<Input {...attrs} type="hidden" />
				<div class="flex">
					<div
						class="border-input bg-background ring-offset-background flex grow select-none
						items-center space-x-1 overflow-x-scroll rounded-l border border-r-0 px-1 text-sm"
					>
						{#key update}
							{#each $data.args ?? [] as { type, arg }, i}
								<Badge class="flex whitespace-nowrap p-0">
									<span class="my-0.5 ml-2.5">{type} <i>{arg}</i></span>
									<Button
										size="icon-xs"
										class="!bg-transparent p-1"
										on:click={() => {
											$data.args.splice(i, 1);
											dispatch('update');
											update += 1;
										}}
									>
										<X />
									</Button>
								</Badge>
							{/each}
						{/key}
					</div>
					<ComboBox2
						bind:value={input_type_value}
						values={types}
						let:builder
						let:open
						on:select={(e) => {
							if (!$data.args) $data.args = [];
							$data.args.push({ arg: e.detail[1], type: e.detail[0] });
							dispatch('update');
							update += 1;
						}}
						validate_value2={(value2) => {
							let res = identifierSchema.safeParse(value2);
							// TODO: no duplicates
							return res.success;
						}}
					>
						<Button
							builders={[builder]}
							variant="outline"
							class="rounded-l-none"
							size="icon"
							role="combobox"
							aria-expanded={open}
						>
							<Plus class="h-4 w-4" />
						</Button>
					</ComboBox2>
				</div>
				<Form.FieldErrors class="col-span-2" />
			</Form.Control>
		</Form.Field>
		<Form.Field {form} name="output" class="contents">
			<Form.Control let:attrs>
				<Form.Label class="flex items-center">Return Type</Form.Label>
				<Input {...attrs} type="hidden" bind:value={$data.solution} />
			</Form.Control>
			<ComboBox values={types} let:builder let:open bind:value={$data.output}>
				<Button
					builders={[builder]}
					role="combobox"
					size="sm"
					variant="outline"
					class="!mt-0"
					aria-expanded={open}
				>
					<span class="grow text-left">{$data.output}</span>
					<ChevronsUpDown class="h-4 w-4" />
				</Button>
			</ComboBox>
			<Form.FieldErrors class="col-span-2 !mt-0" />
		</Form.Field>
		<Form.Field {form} name="solution" class="contents">
			<Form.Control let:attrs>
				<Form.Label class="flex items-center">Solution</Form.Label>
				<Input {...attrs} type="hidden" bind:value={$data.solution} />
			</Form.Control>
			{#key update}
				<Button
					on:click={() => select('solution')}
					size="sm"
					variant={variant($data, 'solution')}
					class="!mt-0"
					disabled={!($data.args?.length && $data.output)}
				>
					<span>Edit</span>
					<CornerUpRight class="ml-2 h-4 w-4" />
				</Button>
			{/key}
			<Form.FieldErrors class="col-span-2 !mt-0" />
		</Form.Field>

		<h6 class="col-span-2 mt-2">Test Cases</h6>
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
				disabled={!solution_editor_enabled}
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
				disabled={!solution_editor_enabled}
			>
				<span>Edit</span>
				<CornerUpRight class="ml-2 h-4 w-4" />
			</Button>
			<Form.FieldErrors class="col-span-2 !mt-0" />
		</Form.Field>
	</div>
</div>
<div class="flex flex-row-reverse gap-2 p-2">
	<Form.Button>Publish</Form.Button>
</div>
