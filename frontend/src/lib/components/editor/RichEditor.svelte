<script lang="ts">
	import Button from '$src/lib/components/ui/button/button.svelte';
	import { BoldIcon, ItalicIcon, CodeIcon, type Icon } from 'lucide-svelte';

	import { Editor } from '@tiptap/core';
	import Document from '@tiptap/extension-document';
	import Text from '@tiptap/extension-text';
	import Paragraph from '@tiptap/extension-paragraph';
	import Bold from '@tiptap/extension-bold';
	import Italic from '@tiptap/extension-italic';
	import Code from '@tiptap/extension-code';
	import DropCursor from '@tiptap/extension-dropcursor';
	import GapCursor from '@tiptap/extension-gapcursor';
	import History from '@tiptap/extension-history';

	import { onMount, onDestroy } from 'svelte';

	let editor: Editor;
	let element: HTMLElement;
	let width: number;

	onMount(() => {
		editor = new Editor({
			element,
			extensions: [Document, Text, Paragraph, Bold, Italic, Code, DropCursor, GapCursor, History],

			onTransaction: () => {
				editor = editor;
				const new_active = new Array(menu_items.length);
				for (let i = 0; i < active.length; i++) {
					const item = menu_items[i];
					if (item.type === 'button') new_active[i] = item.active();
				}
				active = new_active;
			}
		});
	});
	onDestroy(() => {
		editor?.destroy();
	});

	type ChainedCommands = ReturnType<typeof editor.chain>;
	type Command = (c: ChainedCommands) => ChainedCommands;
	type Dropdown = [];

	type Base = {
		type: string;
	};
	type MenuButton = Base & {
		type: 'button';
		title: string;
		icon: typeof BoldIcon;
		command: Command;
		active: () => boolean;
	};
	type Spacer = Base & {
		type: 'spacer';
	};
	type MenuItem = MenuButton | Spacer;

	const menu_items: MenuItem[] = [
		{
			type: 'button',
			title: 'Toggle Bold Mark',
			icon: BoldIcon,
			command: (c) => c.toggleBold(),
			active: () => editor.isActive('bold')
		},
		{
			type: 'button',
			title: 'Toggle Italic Mark',
			icon: ItalicIcon,
			command: (c) => c.toggleItalic(),
			active: () => editor.isActive('italic')
		},
		{
			type: 'button',
			title: 'Toggle Code Mark',
			icon: CodeIcon,
			command: (c) => c.toggleCode(),
			active: () => editor.isActive('code')
		},
		{
			type: 'spacer'
		}
	];
	let active = new Array(menu_items.length);

	const execute = (command: Command) => {
		return (e: MouseEvent) => {
			e.preventDefault();
			command(editor.chain().focus()).run();
		};
	};

	const can = (command: Command) => {
		if (!editor) return false;
		return command(editor.can().chain().focus()).run();
	};
</script>

<div class="editor flex h-full w-full flex-col" bind:this={element} bind:clientWidth={width}>
	<div class="flex gap-0.5 border-b p-0.5">
		{#each menu_items || active as item, i}
			{#if item.type === 'spacer'}
				<div class="border-r border-zinc-100 m-0.5" />
			{:else if item.type === 'button'}
				{@const { title, command, icon } = item}
				<Button
					variant={active[i] ? 'default' : 'outline'}
					size="icon-sm"
					{title}
					disabled={!can(command)}
					on:click={execute(command)}
				>
					<svelte:component this={icon} class="h-4 w-4" />
				</Button>
			{/if}
		{/each}
	</div>
</div>

<style lang="postcss">
	:global(.tiptap) {
		@apply min-h-0 flex-[1_1_0] overflow-auto p-2;
		@apply prose prose-sm max-w-none;
	}

	:global(.tiptap code) {
		@apply rounded bg-zinc-200 p-0.5;
	}
</style>
