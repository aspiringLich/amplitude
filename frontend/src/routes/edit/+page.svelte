<script lang="ts">
	import Page from '$src/lib/Page.svelte';
	import { Button } from '$src/lib/components/ui/button';
	import RichEditor from '$src/lib/components/editor/RichEditor.svelte';
	import Editor from '$src/lib/components/editor/Editor.svelte';
	import { Play, RefreshCcw } from 'lucide-svelte';

	import { drafts, selected_draft } from '../create/+page.svelte';
	import { langs, type CodeFn, type CodeFnDef } from '$src/lib/components/editor/lang';
	import type { EditorView } from 'codemirror';
	import GenerateDialog from '$src/routes/edit/GenerateDialog.svelte';
	import ExerciseForm from '$src/routes/edit/ExerciseForm.svelte';
	import { toast } from 'svelte-sonner';
	import TestCaseEditor from '$src/routes/edit/TestCaseEditor.svelte';
	import LangSelect from '$src/lib/components/editor/LangSelect.svelte';
	import * as api from '$src/routes/api';
	import NumberInput from '$src/lib/components/ui/input/NumberInput.svelte';
	import { debounce } from '@melt-ui/svelte/internal/helpers';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { exerciseSchema } from '$src/routes/api/exec/schema';
	import Input from '$src/lib/components/ui/input/input.svelte';

	let selected = $selected_draft;
	let exercise = $drafts[selected];

	const update = debounce(() => {
		$drafts[selected] = $data;
	}, 500);

	let form = superForm(exercise, {
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
					if (res.status != 400) toast.error(`Submission Failiure: ${res.status}`);
					break;
				case 'redirect':
			}
		},
		dataType: 'json'
	});
	let { form: data, enhance, errors } = form;

	let prev_lang: string | number;
	const onLangChange = (lang: keyof typeof langs | undefined) => {
		if (lang && view && prev_lang !== lang && !$data.generator?.trim()) {
			console.assert(langs[lang].type == 'scripting');
			prev_lang = lang;
			reset_gen_view();
		}
	};

	const reset_solution_view = () => {
		let cfg = {} as any;
		cfg[$data.function_name] = { args: $data.args };
		reset_view($data.solution_lang, cfg);
	};
	const reset_gen_view = () =>
		reset_view($data.generator_lang, { gen: { args: [{ arg: 'ctx', type: '' }], output: '' } });
	const reset_view = (lang: string | undefined, cfg: { [key: string]: CodeFnDef }) => {
		if (!lang) return;

		const code_fn: CodeFn = (langs[lang] as any).code; // fine b/c the only fields available to the lang editor are scripting langs
		if (!code_fn) return;
		const { code, cursor } = code_fn(cfg);
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

	let view: EditorView;

	// NOTE: make sure this stays consistent on ExerciseForm.svelte
	$: solution_editor_enabled =
		$data.args?.length &&
		$data.output &&
		$data.function_name?.length &&
		$data.solution_lang !== undefined;

	// const on_update = () => {
	// 	if (view) $data.generator_state = view.state.toJSON();
	// };
</script>

<Page center class="!max-w-4xl grow">
	<form
		use:enhance
		method="POST"
		class="flex w-full !max-w-none !flex-row items-stretch justify-stretch gap-1 p-2"
	>
		<div class="card prose flex min-w-72 max-w-72 flex-col !overflow-visible">
			<header>
				<h1>Edit Exercise</h1>
				{#if exercise}
					<span>Edit the exercise details below.</span>
				{:else}
					<span>No exercise found</span>
				{/if}
			</header>
			<section class="flex-pass !p-0">
				{#if exercise}
					<ExerciseForm
						bind:data
						bind:form
						on:update={update}
						on:lang_change={(s) => onLangChange(s.detail?.value)}
					/>
				{/if}
			</section>
		</div>
		<div class="card flex h-auto !max-w-none grow flex-col">
			{#if data && $data.selected_field}
				{@const s = $data.selected_field}
				{#if s == 'description'}
					<RichEditor bind:content={$data.description} />
				{:else if s == 'generator'}
					<div class="flex flex-row items-center justify-between gap-2 p-2">
						<Button variant="default" size="default" on:click={generate}>
							<Play class="mr-1 h-4 w-4" />
							Generate
						</Button>
						<NumberInput placeholder="cases" class="w-20" bind:value={$data.generate_cases} />
						<span class="grow" />
						<Button
							variant="outline"
							size="icon"
							tooltip="Reset code"
							disabled={$data.generator_lang === undefined}
							on:click={reset_gen_view}
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
				{:else if s == 'solution' && $data.args?.length && $data.output}
					<div class="flex flex-row items-center justify-between gap-2 p-2">
						<Input placeholder="Function Name" class="w-40" bind:value={$data.function_name} />
						<span class="grow" />
						<Button
							variant="outline"
							size="icon"
							tooltip="Reset code"
							disabled={!solution_editor_enabled}
							on:click={reset_solution_view}
						>
							<!-- TODO: spin?? -->
							<RefreshCcw class="h-4 w-4" />
						</Button>
						<!-- <GenerateDialog /> -->
					</div>
					<Editor
						class="shrink border-y border-zinc-300"
						bind:view
						lang={$data.solution_lang}
						readonly={!solution_editor_enabled}
						initialStateJSON={$data.solution_state}
						on:transactions={() => ($data.solution_state = view.state.toJSON())}
					/>
					<div class="p-2">
						<LangSelect
							on:change={(s) => onLangChange(s.detail?.value)}
							bind:value={$data.solution_lang}
							filter={(l) => l.type == 'scripting'}
						/>
					</div>
				{:else if s == 'table'}
					<TestCaseEditor />
				{:else}
					<div
						class="text-muted-foreground flex h-full w-full select-none
						items-center justify-center rounded-lg bg-zinc-100 italic"
					>
						<span>No Field Selected</span>
					</div>
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
	</form>
</Page>

<style lang="postcss">
	form :global(div[data-fs-field-errors]) {
		@apply cursor-default;
	}

	form :global(div[data-fs-error]) {
		@apply inline break-words pb-4 leading-3;
	}
</style>
