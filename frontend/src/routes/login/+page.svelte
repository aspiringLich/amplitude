<script lang="ts" context="module">
	import { PUBLIC_GOOGLE_CLIENT_ID } from '$env/static/public';
	
	type GoogleUser = { client_id: string; credential: string; select_by: string };
	(window as any).login_google = (user: GoogleUser) => {
		console.log(user);
	};
</script>

<svelte:head>
	<script src="https://accounts.google.com/gsi/client" async defer></script>
</svelte:head>

<div class="content">
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
