// Source: https://github.com/joshnuss/svelte-local-storage-store, modified to use snake case
// https://github.com/joshnuss/svelte-local-storage-store/blob/master/index.ts
// Represents version v0.4.0 (2023-01-18)

import { BROWSER } from 'esm-env';
import { writable as internal, get, type Writable } from 'svelte/store';

declare type Updater<T> = (value: T) => T;
declare type StoreDict<T> = { [key: string]: Writable<T> };

/* eslint-disable @typescript-eslint/no-explicit-any */
const stores: StoreDict<any> = {};

interface Serializer<T> {
	parse(text: string): T;
	stringify(object: T): string;
}

type StorageType = 'local' | 'session';

interface Options<T> {
	serializer?: Serializer<T>;
	storage?: StorageType;
}

function get_storage(type: StorageType) {
	return type === 'local' ? localStorage : sessionStorage;
}

export function local_store<T>(key: string, initialValue: T, options?: Options<T>): Writable<T> {
	const serializer = options?.serializer ?? JSON;
	const storageType = options?.storage ?? 'local';

	function update(key: string, value: T) {
		if (!BROWSER) return;
		get_storage(storageType).setItem(key, serializer.stringify(value));
	}

	if (!stores[key]) {
		const store = internal(initialValue, (set) => {
			const json = BROWSER ? get_storage(storageType).getItem(key) : null;
			if (json) {
				set(<T>serializer.parse(json));
			}
			if (BROWSER) {
				const handle_storage = (event: StorageEvent) => {
					if (event.key === key) set(event.newValue ? serializer.parse(event.newValue) : null);
				};
				window.addEventListener('storage', handle_storage);
				return () => window.removeEventListener('storage', handle_storage);
			}
		});

		const { subscribe, set } = store;

		stores[key] = {
			set(value: T) {
				update(key, value);
				set(value);
			},
			update(updater: Updater<T>) {
				const value = updater(get(store));

				update(key, value);
				set(value);
			},
			subscribe
		};
	}

	return stores[key];
}
