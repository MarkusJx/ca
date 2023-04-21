import type Keycloak from 'keycloak-js';
import { browser } from '$app/environment';
import { KeycloakAdapter } from '$lib/keycloak';
import type { LayoutLoad } from '../../../.svelte-kit/types/src/routes/$types';
import type { HealthInfoDto } from '$lib/api/models';

export const load = (async ({ parent }) => {
	let keycloakPromise: Promise<Keycloak | null> | null = null;
	if (browser) {
		const parentData = (await parent()) as { health: HealthInfoDto };
		if (parentData.health.isInitialized) {
			location.href = '/';
			return {};
		}

		keycloakPromise = KeycloakAdapter.init(true);
	}

	return {
		keycloak: keycloakPromise,
	};
}) satisfies LayoutLoad;
