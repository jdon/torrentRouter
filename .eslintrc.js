module.exports = {
	env: {
		node: true,
		es6: true,
		jest: true
	},
	extends: ['airbnb-typescript/base', 'plugin:prettier/recommended'],
	parserOptions: {
		project: "./tsconfig.json"
	},
	rules:{
		"import/no-extraneous-dependencies": ["warn", { "devDependencies": ["tests/*", "**/*.test.js"] }],
		"@typescript-eslint/camelcase": "off",
		"import/prefer-default-export": "off"
	}
}