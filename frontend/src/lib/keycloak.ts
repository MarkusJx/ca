import Keycloak from 'keycloak-js';
import { env } from './env';

export class KeycloakAdapter {
	private static kc: Keycloak | null = null;

	public static async init(requireLogin: boolean): Promise<Keycloak | null> {
		if (!this.kc) {
			this.kc = new Keycloak({
				url: env.VITE_KEYCLOAK_URL,
				realm: env.VITE_KEYCLOAK_REALM,
				clientId: env.VITE_KEYCLOAK_CLIENT_ID,
			});
		}

		if (this.kc.authenticated) return this.kc;
		const auth = await this.kc.init({
			onLoad: requireLogin ? 'login-required' : 'check-sso',
			checkLoginIframe: false,
			silentCheckSsoRedirectUri:
				window.location.origin + '/silent-check-sso.html',
		});

		if (!auth && requireLogin) {
			window.location.reload();
		}

		return auth ? this.kc : null;
	}

	public static get authenticated(): boolean {
		return this.kc?.authenticated ?? false;
	}

	public static get token(): string | null {
		return this.kc?.token ?? null;
	}

	public static hasRole(role: string): boolean {
		return this.kc?.hasRealmRole(role) ?? false;
	}

	public static get username(): string | null {
		return this.kc?.idTokenParsed?.preferred_username ?? null;
	}

	public static async logout(): Promise<void> {
		await this.kc?.logout();
	}
}
