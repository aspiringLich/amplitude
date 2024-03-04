<script lang="ts">
	import { PUBLIC_GOOGLE_CLIENT_ID } from '$env/static/public';
	import { request } from '$lib/query';
	import { account } from '.';

	type GoogleUser = { client_id: string; credential: string; select_by: string };
	(window as any).login_google = async (user: GoogleUser) => {
		const res = await request.post('/api/auth/google', { credentials: user.credential });
		if (res.ok) {
			const user = await res.json();
			account.set(user);
		}
	};
</script>

<svelte:head>
	<script src="https://accounts.google.com/gsi/client" async defer></script>
</svelte:head>

<div class="content box">
	<div class="login">
		<div
			id="g_id_onload"
			data-client_id={PUBLIC_GOOGLE_CLIENT_ID}
			data-context="signin"
			data-ux_mode="popup"
			data-callback="login_google"
			data-auto_select="true"
			data-itp_support="true"
		/>
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
</div>

<style lang="postcss">
	.login {
		@apply flex h-10 w-full items-center justify-center;
	}
</style>
