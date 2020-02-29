const scss = {
	module: {
		rules: [
			// this will apply to both plain `.scss` files
			// AND `<style lang="scss">` blocks in `.vue` files
			{
				test: /\.scss$/,
				use: [
					'vue-style-loader',
					'css-loader',
					'sass-loader'
				]
			}
		]
	},
};

const cssloader = {
	module: {
		rules: [
			{
				test: /\.css$/i,
				use: ['style-loader', 'css-loader'],
			},
		],
	},
};

module.exports = {
	configureWebpack: {
		plugins: [
			scss,
			cssloader
		]
	}
};