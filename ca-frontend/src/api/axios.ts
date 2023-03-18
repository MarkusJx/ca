import Axios from 'axios';
import type { AxiosError, AxiosRequestConfig } from 'axios';
import { env } from '../lib/env';
import { KeycloakAdapter } from '../lib/keycloak';

export const AXIOS_INSTANCE = Axios.create({
  baseURL: env.VITE_APP_API_BASE_URL,
}); // use your own URL here or environment variable

// add a second `options` argument here if you want to pass extra options to each generated query
export const customInstance = <T>(
  config: AxiosRequestConfig,
  options?: AxiosRequestConfig
): Promise<T> => {
  const source = Axios.CancelToken.source();
  const promise = AXIOS_INSTANCE({
    ...config,
    ...options,
    headers: {
      Authorization: KeycloakAdapter.authenticated
        ? `Bearer ${KeycloakAdapter.token}`
        : undefined,
      ...config.headers,
      ...options?.headers,
    },
    cancelToken: source.token,
  }).then(({ data }) => data);

  // @ts-ignore
  promise.cancel = () => {
    source.cancel('Query was cancelled');
  };

  return promise;
};

// In some case with react-query and swr you want to be able to override the return error type so you can also do it here like this
export type ErrorType<Error> = AxiosError<Error>;
