<script lang="ts">
	import Button from '$src/lib/components/ui/button/button.svelte';
	import { Bold, Italic, type Icon } from 'lucide-svelte';

	import { EditorState, type Command, Plugin } from 'prosemirror-state';
	import { EditorView } from 'prosemirror-view';
	import { Schema, DOMParser, MarkType, Node } from 'prosemirror-model';
	import { schema } from 'prosemirror-schema-basic';
	import { addListNodes } from 'prosemirror-schema-list';
	import { exampleSetup } from 'prosemirror-example-setup';
	import { toggleMark } from 'prosemirror-commands';

	import { onMount } from 'svelte';
	import 'prosemirror-menu/style/menu.css';

	// Mix the nodes from prosemirror-schema-list into the basic schema to
	// create a schema with list support.
	const mySchema = new Schema({
		nodes: addListNodes(schema.spec.nodes, 'paragraph block*', 'block'),
		marks: schema.spec.marks
	});

	let element: HTMLElement;
	let view: EditorView;

	class Updater {
		update() {
			const new_active = new Array(menu_items.length);
			for (let i = 0; i < active.length; i++) {
				new_active[i] = menu_items[i].active();
			}
			active = new_active;
		}
	}
	const updater = new Plugin({
		view(_) {
			return new Updater();
		}
	});

	onMount(() => {
		view = new EditorView(element, {
			state: EditorState.create({
				doc: DOMParser.fromSchema(mySchema).parse(element),
				plugins: [...exampleSetup({ schema: mySchema, menuBar: false }), updater]
			})
		});
	});

	type MenuItem = {
		title: string;
		command: Command;
		icon: typeof Bold;
		active: () => boolean;
	};
	type Dropdown = [];

	function markActive(type: MarkType) {
		const s = view.state.selection;
		if (s.empty) return !!type.isInSet(view.state.storedMarks || s.$from.marks());
		else return view.state.doc.rangeHasMark(s.from, s.to, type);
	}

	const menu_items: MenuItem[] = [
		{
			title: 'Toggle Bold',
			command: toggleMark(schema.marks.strong),
			icon: Bold,
			active: () => markActive(schema.marks.strong)
		},
		{
			title: 'Toggle Italic',
			command: toggleMark(schema.marks.em),
			icon: Italic,
			active: () => markActive(schema.marks.em)
		}
	];
	let active = new Array(menu_items.length);

	const execute = (cmd: Command) => {
		view.focus();
		cmd(view.state, view.dispatch, view);
	};
</script>

<div class="prosemirror-editor flex h-full w-full flex-col">
	<div class="flex gap-0.5 p-0.5">
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
	<div
		class="prosemirror-wrapper border-input flex h-full w-full flex-col
		overflow-auto border-t"
		bind:this={element}
	/>
</div>

<style lang="postcss">
	:global(.ProseMirror) {
		@apply min-h-0 flex-[1_1_0] overflow-auto p-2;
		@apply prose prose-sm max-w-none;
	}
</style>
