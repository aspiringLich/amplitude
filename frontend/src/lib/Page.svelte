<script lang="ts">
	import { fly } from 'svelte/transition';

	export let center = false;
	let _class: string = '';
	export { _class as class };
</script>

<main class="h-full w-full p-4" class:center>
	<div
		in:fly={{ y: -5, duration: 300, delay: 300 }}
		out:fly={{ y: 5, duration: 300 }}
		class={_class}
	>
		<slot />
	</div>
</main>

<style lang="postcss">
	.center {
		@apply flex items-center justify-center max-h-full;
	}
	
	.center div {
		@apply max-h-full flex flex-col overflow-clip;
	}
	
	.center div > :global(*) {
		@apply flex flex-col shrink max-h-full overflow-scroll max-w-xl;
	}
</style>
