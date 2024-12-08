<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { buttonVariants } from '$src/lib/components/ui/button';
	import { Info } from 'lucide-svelte';
</script>

<Dialog.Root>
	<Dialog.Trigger class={buttonVariants({ variant: 'outline', size: 'icon' })}>
		<Info class="h-4 w-4" />
	</Dialog.Trigger>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Generators and You</Dialog.Title>
			<Dialog.Description>
				<p>
					Aside from manually specifying test cases, you have the option of algorithmically
					generating them with a generator function.
				</p>
				<p>blah blah blah yap yap yap</p>
			</Dialog.Description>
		</Dialog.Header>
	</Dialog.Content>
</Dialog.Root>
