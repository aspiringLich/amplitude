<script lang="ts" context="module">
	import { type Writable } from 'svelte/store';
	import { local_store } from '$src/lib/local_store';
	import type { ExerciseDraft } from '$src/routes/create/schema';

	export const selected_draft: Writable<number> = local_store('create/selected_draft', -1);
	export const drafts: Writable<ExerciseDraft[]> = local_store('create/drafts', []);
</script>

<script lang="ts">
	import Card from '$cpt/ui/Card.svelte';
	import { Button } from '$cpt/ui/button';
	import * as Table from '$cpt/ui/table';
	import * as Tooltip from '$cpt/ui/tooltip';
	import { Plus, Trash2 } from 'lucide-svelte';

	import Page from '$lib/Page.svelte';
	import { refresh } from '$src/routes/+layout.svelte';

	let _drafts = $drafts;

	const create_draft = () => {
		drafts.update((e) => {
			e.push({ name: 'Untitled Exercise' });
			return e;
		});
		$selected_draft = $drafts.length - 1;
	};

	const delete_draft = (i: number) => {
		drafts.update((e) => {
			e.splice(i, 1);
			return e;
		});
		_drafts = $drafts;
		if (i <= $selected_draft) $selected_draft -= 1;
	};

	const select_draft = (i: number) => {
		$selected_draft = i;
	};
</script>

<Page center>
	<Card class="m-4">
		<header>
			<h1>Exercise Drafts</h1>
		</header>
		<section class="prose">
			<p>
				These drafts are stored locally on your device. When they are published, they will be
				uploaded to your account and become available to other users.
			</p>
			<Table.Root class="my-0 select-none">
				<Table.Header>
					<Table.Row>
						<Table.Head class="flex items-center justify-between">
							Your Drafts
							<Tooltip.Root>
								<a href="/create/edit">
									<Button
										on:click={create_draft}
										class="rounded-full"
										variant="outline"
										size="icon"
										tooltip="Create a new draft"
									>
										<Plus class="h-4 w-4" />
									</Button>
								</a>
							</Tooltip.Root>
						</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each _drafts as ex, i}
						<Table.Row class="hover:bg-muted/50 flex items-center justify-between">
							<a
								class="block w-full no-underline"
								href="/create/edit"
								on:click={() => select_draft(i)}
							>
								<Table.Cell class="relative w-full">
									{ex.name}
								</Table.Cell>
							</a>
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
								<!-- TODO: Skeleton loading -->
							</Table.Cell>
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>
		</section>
	</Card>
</Page>
