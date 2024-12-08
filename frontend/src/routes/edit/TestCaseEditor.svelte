<script lang="ts" context="module">
	let id = 0;

	export const get_id = () => {
		return id++;
	};
</script>

<script lang="ts">
	import { flip } from 'svelte/animate';
	import { dndzone } from 'svelte-dnd-action';

	let items: { id: number; inputs: any[]; output: any }[] = [];

	const flipDurationMs = 300;
</script>

<div use:dndzone={{ items, flipDurationMs }}></div>
