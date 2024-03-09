<script lang="ts">
	import Card from '$cpt/ui/Card.svelte';
	import { Button } from '$cpt/ui/button';
	import * as Table from '$cpt/ui/table';
	import * as Tooltip from '$cpt/ui/tooltip';

	import { state, drafts } from '$src/routes/create/+page.svelte';
	import { Plus, Trash2 } from 'lucide-svelte';

	let _drafts = $drafts;

	const create_draft = () => {
		drafts.update((e) => {
			e.push({ name: 'Untitled Exercise' });
			return e;
		});
		$state.open = $drafts.length - 1;
	};

	const delete_draft = (i: number) => {
		drafts.update((e) => {
			e.splice(i, 1);
			return e;
		});
		_drafts = $drafts;
	};

	const select_draft = (i: number) => {
		$state.open = i;
	};
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
							<Button
								on:click={create_draft}
								class="rounded-full"
								variant="outline"
								size="icon"
								tooltip="Create a new draft"
							>
								<Plus class="h-4 w-4" />
							</Button>
						</Tooltip.Root>
					</Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#each _drafts as ex, i}
					<Table.Row
						on:click={() => select_draft(i)}
						class="hover:bg-muted/50 flex items-center justify-between"
					>
						<Table.Cell class="relative">
							{ex.name}
						</Table.Cell>
						<Button
							on:click={(e) => {
								e.stopImmediatePropagation();
								delete_draft(i);
							}}
							class="mr-4"
							variant="line-destructive"
							size="icon-sm"
						>
							<Trash2 class="h-4 w-4" />
						</Button>
					</Table.Row>
				{:else}
					<Table.Row>
						<Table.Cell class="text-muted-foreground italic" colspan={2}>
							No drafts found
						</Table.Cell>
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
	</section>
</Card>
