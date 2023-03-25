import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { getClientById } from '$lib/api/clients/clients';
import { getSigningRequestsByClientId } from '$lib/api/signing-requests/signing-requests';
import { AxiosError } from 'axios';

export const load = (({ params, parent }) => {
	return {
		client: parent().then(async (data) => {
			if (!data.keycloak || !params.id) return [null, null];

			try {
				const client = await getClientById(params.id, {
					includeInactive: true,
				});
				const signingRequests = client.active
					? await getSigningRequestsByClientId(params.id)
					: [];

				return [client, signingRequests];
			} catch (e: any) {
				if (e instanceof AxiosError) {
					if (e.response?.status === 401) {
						return [null, null];
					} else {
						throw error(e.response?.status || 500);
					}
				} else {
					throw error(500);
				}
			}
		}),
	};
}) satisfies PageLoad;
