import { writable } from 'svelte/store';

export type AccountStore = {
    name: string | undefined;
    avatar_url: string | undefined;
};
export let account = writable<AccountStore>({
	name: undefined,
	avatar_url: undefined
});
export let logged_in = writable(false);

account.subscribe((value) => {
	logged_in.set(value.name !== undefined);
});
