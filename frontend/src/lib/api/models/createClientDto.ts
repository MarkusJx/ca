/**
 * Generated by orval v6.12.1 🍺
 * Do not edit manually.
 * Certificate Authority API
 * A simple API for managing certificates
 * OpenAPI spec version: 0.0.1
 */

export interface CreateClientDto {
	/** The client name
Only required when creating a new client */
	name: string | null;
	validUntil: string;
}