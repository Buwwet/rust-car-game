const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');


module.exports = {
  entry: "./bootstrap.ts",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: path.resolve(__dirname, "index.html") },
        { from: path.resolve(__dirname, "style.css") },
        { from: path.resolve(__dirname, "./resources"), to: "resources" },
      ]
    })
  ],
  // CRITICAL
  experiments: {
    //asyncWebAssembly: true,
    syncWebAssembly: true,
  },
};
