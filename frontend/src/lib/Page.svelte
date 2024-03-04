<script lang="ts">
	import { fly } from 'svelte/transition';

	export { _class as class };
	let _class: string = '';
</script>

<div in:fly={{ y: -5, duration: 400, delay: 400 }} out:fly={{ y: 5, duration: 400 }} class={_class}>
	<slot />
</div>
