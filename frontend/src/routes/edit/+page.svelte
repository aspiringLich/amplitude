<script lang="ts">
	import Page from '$src/lib/Page.svelte';
	import * as Form from '$lib/components/ui/form';
	import * as Tabs from '$lib/components/ui/tabs';
	import { Button } from '$src/lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import RichEditor from '$src/lib/components/editor/RichEditor.svelte';
	import Editor from '$src/lib/components/editor/Editor.svelte';
	import LangSelect from '$src/lib/components/editor/LangSelect.svelte';
	import { CornerUpRight } from 'lucide-svelte';

	import { drafts, selected_draft } from '../create/+page.svelte';
	import { exerciseSchema } from './schema';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { debounce } from '@melt-ui/svelte/internal/helpers';
	import { superForm } from 'sveltekit-superforms';
	import { toast } from 'svelte-sonner';
	import { langs } from '$src/lib/components/editor/lang';
	import type { EditorView } from 'codemirror';

	let selected = $selected_draft;
	let exercise = $drafts[selected];

	const update = debounce(() => {
		$drafts[selected] = $data;
	}, 500);

	const form = superForm(exercise, {
		validators: zodClient(exerciseSchema),
		onChange: update,
		onError: (e) => toast.error(e.result.error.message)
	});
	const { form: data, enhance } = form;

	const select = (field: string) => {
		if ($data.selected_field === field) $data.selected_field = undefined;
		else $data.selected_field = field;
		update();
	};
	
	let generator_tab: string;
	let view: EditorView;
	const onLangChange = (lang: keyof typeof langs | undefined) => {
		if (lang && view) {
			const code_fn = langs[lang].code;
			if (!code_fn) return;
			const { code, cursor } = code_fn({
				func: ['example']
			});
			if (typeof cursor === 'number') {
				const t = view.state.update({
					changes: { from: 0, to: view.state.doc.length, insert: code },
					selection: { anchor: cursor, head: cursor }
				});
				view.dispatch(t);
			} else {
				const [anchor, head] = cursor;
				const t = view.state.update({
					changes: { from: 0, to: view.state.doc.length, insert: code },
					selection: { anchor, head }
				});
				view.dispatch(t);
			}
			view.focus();
		}
	};
</script>

<Page center class="flex w-full max-w-4xl grow items-stretch justify-stretch gap-1 p-2">
	<div class="card prose h-full max-w-full">
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
				<div class="grid grid-cols-[6rem_1fr] gap-2">
					<Form.Field {form} name="description" class="flex items-center">
						<Form.Control let:attrs>
							<Form.Label>Description</Form.Label>
							<Input {...attrs} type="hidden" bind:value={$data.description} />
						</Form.Control>
						<Form.FieldErrors />
					</Form.Field>
					<Button on:click={() => select('description')} size="sm" variant="outline">
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
					<LangSelect
						bind:value={$data.generator_lang}
						filter={({ type }) => type == 'scripting'}
					/>
					<Form.Field {form} name="generator" class="flex items-center">
						<Form.Control let:attrs>
							<Form.Label>Generator</Form.Label>
							<Input {...attrs} type="hidden" bind:value={$data.generator} />
						</Form.Control>
						<Form.FieldErrors />
					</Form.Field>
					<Button on:click={() => select('generator')} size="sm" variant="outline">
						<span>Edit</span>
						<CornerUpRight class="ml-2 h-4 w-4" />
					</Button>
					<Form.Field {form} name="generated_table" class="flex items-center">
						<Form.Control let:attrs>
							<Form.Label>Table</Form.Label>
							<Input {...attrs} type="hidden" bind:value={$data.generated_table} />
						</Form.Control>
						<Form.FieldErrors />
					</Form.Field>
					<Button on:click={() => select('table')} size="sm" variant="outline">
						<span>Edit</span>
						<CornerUpRight class="ml-2 h-4 w-4" />
					</Button>

					<div class=" col-span-2 border-b border-zinc-100" />

					<Button variant="outline" disabled>Preview</Button>
					<Form.Button>Submit</Form.Button>
				</div>
			</form>
		</section>
	</div>
	<div class="card flex h-auto flex-grow flex-col">
		{#if $data.selected_field}
			{@const s = $data.selected_field}
			{#if s === 'description'}
				<RichEditor bind:content={$data.description} />
			{:else if s === 'generator'}
				<Tabs.Root class="h-full flex flex-col" bind:value={generator_tab}>
					<Tabs.List class="flex w-full items-stretch bg-zinc-100">
						<Tabs.Trigger value="editor">Editor</Tabs.Trigger>
						<Tabs.Trigger value="cases">Test Cases</Tabs.Trigger>
					</Tabs.List>
					{#if generator_tab === 'editor'}
						<Editor
							class="grow"
							bind:value={$data.generator}
							bind:view
							lang={$data.generator_lang}
							{onLangChange}
						/>
					{:else if generator_tab === 'cases'}
						<Editor
							class="grow"
							bind:value={$data.generator}
							bind:view
							lang={$data.generator_lang}
							{onLangChange}
						/>
					{/if}
				</Tabs.Root>
			{:else}
				Selected an invalid field. This is a bug.
			{/if}
		{:else}
			<div
				class="text-muted-foreground flex h-full w-full select-none
						items-center justify-center rounded-lg bg-zinc-100 italic"
			>
				<span>No Field Selected</span>
			</div>
		{/if}
	</div>
</Page>
