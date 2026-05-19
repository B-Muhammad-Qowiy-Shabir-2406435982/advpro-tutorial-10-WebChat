const path = require('path');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

const distPath = path.resolve(__dirname, 'dist');

module.exports = {
    mode: 'production',

    experiments: {
        asyncWebAssembly: true,
        syncWebAssembly: true,
    },

    devServer: {
        port: 8000,
    },

    entry: './bootstrap.js',

    output: {
        path: distPath,
        filename: 'yewchat.js',
        webassemblyModuleFilename: 'yewchat_bg.wasm',
    },

    module: {
        rules: [
            {
                test: /\.wasm$/,
                use: "wasm-loader",
                type: "javascript/auto",
            }
        ]
    },

    plugins: [
        new CopyWebpackPlugin({
            patterns: [{ from: './static', to: distPath }],
        }),

        new WasmPackPlugin({
            crateDirectory: '.',
            extraArgs: '-- --features wee_alloc',
            outName: 'yewchat',
        }),
    ],
};