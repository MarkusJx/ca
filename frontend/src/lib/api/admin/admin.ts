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
import type { ErrorDto } from '.././models';
import { customInstance } from '.././axios';
import type { ErrorType } from '.././axios';

// eslint-disable-next-line
type SecondParameter<T extends (...args: any) => any> = T extends (
	config: any,
	args: infer P
) => any
	? P
	: never;

export const listRoles = (
	options?: SecondParameter<typeof customInstance>,
	signal?: AbortSignal
) => {
	return customInstance<string[]>(
		{ url: `/api/v1/admin/roles`, method: 'get', signal },
		options
	);
};

export const getListRolesQueryKey = () => [`/api/v1/admin/roles`];

export type ListRolesQueryResult = NonNullable<
	Awaited<ReturnType<typeof listRoles>>
>;
export type ListRolesQueryError = ErrorType<ErrorDto>;

export const createListRoles = <
	TData = Awaited<ReturnType<typeof listRoles>>,
	TError = ErrorType<ErrorDto>
>(options?: {
	query?: CreateQueryOptions<
		Awaited<ReturnType<typeof listRoles>>,
		TError,
		TData
	>;
	request?: SecondParameter<typeof customInstance>;
}): CreateQueryResult<TData, TError> & { queryKey: QueryKey } => {
	const { query: queryOptions, request: requestOptions } = options ?? {};

	const queryKey = queryOptions?.queryKey ?? getListRolesQueryKey();

	const queryFn: QueryFunction<Awaited<ReturnType<typeof listRoles>>> = ({
		signal,
	}) => listRoles(requestOptions, signal);

	const query = createQuery<
		Awaited<ReturnType<typeof listRoles>>,
		TError,
		TData
	>({ queryKey, queryFn, ...queryOptions }) as CreateQueryResult<
		TData,
		TError
	> & { queryKey: QueryKey };

	query.queryKey = queryKey;

	return query;
};
