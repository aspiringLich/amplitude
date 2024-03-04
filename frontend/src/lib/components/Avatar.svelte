<script lang="ts">
	import { createAvatar, melt, createSync } from '@melt-ui/svelte';

	export let src: string | undefined;
	export let name: string | undefined;
	export let size: string = 'h-10 w-10';
	export let text: string = 'text-lg';
	export { _class as class };
	let _class: string = '';

	$: split = name?.split(' ');
	$: abbreviation = `${split?.[0][0] || '!'}${split?.length || 0 > 1 ? split?.[1][0] : ''}`;

	const {
		elements: { image, fallback },
		states
	} = createAvatar();

	const sync = createSync(states);
</script>

<div class="avatar flex {size} items-center justify-center rounded-full bg-zinc-500 {_class}">
	<img use:melt={$image} alt="avatar" class="h-full w-full rounded-full" {src} />
	<span use:melt={$fallback} class="overflow-hidden uppercase {text} select-none">
		{abbreviation}
	</span>
</div>
