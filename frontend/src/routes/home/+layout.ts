import { browser } from '$app/environment';
import { KeycloakAdapter } from '$lib/keycloak';
import type Keycloak from 'keycloak-js';
import type { LayoutLoad } from './$types';

export const load = (async ({ parent }) => {
	let keycloakPromise: Promise<Keycloak | null> | null = null;
	let keycloak: Keycloak | null = null;
	if (browser) {
		keycloak = (await parent()).keycloak ?? null;

		if (!keycloak) {
			keycloakPromise = KeycloakAdapter.init(true);
		}
	}

	return {
		keycloak: keycloakPromise || keycloak,
	};
}) satisfies LayoutLoad;
