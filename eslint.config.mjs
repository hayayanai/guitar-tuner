// ESLint 9+ flat config でoxlintのルールを自動除外する設定
import vue from 'eslint-plugin-vue';
import oxlint from 'eslint-plugin-oxlint';
// ESLint 9+ では 'eslint/conf/recommended' の直接import不可。推奨ルールは明示的に記述。
import prettier from 'eslint-config-prettier';

import tsParser from '@typescript-eslint/parser';

export default [
    {
        ignores: [
            '**/dist/**',
            '**/node_modules/**',
            '**/src-tauri/**',
        ],
    },
    // TypeScriptファイル用
    {
        files: ['**/*.ts'],
        languageOptions: {
            parser: tsParser,
            parserOptions: {
                sourceType: 'module',
            },
        },
    },
    // oxlintの設定ファイルを参照し、重複ルールを自動でoff
    ...oxlint.buildFromOxlintConfigFile('.oxlintrc.json'),
    // prettier
    prettier,
    // Vueファイル用（parserOptions.parserでTypeScript対応）
    ...vue.configs['flat/recommended'].map((cfg) => {
        if (cfg.languageOptions) {
            return {
                ...cfg,
                languageOptions: {
                    ...cfg.languageOptions,
                    parserOptions: {
                        ...(cfg.languageOptions.parserOptions || {}),
                        parser: '@typescript-eslint/parser',
                    },
                },
            };
        }
        return cfg;
    }),
];
