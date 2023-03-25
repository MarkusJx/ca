/**
 * Generated by orval v6.12.1 🍺
 * Do not edit manually.
 * Certificate Authority API
 * A simple API for managing certificates
 * OpenAPI spec version: 0.0.1
 */
import { createQuery, createMutation } from '@tanstack/svelte-query';
import type {
	CreateQueryOptions,
	CreateMutationOptions,
	QueryFunction,
	MutationFunction,
	CreateQueryResult,
	QueryKey,
} from '@tanstack/svelte-query';
import type {
	CACertificateDto,
	ErrorDto,
	GenerateIntermediateDto,
	SigningRequestDto,
	NewSigningRequestDto,
} from '.././models';
import { customInstance } from '.././axios';
import type { ErrorType } from '.././axios';

// eslint-disable-next-line
type SecondParameter<T extends (...args: any) => any> = T extends (
	config: any,
	args: infer P
) => any
	? P
	: never;

/**
 * Get the CA's intermediate certificate
This is the certificate that is used to sign the client certificates
 * @summary Get the CA's intermediate certificate
 */
export const getCaCertificate = (
	options?: SecondParameter<typeof customInstance>,
	signal?: AbortSignal
) => {
	return customInstance<CACertificateDto>(
		{ url: `/api/v1/certificate/intermediate`, method: 'get', signal },
		options
	);
};

export const getGetCaCertificateQueryKey = () => [
	`/api/v1/certificate/intermediate`,
];

export type GetCaCertificateQueryResult = NonNullable<
	Awaited<ReturnType<typeof getCaCertificate>>
>;
export type GetCaCertificateQueryError = ErrorType<void | ErrorDto>;

export const createGetCaCertificate = <
	TData = Awaited<ReturnType<typeof getCaCertificate>>,
	TError = ErrorType<void | ErrorDto>
>(options?: {
	query?: CreateQueryOptions<
		Awaited<ReturnType<typeof getCaCertificate>>,
		TError,
		TData
	>;
	request?: SecondParameter<typeof customInstance>;
}): CreateQueryResult<TData, TError> & { queryKey: QueryKey } => {
	const { query: queryOptions, request: requestOptions } = options ?? {};

	const queryKey = queryOptions?.queryKey ?? getGetCaCertificateQueryKey();

	const queryFn: QueryFunction<
		Awaited<ReturnType<typeof getCaCertificate>>
	> = ({ signal }) => getCaCertificate(requestOptions, signal);

	const query = createQuery<
		Awaited<ReturnType<typeof getCaCertificate>>,
		TError,
		TData
	>({ queryKey, queryFn, ...queryOptions }) as CreateQueryResult<
		TData,
		TError
	> & { queryKey: QueryKey };

	query.queryKey = queryKey;

	return query;
};

export const generateIntermediate = (
	generateIntermediateDto: GenerateIntermediateDto,
	options?: SecondParameter<typeof customInstance>
) => {
	return customInstance<CACertificateDto>(
		{
			url: `/api/v1/certificate/intermediate/generate`,
			method: 'post',
			headers: { 'Content-Type': 'application/json' },
			data: generateIntermediateDto,
		},
		options
	);
};

export type GenerateIntermediateMutationResult = NonNullable<
	Awaited<ReturnType<typeof generateIntermediate>>
>;
export type GenerateIntermediateMutationBody = GenerateIntermediateDto;
export type GenerateIntermediateMutationError = ErrorType<ErrorDto>;

export const createGenerateIntermediate = <
	TError = ErrorType<ErrorDto>,
	TContext = unknown
>(options?: {
	mutation?: CreateMutationOptions<
		Awaited<ReturnType<typeof generateIntermediate>>,
		TError,
		{ data: GenerateIntermediateDto },
		TContext
	>;
	request?: SecondParameter<typeof customInstance>;
}) => {
	const { mutation: mutationOptions, request: requestOptions } = options ?? {};

	const mutationFn: MutationFunction<
		Awaited<ReturnType<typeof generateIntermediate>>,
		{ data: GenerateIntermediateDto }
	> = (props) => {
		const { data } = props ?? {};

		return generateIntermediate(data, requestOptions);
	};

	return createMutation<
		Awaited<ReturnType<typeof generateIntermediate>>,
		TError,
		{ data: GenerateIntermediateDto },
		TContext
	>(mutationFn, mutationOptions);
};
/**
 * Get the root CA certificate
Only returns the public key as the private key isn't stored
on the server
 * @summary Get the root CA certificate
 */
