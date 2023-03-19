import { browser } from '$app/environment';
import { KeycloakAdapter } from '../lib/keycloak';
import type Keycloak from 'keycloak-js';

export const prerender = true;

export const load = async () => {
  let keycloakPromise: Promise<Keycloak | null> | null = null;
  if (browser) {
    keycloakPromise = KeycloakAdapter.init();
  }

  return {
    keycloak: keycloakPromise,
  };
};
