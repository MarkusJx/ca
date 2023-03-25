/**
 * Generated by orval v6.12.1 🍺
 * Do not edit manually.
 * Certificate Authority API
 * A simple API for managing certificates
 * OpenAPI spec version: 0.0.1
 */
import { createQuery } from '@tanstack/svelte-query';
import type {
	CreateQueryOptions,
	QueryFunction,
	CreateQueryResult,
	QueryKey,
} from '@tanstack/svelte-query';
import type { HealthInfoDto, ErrorDto } from '.././models';
import { customInstance } from '.././axios';
import type { ErrorType } from '.././axios';

// eslint-disable-next-line
type SecondParameter<T extends (...args: any) => any> = T extends (
	config: any,
	args: infer P
) => any
	? P
	: never;

export const healthCheck = (
	options?: SecondParameter<typeof customInstance>,
	signal?: AbortSignal
) => {
	return customInstance<HealthInfoDto>(
		{ url: `/api/v1/health`, method: 'get', signal },
		options
	);
};

export const getHealthCheckQueryKey = () => [`/api/v1/health`];

export type HealthCheckQueryResult = NonNullable<
	Awaited<ReturnType<typeof healthCheck>>
>;
export type HealthCheckQueryError = ErrorType<ErrorDto>;

export const createHealthCheck = <
	TData = Awaited<ReturnType<typeof healthCheck>>,
	TError = ErrorType<ErrorDto>
>(options?: {
	query?: CreateQueryOptions<
		Awaited<ReturnType<typeof healthCheck>>,
		TError,
		TData
	>;
	request?: SecondParameter<typeof customInstance>;
}): CreateQueryResult<TData, TError> & { queryKey: QueryKey } => {
	const { query: queryOptions, request: requestOptions } = options ?? {};

	const queryKey = queryOptions?.queryKey ?? getHealthCheckQueryKey();

	const queryFn: QueryFunction<Awaited<ReturnType<typeof healthCheck>>> = ({
		signal,
	}) => healthCheck(requestOptions, signal);

	const query = createQuery<
		Awaited<ReturnType<typeof healthCheck>>,
		TError,
		TData
	>({ queryKey, queryFn, ...queryOptions }) as CreateQueryResult<
		TData,
		TError
	> & { queryKey: QueryKey };

	query.queryKey = queryKey;

	return query;
};
