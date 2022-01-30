var CopyWebpackPlugin = require("copy-webpack-plugin");
var path = require('path');
module.exports = {
    entry: "./bootstrap.ts",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bootstrap.js",
    },
    mode: "development",
    plugins: [
        new CopyWebpackPlugin(['index.html'])
    ],
};
