<script>
	import '../app.css';
</script>

<div class="flex min-h-screen flex-row">
	<div class="flex min-w-48 flex-col items-stretch bg-zinc-800 p-4 text-white">
		<div class="relative">
			<div class="relative flex h-full w-full select-none items-center text-3xl font-bold">
				<span>amplitude</span>
			</div>
		</div>
	</div>
	<main>
		<slot />
	</main>
</div>
