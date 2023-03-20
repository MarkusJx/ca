import type { LayoutLoad } from './$types';

export const load = (async ({ parent }) => {
  const { keycloak } = await parent();
  if (
    keycloak &&
    (!keycloak.authenticated || !keycloak.hasRealmRole('admin'))
  ) {
    location.href = '/';
  }
}) satisfies LayoutLoad;