export const getRootCertificate = (
	options?: SecondParameter<typeof customInstance>,
	signal?: AbortSignal
) => {
	return customInstance<CACertificateDto>(
		{ url: `/api/v1/certificate/root`, method: 'get', signal },
		options
	);
};

export const getGetRootCertificateQueryKey = () => [`/api/v1/certificate/root`];

export type GetRootCertificateQueryResult = NonNullable<
	Awaited<ReturnType<typeof getRootCertificate>>
>;
export type GetRootCertificateQueryError = ErrorType<void | ErrorDto>;

export const createGetRootCertificate = <
	TData = Awaited<ReturnType<typeof getRootCertificate>>,
	TError = ErrorType<void | ErrorDto>
>(options?: {
	query?: CreateQueryOptions<
		Awaited<ReturnType<typeof getRootCertificate>>,
		TError,
		TData
	>;
	request?: SecondParameter<typeof customInstance>;
}): CreateQueryResult<TData, TError> & { queryKey: QueryKey } => {
	const { query: queryOptions, request: requestOptions } = options ?? {};

	const queryKey = queryOptions?.queryKey ?? getGetRootCertificateQueryKey();

	const queryFn: QueryFunction<
		Awaited<ReturnType<typeof getRootCertificate>>
	> = ({ signal }) => getRootCertificate(requestOptions, signal);

	const query = createQuery<
		Awaited<ReturnType<typeof getRootCertificate>>,
		TError,
		TData
	>({ queryKey, queryFn, ...queryOptions }) as CreateQueryResult<
		TData,
		TError
	> & { queryKey: QueryKey };

	query.queryKey = queryKey;

	return query;
};

/**
 * Generate a new root certificate
This will invalidate the old root certificate
and all certificates signed by it (not yet implemented)
 * @summary Generate a new root certificate
 */
export const generateRootCertificate = (
	options?: SecondParameter<typeof customInstance>
) => {
	return customInstance<CACertificateDto>(
		{ url: `/api/v1/certificate/root/generate`, method: 'post' },
		options
	);
};

export type GenerateRootCertificateMutationResult = NonNullable<
	Awaited<ReturnType<typeof generateRootCertificate>>
>;

export type GenerateRootCertificateMutationError = ErrorType<ErrorDto>;

export const createGenerateRootCertificate = <
	TError = ErrorType<ErrorDto>,
	TVariables = void,
	TContext = unknown
>(options?: {
	mutation?: CreateMutationOptions<
		Awaited<ReturnType<typeof generateRootCertificate>>,
		TError,
		TVariables,
		TContext
	>;
	request?: SecondParameter<typeof customInstance>;
}) => {
	const { mutation: mutationOptions, request: requestOptions } = options ?? {};

	const mutationFn: MutationFunction<
		Awaited<ReturnType<typeof generateRootCertificate>>,
		TVariables
	> = () => {
		return generateRootCertificate(requestOptions);
	};

	return createMutation<
		Awaited<ReturnType<typeof generateRootCertificate>>,
		TError,
		TVariables,
		TContext
	>(mutationFn, mutationOptions);
};
/**
 * Sign a certificate signing request
using the server's CA certificate
 * @summary Sign a certificate signing request
 */
export const signCertificate = (
	newSigningRequestDto: NewSigningRequestDto,
	options?: SecondParameter<typeof customInstance>
) => {
	return customInstance<SigningRequestDto>(
		{
			url: `/api/v1/certificate/sign`,
			method: 'post',
			headers: { 'Content-Type': 'application/json' },
			data: newSigningRequestDto,
		},
		options
	);
};

export type SignCertificateMutationResult = NonNullable<
	Awaited<ReturnType<typeof signCertificate>>
>;
export type SignCertificateMutationBody = NewSigningRequestDto;
export type SignCertificateMutationError = ErrorType<ErrorDto>;

export const createSignCertificate = <
	TError = ErrorType<ErrorDto>,
	TContext = unknown
>(options?: {
	mutation?: CreateMutationOptions<
		Awaited<ReturnType<typeof signCertificate>>,
		TError,
		{ data: NewSigningRequestDto },
		TContext
	>;
	request?: SecondParameter<typeof customInstance>;
}) => {
	const { mutation: mutationOptions, request: requestOptions } = options ?? {};

	const mutationFn: MutationFunction<
		Awaited<ReturnType<typeof signCertificate>>,
		{ data: NewSigningRequestDto }
	> = (props) => {
		const { data } = props ?? {};

		return signCertificate(data, requestOptions);
	};

	return createMutation<
		Awaited<ReturnType<typeof signCertificate>>,
		TError,
		{ data: NewSigningRequestDto },
		TContext
	>(mutationFn, mutationOptions);
};