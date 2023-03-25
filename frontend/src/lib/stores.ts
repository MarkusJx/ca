import { writable } from 'svelte/store';

interface Store {
	sideDrawerOpen: boolean;
	includeInactive: boolean;
}

const defaultStore: Store = {
	sideDrawerOpen: false,
	includeInactive: false,
};

export const sideDrawerOpen = writable(false);
export const includeInactive = writable(false);

let _saveStore = false;

export function loadStores(): void {
	const store = window.sessionStorage.getItem('store');
	if (store) {
		const data: Store = JSON.parse(store);
		sideDrawerOpen.set(data.sideDrawerOpen);
		includeInactive.set(data.includeInactive);
	}
	_saveStore = true;
}

export function saveStore<K extends keyof Store>(
	key: K,
	value: Store[K]
): void {
	if (_saveStore) {
		const prev = window.sessionStorage.getItem('store');
		if (prev) {
			window.sessionStorage.setItem(
				'store',
				JSON.stringify({ ...JSON.parse(prev), [key]: value })
			);
		} else {
			window.sessionStorage.setItem(
				'store',
				JSON.stringify({ ...defaultStore, [key]: value })
			);
		}
	}
}
