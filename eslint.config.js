import js from "@eslint/js";
import globals from "globals";
import securityPlugin from "eslint-plugin-security";

export default [
  js.configs.recommended,
  {
    plugins: {
      security: securityPlugin,
    },
    rules: {
      ...securityPlugin.configs.recommended.rules,
      "no-unused-vars": "warn",
      "no-console": "off",
    },
    languageOptions: {
      ecmaVersion: "latest",
      sourceType: "module",
      globals: {
        ...globals.browser,
        ...globals.node,
      },
    },
  },
];
