<script lang="ts">
	import { fly } from 'svelte/transition';

	export let center = false;
	export { _class as class };
	let _class: string = '';
</script>

<main class="h-full w-full" class:center>
	<div
		in:fly={{ y: -5, duration: 400, delay: 400 }}
		out:fly={{ y: 5, duration: 400 }}
		class={_class}
	>
		<slot />
	</div>
</main>

<style lang="postcss">
	.center {
		@apply flex items-center justify-center;
	}
</style>
