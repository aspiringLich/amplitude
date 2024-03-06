<script lang="ts">
	import Avatar from '$lib/components/Avatar.svelte';

	export let name: string | undefined = undefined;
	export let src: string | undefined = undefined;
</script>

<a class="flex flex-initial items-center" href="/login">
	<Avatar {src} {name} class="flex-none" />
	<div class="ml-1.5 flex min-w-0 shrink flex-col leading-3">
		<span class="line-clamp-1 select-none" class:italic={!name}>
			{name || 'Not Logged In'}
		</span>
		<span class="line-clamp-1 text-sm italic text-zinc-300">
			{name ? 'Logged In' : 'Click to Log In'}
		</span>
	</div>
</a>
