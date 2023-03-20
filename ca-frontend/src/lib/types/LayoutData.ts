import type Keycloak from 'keycloak-js';

export default interface LayoutData {
  keycloak: Keycloak | null;
}
