module.exports = {
  ca: {
    output: {
      mode: 'tags-split',
      target: 'src/api/apiClient.ts',
      schemas: 'src/api/models',
      client: 'svelte-query',
      mock: false,
      prettier: true,
      override: {
        mutator: {
          path: './src/api/axios.ts',
          name: 'customInstance',
        },
      },
    },
    input: {
      target: './src/api/schema.json',
    },
  },
};
