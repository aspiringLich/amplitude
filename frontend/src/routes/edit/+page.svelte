<script lang="ts">
	import Page from '$src/lib/Page.svelte';
	import { Button } from '$src/lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import RichEditor from '$src/lib/components/editor/RichEditor.svelte';
	import Editor from '$src/lib/components/editor/Editor.svelte';
	import { Play } from 'lucide-svelte';
	
	import { type ExerciseDraft } from '../create/+page.svelte';
	import { langs, type CodeFn } from '$src/lib/components/editor/lang';
	import type { EditorView } from 'codemirror';
	import GenerateDialog from '$src/routes/edit/GenerateDialog.svelte';
	import ExerciseForm from '$src/routes/edit/ExerciseForm.svelte';
	import type { Writable } from 'svelte/store';
	
	let view: EditorView;
	const onLangChange = (lang: keyof typeof langs | undefined) => {
		if (lang && view) {
			console.assert(langs[lang].type == 'scripting');
			const code_fn: CodeFn = (langs[lang] as any).code; // fine b/c the only fields available to the lang editor are scripting langs
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
		}
	};
	
	let data: Writable<ExerciseDraft>;
</script>

<Page center class="!flex-row !max-w-4xl grow items-stretch justify-stretch gap-1 p-2">
	<div class="card prose flex flex-col">
		<header>
			<h1>Edit Exercise</h1>
			<span>Edit the exercise details below.</span>
		</header>
		<section class="flex-shrink overflow-scroll flex flex-col !p-0">
			<ExerciseForm bind:data />
		</section>
	</div>
	<div class="card flex h-auto flex-grow flex-col">
		{#if data && $data.selected_field}
			{@const s = $data.selected_field}
			{#if s === 'description'}
				<RichEditor bind:content={$data.description} />
			{:else if s === 'generator'}
				<div class="flex flex-row items-center justify-between border-b border-zinc-300 p-2">
					<span class="flex flex-row">
						<Button
							variant="default"
							size="default"
							class=""
						>
							<Play class="mr-1 h-4 w-4" />
							Generate
						</Button>
						<Input placeholder="cases" class="ml-2 w-20" />
					</span>
					<GenerateDialog />
				</div>
				<Editor
					class="grow"
					bind:value={$data.generator}
					bind:view
					lang={$data.generator_lang}
					{onLangChange}
				/>
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
