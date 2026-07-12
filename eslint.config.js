import js from "@eslint/js";
import pluginSecurity from "eslint-plugin-security";
import globals from "globals";

export default [
  js.configs.recommended,
  pluginSecurity.configs.recommended,
  {
    languageOptions: {
      ecmaVersion: "latest",
      sourceType: "module",
      globals: {
        ...globals.browser,
        ...globals.node,
        ...globals.es2021,
      }
    },
    rules: {
      "no-unused-vars": "warn",
    }
  }
];
