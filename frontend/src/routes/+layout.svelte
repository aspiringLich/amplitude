<script>
	import '../app.css';
</script>

<div class="flex h-screen w-screen flex-row">
	<div class="flex min-w-48 flex-col items-stretch bg-zinc-800 p-4 text-white">
		<div class="relative h-20">
			<div class="absolute flex h-full w-full justify-around">
				<div id="wave" class="bg-red-500"></div>
				<div id="wave" class="bg-orange-500"></div>
				<div id="wave" class="bg-yellow-500"></div>
				<div id="wave" class="bg-lime-500"></div>
				<div id="wave" class="bg-green-500"></div>
				<div id="wave" class="bg-cyan-500"></div>
				<div id="wave" class="bg-blue-500"></div>
				<div id="wave" class="bg-indigo-500"></div>
			</div>

			<div
				class="absolute flex h-full w-full select-none items-center
				justify-center text-3xl font-bold italic"
			>
				amplitude
			</div>
		</div>
	</div>
	<main>
		<slot />
	</main>
</div>

<style lang="postcss">
	#wave {
		@apply h-3.5 w-3.5 rounded-full bg-opacity-50;
		animation: wave 5s cubic-bezier(0.37, 0, 0.63, 1) infinite alternate;
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

	@keyframes wave {
		0% {
			transform: translateY(0);
		}
		100% {
			transform: translateY(4.125rem);
		}
	}
</style>