// A dependency graph that contains any wasm must all be imported
// asynchronously. This `bootstrap.js` file does the single async import, so
// that no one else needs to worry about it again.
Promise.resolve().then(function () { return require("./index.js"); }).catch(function (e) { return console.error("Error importing `index.js`:", e); });
