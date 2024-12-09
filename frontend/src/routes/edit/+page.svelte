<script lang="ts">
	import Page from '$src/lib/Page.svelte';
	import { Button } from '$src/lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import RichEditor from '$src/lib/components/editor/RichEditor.svelte';
	import Editor from '$src/lib/components/editor/Editor.svelte';
	import { Play, RefreshCcw } from 'lucide-svelte';

	import { drafts, selected_draft, type ExerciseDraft } from '../create/+page.svelte';
	import { langs, type CodeFn } from '$src/lib/components/editor/lang';
	import type { EditorView } from 'codemirror';
	import GenerateDialog from '$src/routes/edit/GenerateDialog.svelte';
	import ExerciseForm from '$src/routes/edit/ExerciseForm.svelte';
	import type { Writable } from 'svelte/store';
	import { request } from '$src/lib/request';
	import { toast } from 'svelte-sonner';
	import TestCaseEditor from '$src/routes/edit/TestCaseEditor.svelte';
	import LangSelect from '$src/lib/components/editor/LangSelect.svelte';
	import * as api from '$src/routes/api';
	import NumberInput from '$src/lib/components/ui/input/NumberInput.svelte';

	let prev_lang: string | number;
	const onLangChange = (lang: keyof typeof langs | undefined) => {
		if (lang && view && prev_lang !== lang && !$data.generator?.trim()) {
			console.assert(langs[lang].type == 'scripting');
			prev_lang = lang;
			reset();
		}
	};

	const reset = () => {
		if (!$data.generator_lang) return;

		const code_fn: CodeFn = (langs[$data.generator_lang] as any).code; // fine b/c the only fields available to the lang editor are scripting langs
		if (!code_fn) return;
		const { code, cursor } = code_fn({
			gen: ['ctx']
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
	};

	const generate = async () => {
		const res = await api.exec.gen({
			content: $data.generator,
			language: $data.generator_lang,
			inputs: ['int'],
			output: 'int',
			hidden_cases: 1,
			visible_cases: 1,
			generate_cases: $data.generate_cases
		} as any);
		if (res.ok) {
			const data = res.result;
			if (data.exit_code !== undefined) {
				toast.error(`Program exited unsuccessfully with exit code ${data.exit_code}`);
			} else {
				toast.success('Generated!');
				console.log(res);
			}
		}
	};

	let data: Writable<ExerciseDraft>;

	let selected = $selected_draft;
	let exercise = $drafts[selected];

	let view: EditorView;

	const on_update = () => {
		if (view) $data.generator_state = view.state.toJSON();
	};

	let generate_cases_str = $data?.generate_cases?.toString();
</script>

<Page center class="!max-w-4xl grow !flex-row items-stretch justify-stretch gap-1 p-2">
	<div class="card prose flex flex-col !overflow-visible">
		<header>
			<h1>Edit Exercise</h1>
			{#if exercise}
				<span>Edit the exercise details below.</span>
			{:else}
				<span>No exercise found</span>
			{/if}
		</header>
		<section class="flex flex-shrink flex-col overflow-y-scroll !p-0">
			{#if exercise}
				<ExerciseForm
					bind:data
					on:update={on_update}
					on:lang_change={(s) => onLangChange(s.detail?.value)}
				/>
			{/if}
		</section>
	</div>
	<div class="card flex h-auto !max-w-none flex-shrink flex-grow flex-col">
		{#if data && $data.selected_field}
			{@const s = $data.selected_field}
			{#if s === 'description'}
				<RichEditor bind:content={$data.description} />
			{:else if s === 'generator'}
				<div class="flex flex-row items-center justify-between gap-2 p-2">
					<Button variant="default" size="default" on:click={generate}>
						<Play class="mr-1 h-4 w-4" />
						Generate
					</Button>
					<NumberInput
						placeholder="cases"
						class="w-20"
						bind:value={$data.generate_cases}
					/>
					<span class="grow" />
					<Button
						variant="outline"
						size="icon"
						tooltip="Reset code"
						disabled={$data.generator_lang === undefined}
						on:click={reset}
					>
						<!-- TODO: spin?? -->
						<RefreshCcw class="h-4 w-4" />
					</Button>
					<GenerateDialog />
				</div>
				<Editor
					class="shrink border-y border-zinc-300"
					bind:view
					lang={$data.generator_lang}
					readonly={$data.generator_lang === undefined}
					initialStateJSON={$data.generator_state}
					on:transactions={() => ($data.generator_state = view.state.toJSON())}
				/>
				<div class="p-2">
					<LangSelect
						on:change={(s) => onLangChange(s.detail?.value)}
						bind:value={$data.generator_lang}
						filter={(l) => l.type == 'scripting'}
					/>
				</div>
			{:else if s === 'table'}
				<TestCaseEditor />
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
