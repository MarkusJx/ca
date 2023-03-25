import type Keycloak from 'keycloak-js';
import { browser } from '$app/environment';
import { KeycloakAdapter } from '$lib/keycloak';
import type { LayoutLoad } from './$types';
import { healthCheck } from '$lib/api/common/common';
import type { HealthInfoDto } from '$lib/api/models';

export const prerender = true;

export const load = (async ({ url }) => {
	let keycloakPromise: Promise<Keycloak | null> | null = null;
	let health: HealthInfoDto | null = null;
	if (browser) {
		keycloakPromise = KeycloakAdapter.init(false);

		health = await healthCheck();
		if (!health.isInitialized && !url.pathname.startsWith('/setup')) {
			location.href = '/setup';
			return {};
		}
	}

	return {
		keycloak: keycloakPromise,
		health,
	};
}) satisfies LayoutLoad;
