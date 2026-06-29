// eslint.config.js — ESLint 9+ flat config for SigmaOS
import js from '@eslint/js';
import security from 'eslint-plugin-security';
import globals from 'globals';

export default [
  js.configs.recommended,
  security.configs.recommended,
  {
    files: ['zenith_desktop/**/*.js'],
    languageOptions: {
      ecmaVersion: 2024,
      sourceType: 'module',
      globals: {
        ...globals.browser,
        ...globals.node,
      },
    },
    rules: {
      // Allow console for kernel-style log output
      'no-console': 'off',
      // Prevent prototype pollution
      'no-prototype-builtins': 'warn',
    },
  },
  {
    // Also lint web_ui scripts
    files: ['web_ui/**/*.js'],
    languageOptions: {
      ecmaVersion: 2024,
      sourceType: 'module',
      globals: {
        ...globals.browser,
      },
    },
  },
  {
    ignores: [
      'node_modules/**',
      'dist/**',
      'build/**',
      'sigma-build/**',
      'iso_root/**',
      '_bld/**',
      '**/third_party/**',
      '**/*.cjs',
    ],
  },
];
