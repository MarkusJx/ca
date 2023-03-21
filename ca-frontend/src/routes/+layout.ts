import type Keycloak from 'keycloak-js';
import { browser } from '$app/environment';
import { KeycloakAdapter } from '$lib/keycloak';
import type { LayoutLoad } from './$types';

export const prerender = true;

export const load = (async () => {
	let keycloakPromise: Promise<Keycloak | null> | null = null;
	if (browser) {
		keycloakPromise = KeycloakAdapter.init(false);
	}

	return {
		keycloak: keycloakPromise,
	};
}) satisfies LayoutLoad;
