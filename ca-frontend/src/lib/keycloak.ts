import Keycloak from 'keycloak-js';
import { env } from './env';

export class KeycloakAdapter {
  private static kc: Keycloak | null = null;

  public static async init(): Promise<Keycloak | null> {
    if (!this.kc) {
      this.kc = new Keycloak({
        url: env.VITE_KEYCLOAK_URL,
        realm: env.VITE_KEYCLOAK_REALM,
        clientId: env.VITE_KEYCLOAK_CLIENT_ID,
      });
    }

    const auth = await this.kc.init({
      onLoad: 'login-required',
    });

    /*if (!auth) {
      window.location.reload();
    }*/

    return auth ? this.kc : null;
  }

  public static get authenticated(): boolean {
    return this.kc?.authenticated ?? false;
  }

  public static get token(): string | null {
    return this.kc?.token ?? null;
  }
}
