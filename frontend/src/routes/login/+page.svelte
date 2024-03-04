<script lang="ts">
	import { PUBLIC_GOOGLE_CLIENT_ID } from '$env/static/public';
	import { request } from '$lib/query';
	import { account, logged_in } from '.';
	import Page from '$lib/Page.svelte';

	import { goto, afterNavigate } from '$app/navigation';
	import { base } from '$app/paths';

	let previousPage: string = base;
	afterNavigate(({ from }) => {
		previousPage = from?.url.pathname || previousPage;
	});

	type GoogleUser = { client_id: string; credential: string; select_by: string };
	(window as any).login_google = async (user: GoogleUser) => {
		const res = await request.post('/api/auth/google', { credentials: user.credential });
		if (res.ok) {
			const user = await res.json();
			account.set(user);
			goto(previousPage);
		}
		// TODO: error
	};
</script>

<svelte:head>
	<script src="https://accounts.google.com/gsi/client" async defer></script>
</svelte:head>

<Page class="content box">
	<span class="title">Log In to Amplitude</span>
	<div class="login">
		{#if !$logged_in}
			<div
				id="g_id_onload"
				data-client_id={PUBLIC_GOOGLE_CLIENT_ID}
				data-context="signin"
				data-ux_mode="popup"
				data-callback="login_google"
				data-auto_select="true"
				data-itp_support="true"
			/>
		{/if}
		<div
			class="g_id_signin"
			data-type="standard"
			data-shape="pill"
			data-theme="filled_black"
			data-text="signin_with"
			data-size="large"
			data-logo_alignment="left"
		/>
	</div>
</Page>

<style lang="postcss">
	.login {
		@apply flex h-10 w-full items-center justify-center;
	}
</style>
