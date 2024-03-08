<script lang="ts">
	import Card from '$src/lib/components/ui/Card.svelte';
	import { getContext } from 'svelte';
	import { state } from '$src/routes/create/+page.svelte';
	import { Button } from '$src/lib/components/ui/button';
	import { refresh } from '$src/routes/+layout.svelte';
</script>

<Card class="m-4">
	<header>
		<h1>Exercise Drafts</h1>
	</header>
	<section>
		<p>
			These drafts are stored locally, on your device. When they are published, they will be
			uploaded to your account and become available to other users.
		</p>
		<Button on:click={refresh}>refresh</Button>
	</section>
</Card>
