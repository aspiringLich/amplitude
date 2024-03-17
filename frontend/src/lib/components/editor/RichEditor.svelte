<script lang="ts" context="module">
	import { createLowlight, common } from 'lowlight';
	import 'highlight.js/styles/base16/tomorrow.css';

	let lowlight: any = undefined;
	const getLowlight = () => {
		if (lowlight) return lowlight;
		lowlight = createLowlight();
		lowlight.register(common);
		return lowlight;
	};
</script>

<script lang="ts">
	import Button from '$src/lib/components/ui/button/button.svelte';
	import { BoldIcon, ItalicIcon, CodeIcon, FileCode2Icon, Wand2Icon } from 'lucide-svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';

	import { Editor } from '@tiptap/core';
	import Document from '@tiptap/extension-document';
	import Text from '@tiptap/extension-text';
	import Paragraph from '@tiptap/extension-paragraph';
	import Bold from '@tiptap/extension-bold';
	import Italic from '@tiptap/extension-italic';
	import Code from '@tiptap/extension-code';
	import CodeBlockLowLight from '@tiptap/extension-code-block-lowlight';
	import DropCursor from '@tiptap/extension-dropcursor';
	import GapCursor from '@tiptap/extension-gapcursor';
	import History from '@tiptap/extension-history';

	import { onMount, onDestroy } from 'svelte';
	import { debounce, sleep } from '@melt-ui/svelte/internal/helpers';
	
	export let content: string | undefined = undefined;
	
	let editor: Editor;
	let element: HTMLElement;
	let width: number;
	
	const update = debounce(() => {
		if (editor) {
			content = editor.getHTML();
		}
	}, 500);

	onMount(() => {
		editor = new Editor({
			element,
			extensions: [
				Document,
				Text,
				Paragraph,
				Bold,
				Italic,
				Code,
				CodeBlockLowLight.configure({ lowlight: getLowlight() }),
				DropCursor,
				GapCursor,
				History
			],
			onTransaction: () => {
				editor = editor;
				update();
			},
			content,
		});
	});
	onDestroy(() => {
		editor?.destroy();
	});

	type ChainedCommands = ReturnType<typeof editor.chain>;
	type Command = (c: ChainedCommands) => ChainedCommands;

	const execute = (command: Command) => {
		return (e: MouseEvent) => {
			command(editor.chain().focus()).focus().run();
		};
	};
	const variant = (active: boolean) => (active ? 'default' : 'outline');
	const can = (command: Command) => {
		if (!editor) return false;
		return command(editor.can().chain()).run();
	};

	// force focus back to editor (jank but it works so like)
	const forceFocus = () => {
		const r = document.activeElement?.getAttribute('data-mark') === 'dropdown-insert-rich-editor';
		if (r) editor.commands.focus();
		return r;
	};
	const setForceFocus = () => {
		let max = 100;
		setTimeout(async () => {
			while (!forceFocus() && max-- > 0) await sleep(10);
		}, 0);
	};
	const insertCodeBlock = (language?: string) => {
		return execute((c) => {
			const out = c.insertContent({
				type: 'codeBlock',
				attrs: { language }
			});
			setForceFocus();
			return out;
		});
	};
</script>

<div class="editor flex h-full w-full flex-col" bind:this={element} bind:clientWidth={width}>
	<div class="flex items-center gap-0.5 border-b p-0.5">
		{#if editor}
			<Button
				variant={variant(editor?.isActive('bold'))}
				size="icon-xs"
				title="Toggle Bold Mark"
				disabled={!can((c) => c.toggleBold()) ?? editor}
				on:click={execute((c) => c.toggleBold())}
			>
				<BoldIcon class="h-4 w-4" />
			</Button>
			<Button
				variant={variant(editor?.isActive('italic'))}
				size="icon-xs"
				title="Toggle Italic Mark"
				disabled={!can((c) => c.toggleItalic()) ?? editor}
				on:click={execute((c) => c.toggleItalic())}
			>
				<ItalicIcon class="h-4 w-4" />
			</Button>
			<Button
				variant={variant(editor?.isActive('code'))}
				size="icon-xs"
				title="Toggle Code Mark"
				disabled={!can((c) => c.toggleCode()) ?? editor}
				on:click={execute((c) => c.toggleCode())}
			>
				<CodeIcon class="h-4 w-4" />
			</Button>
			<div class="m-1 h-full border border-zinc-100" />
			<DropdownMenu.Root>
				<DropdownMenu.Trigger asChild let:builder>
					<Button
						builders={[builder]}
						variant="outline"
						size="sm"
						class="h-6 italic"
						title="Insert Content"
						data-mark="dropdown-insert-rich-editor"
						disabled={!can((c) => c.insertContent('')) ?? editor}
					>
						Insert
					</Button>
				</DropdownMenu.Trigger>
				<DropdownMenu.Content>
					<DropdownMenu.Group>
						<DropdownMenu.Sub>
							<DropdownMenu.SubTrigger>
								<FileCode2Icon class="mr-2 h-4 w-4" />
								<span>Code Block</span>
							</DropdownMenu.SubTrigger>
							<DropdownMenu.SubContent>
								<DropdownMenu.Label>Language</DropdownMenu.Label>
								<DropdownMenu.Separator />
								<DropdownMenu.Item on:click={insertCodeBlock(undefined)}>
									<Wand2Icon class="mr-2 h-4 w-4" />
									<span>Autodetect</span>
								</DropdownMenu.Item>
								
								<DropdownMenu.Item on:click={insertCodeBlock('java')}>
									<FileCode2Icon class="mr-2 h-4 w-4" />
									<span>Java</span>
								</DropdownMenu.Item>
								<DropdownMenu.Item on:click={insertCodeBlock('javascript')}>
									<FileCode2Icon class="mr-2 h-4 w-4" />
									<span>JavaScript</span>
								</DropdownMenu.Item>
								<DropdownMenu.Item on:click={insertCodeBlock('python')}>
									<FileCode2Icon class="mr-2 h-4 w-4" />
									<span>Python</span>
								</DropdownMenu.Item>
							</DropdownMenu.SubContent>
						</DropdownMenu.Sub>
					</DropdownMenu.Group>
				</DropdownMenu.Content>
			</DropdownMenu.Root>
		{/if}
	</div>
</div>

<style lang="postcss">
	:global(.tiptap) {
		@apply min-h-0 flex-[1_1_0] overflow-auto p-2;
		@apply prose prose-sm max-w-none;
	}

	:global(.tiptap :not(pre) code) {
		@apply rounded bg-zinc-100 p-0.5;
	}

	:global(.tiptap pre) {
		@apply my-1 rounded bg-zinc-100 p-1;
	}
</style>
