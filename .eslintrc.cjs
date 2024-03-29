module.exports = {
	env: {
		browser: true,
		es2021: true
	},
	root: true,
	extends: ["eslint:recommended", "plugin:svelte/recommended", "plugin:@typescript-eslint/recommended"],
	parser: "@typescript-eslint/parser",
	parserOptions: {
		ecmaVersion: "latest",
		project: "./tsconfig.json",
		extraFileExtensions: [".svelte"]
	},
	rules: {
		"no-unused-vars": "off"
	},
	overrides: [
		{
			files: ["*.svelte"],
			parser: "svelte-eslint-parser",
			parserOptions: {
				parser: "@typescript-eslint/parser"
			}
		}
	]
};
