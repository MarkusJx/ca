interface Env {
	VITE_APP_API_BASE_URL: string;
	VITE_KEYCLOAK_URL: string;
	VITE_KEYCLOAK_REALM: string;
	VITE_KEYCLOAK_CLIENT_ID: string;
}

export const env = import.meta.env as unknown as Env;
