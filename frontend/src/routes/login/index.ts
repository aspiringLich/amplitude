import { writable } from 'svelte/store';

export let account = writable({
	name: undefined,
    avatar_url: undefined,
});
export let logged_in = writable(false);

account.subscribe((value) => {
    logged_in.set(value.name !== undefined);
});