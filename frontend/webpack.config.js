const path = require('path');

module.exports = {
    entry: path.join(__dirname, '/src/main.tsx'),
    output: {
        filename: 'build/dist/main.js',
        path: __dirname
    },
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                loader: 'ts-loader',
                // loaders: ['ts-loader', 'wasm-loader'],
                exclude: /node_modules/,
            },
            {
                test: /\.wasm$/,
                loader: 'wasm-loader',
                exclude: /node_modules/,
            },
        ],

    },
    resolve: {
        extensions: [".tsx", ".ts", ".js", ".wasm"]
    },
};