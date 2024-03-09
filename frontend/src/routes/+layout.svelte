<script lang="ts" context="module">
	let root_element: HTMLElement;
	
	export const refresh = () => {
		root_element.dispatchEvent(new Event('refresh'));
	};
</script>

<script lang="ts">
	import '../styles/app.css';

	export let data;

	import { account, logged_in } from './login';
	import { onMount } from 'svelte';
	import AvatarSection from './AvatarSection.svelte';
	import { writable } from 'svelte/store';

	let notify = writable<Event | undefined>(undefined);
	onMount(async () => {
		if (!$logged_in && data.avatar) {
			$account = data.avatar;
		}
		root_element.addEventListener('refresh', (e) => {
			notify.set(e);
		});
	});

	const sidebar = ['create'];
	$: root = data.url.pathname.split('/')[1];
</script>

<div class="flex h-screen w-screen flex-row overflow-hidden" bind:this={root_element}>
	<aside
		class="flex min-w-52 max-w-52 flex-col items-stretch justify-between
		space-y-6 bg-zinc-800 py-4 text-white"
	>
		<div class="relative box-border h-20 w-full">
			<div class="absolute box-border flex w-full justify-around px-4">
				<div id="wave" />
				<div id="wave" />
				<div id="wave" />
				<div id="wave" />
				<div id="wave" />
				<div id="wave" />
				<div id="wave" />
				<div id="wave" />
				<div id="wave" />
				<div id="wave" />
				<div />
			</div>
			<a
				class="sidebar absolute flex h-full w-full select-none
				items-center justify-center text-center text-3xl font-bold italic"
				href="/"
			>
				amplitude
			</a>
		</div>
		<nav class="sidebar grow px-2">
			{#each sidebar as item}
				<a href="/{item}" class="nav pr-2" class:highlight={root === item}>
					{item}
				</a>
			{/each}
		</nav>
		{#if data.avatar}
			<AvatarSection name={data.avatar.name} src={data.avatar.avatar_url} />
		{:else}
			<AvatarSection name={$account.name} src={$account.avatar_url} />
		{/if}
	</aside>
	<div class="h-full w-full bg-zinc-200">
		{#key data.url || $notify}
			<slot />
		{/key}
	</div>
</div>

<style lang="postcss">
	.nav.highlight {
		@apply bg-zinc-700 text-white;
	}

	.nav {
		@apply block rounded-full;
		@apply text-muted text-right text-2xl font-extrabold capitalize italic;
		@apply transition-colors;
	}

	.nav:hover {
		@apply text-white;
	}

	.sidebar {
		font-family: 'Inter', sans-serif;
	}

	#wave {
		@apply h-3.5 w-3.5 rounded-full bg-opacity-50;
		animation:
			wave 5s cubic-bezier(0.37, 0, 0.63, 1) infinite alternate,
			color 10s infinite,
			z-shift 10s infinite;
	}

	#wave:nth-child(2) {
		animation-delay: -1s;
	}
	#wave:nth-child(3) {
		animation-delay: -2s;
	}
	#wave:nth-child(4) {
		animation-delay: -3s;
	}
	#wave:nth-child(5) {
		animation-delay: -4s;
	}
	#wave:nth-child(6) {
		animation-delay: -5s;
	}
	#wave:nth-child(7) {
		animation-delay: -6s;
	}
	#wave:nth-child(8) {
		animation-delay: -7s;
	}
	#wave:nth-child(9) {
		animation-delay: -8s;
	}
	#wave:nth-child(10) {
		animation-delay: -9s;
	}

	@keyframes wave {
		0% {
			transform: translateY(0);
		}
		100% {
			transform: translateY(4.125rem);
		}
	}

	@keyframes z-shift {
		0% 20% {
			z-index: 1;
		}
		20%,
		80% {
			z-index: auto;
		}
		80%,
		100% {
			z-index: 1;
		}
	}

	@keyframes color {
		0% {
			background-color: theme('colors.red.500');
			opacity: 80%;
		}
		10% {
			background-color: theme('colors.orange.500');
			opacity: 80%;
		}
		20% {
			background-color: theme('colors.yellow.500');
			opacity: 80%;
		}
		30% {
			background-color: theme('colors.lime.500');
			opacity: 80%;
		}
		40% {
			background-color: theme('colors.green.500');
			opacity: 80%;
		}
		50% {
			background-color: theme('colors.cyan.500');
			opacity: 80%;
		}
		60% {
			background-color: theme('colors.blue.500');
			opacity: 80%;
		}
		70% {
			background-color: theme('colors.indigo.500');
			opacity: 80%;
		}
		80% {
			background-color: theme('colors.purple.500');
			opacity: 80%;
		}
		90% {
			background-color: theme('colors.fuchsia.500');
			opacity: 80%;
		}
		100% {
			background-color: theme('colors.red.500');
			opacity: 80%;
		}
	}
</style>
