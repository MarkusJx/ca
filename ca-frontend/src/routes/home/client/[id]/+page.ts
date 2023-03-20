import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { getClientById } from '$lib/api/clients/clients';
import { getSigningRequestsByClientId } from '$lib/api/signing-requests/signing-requests';

export const load = (({ params, parent }) => {
  return {
    client: parent().then(async (data) => {
      if (!data.keycloak || !params.id) return [null, null];

      try {
        return await Promise.all([
          getClientById(params.id),
          getSigningRequestsByClientId(params.id),
        ]);
      } catch (e: any) {
        if (e?.response?.status === 401) {
          return [null, null];
        } else if (typeof e?.response?.status === 'number') {
          return error(e.response.status);
        } else {
          return error(500);
        }
      }
    }),
  };
}) satisfies PageLoad;
