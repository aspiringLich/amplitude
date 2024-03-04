<script lang="ts">
	import '../styles/app.css';

	import Avatar from '$lib/components/Avatar.svelte';

	import { account, logged_in } from './login';
</script>

<div class="flex h-screen w-screen flex-row overflow-hidden">
	<aside
		id="sidebar"
		class="flex min-w-48 max-w-48 flex-col items-stretch
		justify-between bg-zinc-800 p-4 text-white"
	>
		<div class="relative h-20 w-full">
			<div class="absolute flex w-full justify-around">
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
			<div
				class="absolute flex h-full select-none items-center
				justify-center text-3xl font-bold italic w-full text-center"
			>
				amplitude
			</div>
		</div>
		<div class="grow"></div>
		<div class="flex flex-initial items-center">
			<Avatar src={$account.avatar_url} name={$account.name} class="flex-none" />
			<div class="ml-1.5 flex min-w-0 shrink flex-col">
				<span class="text-clamp" class:italic={!$logged_in}>
					{$account.name || 'Not Logged in'}
				</span>
			</div>
		</div>
	</aside>
	<main class="h-full w-full flex justify-center items-center bg-zinc-200">
		<slot />
	</main>
</div>

<style lang="postcss">
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
