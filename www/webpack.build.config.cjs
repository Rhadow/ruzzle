const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
    entry: "./bootstrap.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bootstrap.js",
    },
    mode: "production",
    module: {
      rules: [
        {
          test: /\.js$/,
          include: [
            path.resolve(__dirname, 'node_modules/ruzzle'),
          ],
          use: {
            loader: 'babel-loader',
            options: {
              plugins: ['@babel/plugin-syntax-import-meta']
            }
          }
        }
      ]
    },
    plugins: [
        new CopyWebpackPlugin([
            'index.html',
            'index.css',
            { from: './assets', to: './assets' },
            { from: 'node_modules/ruzzle/ruzzle_bg.wasm', to: 'ruzzle_bg.wasm' }
        ])
    ],
    devServer: {
        port: 3000
    }
};
