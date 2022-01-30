"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var game_test_1 = require("game-test");
// Create a game structure
var game_structure = game_test_1.GameContainer.create();
var renderLoop = function () {
    // Run the game systems.
    game_structure.run_systems();
    requestAnimationFrame(renderLoop);
};
requestAnimationFrame(renderLoop);
