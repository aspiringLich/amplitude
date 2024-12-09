<script lang="ts" context="module">
	
</script>

<script lang="ts">
	import { flip } from 'svelte/animate';
	import { dndzone } from 'svelte-dnd-action';

	let items: { id: number; inputs: any[]; output: any }[] = [];

	const flipDurationMs = 300;
</script>

<div use:dndzone={{ items, flipDurationMs }}></div>
