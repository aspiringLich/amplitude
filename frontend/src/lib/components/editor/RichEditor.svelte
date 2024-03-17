<script lang="ts">
	import Button from '$src/lib/components/ui/button/button.svelte';
	import { Bold, Italic, type Icon } from 'lucide-svelte';

	import { Editor } from '@tiptap/core';
	import StarterKit from '@tiptap/starter-kit';

	import { onMount, onDestroy } from 'svelte';

	let editor: Editor;
	let element: HTMLElement;

	onMount(() => {
		editor = new Editor({
			element,
			extensions: [StarterKit],

			onTransaction: () => {
				editor = editor;
				const new_active = new Array(menu_items.length);
				for (let i = 0; i < active.length; i++) {
					new_active[i] = menu_items[i].active();
				}
				active = new_active;
			}
		});
	});

	type ChainedCommands = ReturnType<typeof editor.chain>;
	type Command = (c: ChainedCommands) => ChainedCommands;
	type MenuItem = {
		title: string;
		icon: typeof Bold;
		command: Command;
		active: () => boolean;
	};
	type Dropdown = [];

	const menu_items: MenuItem[] = [
		{
			title: 'Toggle Bold',
			icon: Bold,
			command: (c) => c.toggleBold(),
			active: () => editor.isActive('bold'),
		},
		{
			title: 'Toggle Italic',
			icon: Italic,
			command: (c) => c.toggleItalic(),
			active: () => editor.isActive('italic'),
		}
	];
	let active = new Array(menu_items.length);

	const execute = (command: Command) => {
		command(editor.chain().focus()).run();
	};
</script>

<div class="editor flex h-full w-full flex-col" bind:this={element}>
	<div class="flex gap-0.5 p-0.5 border-b">
		{#each menu_items || active as { title, command, icon }, i}
			<Button
				variant={active[i] ? 'default' : 'outline'}
				size="icon-sm"
				{title}
				on:click={() => execute(command)}
			>
				<svelte:component this={icon} class="h-4 w-4" />
			</Button>
		{/each}
	</div>
</div>

<style lang="postcss">
	:global(.ProseMirror) {
		@apply min-h-0 flex-[1_1_0] overflow-auto p-2;
		@apply prose prose-sm max-w-none;
	}
</style>
