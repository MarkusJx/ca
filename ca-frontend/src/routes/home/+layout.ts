import { browser } from '$app/environment';
import { KeycloakAdapter } from '$lib/keycloak';
import type Keycloak from 'keycloak-js';
import type { LayoutLoad } from './$types';

export const load = (async ({ parent }) => {
  const { keycloak } = await parent();
  let keycloakPromise: Promise<Keycloak | null> | null = null;
  if (browser && !keycloak) {
    keycloakPromise = KeycloakAdapter.init(true);
  }

  return {
    keycloak: keycloakPromise || keycloak,
  };
}) satisfies LayoutLoad;
