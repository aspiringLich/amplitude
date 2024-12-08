<script lang="ts" context="module">
	import { type Writable } from 'svelte/store';
	import { local_store } from '$src/lib/local_store';
	import type { Exercise } from '$src/routes/edit/schema';

	export const selected_draft: Writable<number> = local_store('create/selected_draft', -1);
	export type ExerciseDraft = Exercise & {
		selected_field?: string;
		generator?: string;
		generator_lang?: string;
		generate_cases?: number;
	};
	export const drafts: Writable<ExerciseDraft[]> = local_store('create/drafts', []);
</script>

<script lang="ts">
	import { Button } from '$cpt/ui/button';
	import * as Table from '$cpt/ui/table';
	import * as Tooltip from '$cpt/ui/tooltip';
	import { Plus, Trash2 } from 'lucide-svelte';

	import Page from '$lib/Page.svelte';
	import { fly } from 'svelte/transition';
	import { flip } from 'svelte/animate';

	let _drafts = $drafts;

	const create_draft = () => {
		drafts.update((e) => {
			e.push({
				title: 'Untitled Exercise',
				description: '',
				function_name: '',
				input: [],
				output: '',
				generated_table: [],
				starting_code: '',
				generator: undefined,
				generator_lang: undefined,
				generate_cases: 10
			});
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
	<div class="card m-4">
		<header>
			<h1>Exercise Drafts</h1>
			<p>
				These drafts are stored locally on your device. When they are published, they will be
				uploaded to your account and become available to other users.
			</p>
		</header>
		<section class="flex-pass">
			<Table.Root class="flex-pass my-0 select-none items-stretch">
				<Table.Header>
					<Table.Row class="hover:bg-background block">
						<Table.Head class="flex items-center justify-between">
							Your Drafts
							<Tooltip.Root>
								<a href="/edit">
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
				<Table.Body class="flex-pass max-h-96 overflow-scroll">
					{#each _drafts as ex, i}
						<span out:fly={{ y: -50, duration: 200 }} >
							<Table.Row class="hover:bg-muted/50 flex items-center justify-between">
								<a class="block w-full no-underline" href="/edit" on:click={() => select_draft(i)}>
									<Table.Cell class="relative w-full">
										{ex?.title}
									</Table.Cell>
								</a>
								<Button
									on:click={(e) => {
										e.stopImmediatePropagation();
										delete_draft(i);
									}}
									class="mr-6"
									variant="line-destructive"
									size="icon-xs"
								>
									<Trash2 class="h-4 w-4" />
								</Button>
							</Table.Row>
						</span>
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
	</div>
</Page>
