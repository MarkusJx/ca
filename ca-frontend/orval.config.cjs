module.exports = {
	ca: {
		output: {
			mode: 'tags-split',
			target: 'src/lib/api/apiClient.ts',
			schemas: 'src/lib/api/models',
			client: 'svelte-query',
			mock: false,
			prettier: true,
			override: {
				mutator: {
					path: './src/lib/api/axios.ts',
					name: 'customInstance',
				},
			},
		},
		input: {
			target: './src/lib/api/schema.json',
		},
	},
};
