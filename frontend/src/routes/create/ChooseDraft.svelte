<script lang="ts">
	import Card from '$cpt/ui/Card.svelte';
	import { Button } from '$cpt/ui/button';
	import * as Table from '$cpt/ui/table';
	import * as Tooltip from '$cpt/ui/tooltip';

	import { state, ex_drafts } from '$src/routes/create/+page.svelte';
	import { refresh } from '$src/routes/+layout.svelte';
	import { Plus } from 'lucide-svelte';
</script>

<Card class="m-4">
	<header>
		<h1>Exercise Drafts</h1>
	</header>
	<section class="prose">
		<p>
			These drafts are stored locally on your device. When they are published, they will be uploaded
			to your account and become available to other users.
		</p>
		<Table.Root class="my-0 select-none">
			<Table.Header>
				<Table.Row>
					<Table.Head class="flex items-center justify-between">
						Your Drafts
						<Tooltip.Root>
							<Tooltip.Trigger asChild let:builder>
								<Button builders={[builder]} class="rounded-full" variant="outline" size="icon">
									<Plus class="h-4 w-4" />
								</Button>
							</Tooltip.Trigger>
							<Tooltip.Content>Create a new draft</Tooltip.Content>
						</Tooltip.Root>
					</Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#each $ex_drafts as ex}
					<Table.Row>
						<Table.Cell>{ex.name}</Table.Cell>
					</Table.Row>
				{/each}
				{#if $ex_drafts.length === 0}
					<Table.Row>
						<Table.Cell class="italic" colspan={2}>No drafts found</Table.Cell>
					</Table.Row>
				{/if}
			</Table.Body>
		</Table.Root>
	</section>
</Card>
