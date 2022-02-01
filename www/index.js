"use strict";
exports.__esModule = true;
var game_test_1 = require("game-test");
var THREE = require("three");
// Better logging on Errors.
(0, game_test_1.set_panic_hook)();
// Create the 3js scene.
var scene = new THREE.Scene();
var camara = new THREE.OrthographicCamera(-3.2 * 3, //Left
3.2 * 3, //Ruight
2.4 * 3, //Top
-2.4 * 3, //Bottom
0.01, //Near
100);
// Move camera to look at center.
camara.position.set(12, 12, 12);
camara.lookAt(0, 0, 0);
// Grid
scene.add(new THREE.GridHelper(10, 10));
var renderer = new THREE.WebGLRenderer();
document.getElementById('root').appendChild(renderer.domElement);
renderer.setSize(640, 480);
// Create a game structure
var game_structure = game_test_1.GameContainer.create();
var renderLoop = function () {
    // Run the game systems.
    game_structure.run_systems();
    // Compared threejs objects with Rust GameObjects
    var gameObjects = game_structure.log_entities();
    // Create meshes if a gameObject without an id is found, update
    // the positions of the others.
    for (var i = 0; i < gameObjects.len(); i++) {
        var gameObject = gameObjects.get(1);
        var entID = gameObject.id();
        var entName = gameObject.name();
        // Check if an object exists with entID and entName
        var object = scene.getObjectByName(entID + entName);
        if (object === undefined) {
            // Create that object!
            var newObject = create_object(entName);
            // Create its identifier.
            newObject.name = entID + entName;
            scene.add(newObject);
            // Set the position of that object.
            update_object(newObject, gameObject);
        }
        else {
            // Check if object is dynamic.
            if (gameObject.physics_type() == game_test_1.PhysicsType.Dynamic) {
                // Update that object!
                update_object(object, gameObject);
            }
        }
    }
    // Render the scene.
    renderer.render(scene, camara);
    // Console log the game objects.
    requestAnimationFrame(renderLoop);
};
function update_object(object, gameObject) {
    // Function to update an objects position and rotation.
    var pos = gameObject.pos();
    object.position.set(pos[0], pos[1], pos[2]);
    var rot = gameObject.rot();
    object.rotation.set(rot[0], rot[1], rot[2]);
}
function create_object(name) {
    // NOTE: these meshes' geometries are just the same values
    // given to the colliders. (but * 2 because those are generated like in a mirror)
    if (name == "Car00") {
        var carObject = new THREE.Mesh(new THREE.BoxGeometry(4, 2, 8), new THREE.MeshNormalMaterial());
        return carObject;
    }
    if (name == "Floor") {
        var floorObject = new THREE.Mesh(new THREE.BoxGeometry(200, 0.2, 200), new THREE.MeshBasicMaterial());
        return floorObject;
    }
    // !!! If nothing matches, it returns undefined so watch out!
}
requestAnimationFrame(renderLoop);
