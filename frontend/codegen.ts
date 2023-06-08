import { CodegenConfig } from "@graphql-codegen/cli";

/*
    Run `npm run compile` or `npm run watch` to regenerate type definitions
*/

const config: CodegenConfig = {
    schema: 'http://localhost:8080/data',
    documents: ['src/**/*.tsx'],
    generates: {
        './src/__generated__/': {
            preset: 'client',
            plugins: [],
            presetConfig: {
                gqlTagName: 'gql',
            }
        }
    },
    ignoreNoDocuments: true,
}

export default config;