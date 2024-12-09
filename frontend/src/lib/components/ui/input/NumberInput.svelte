<script lang="ts">
	import type { HTMLInputAttributes } from 'svelte/elements';
	import Input from './input.svelte';
	import { type FormInputEvent, type InputEvents } from '.';
	import { createEventDispatcher } from 'svelte';
	import { ChangeDesc } from '@codemirror/state';

	type $$Props = HTMLInputAttributes;
	type $$Events = InputEvents;

	const dispatch = createEventDispatcher<{ change: FormInputEvent<Event> }>();

	let className: $$Props['class'] = undefined;
	export let value: number | undefined = undefined;
	export { className as class };

	let str_value: string | undefined;

	const update_str = (value: number | undefined) => {
		str_value = value?.toString();
	};

	$: update_str(value);
</script>

<Input
	type="number"
	class={className}
	bind:value={str_value}
	on:blur
	on:change={(e) => {
		value = str_value ? Number.parseFloat(str_value) : undefined;
		dispatch('change', e);
	}}
	on:click
	on:focus
	on:focusin
	on:focusout
	on:keydown
	on:keypress
	on:keyup
	on:mouseover
	on:mouseenter
	on:mouseleave
	on:mousemove
	on:paste
	on:input
	{...$$restProps}
/>
