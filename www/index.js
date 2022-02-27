"use strict";
exports.__esModule = true;
var game_test_1 = require("game-test");
var THREE = require("three");
var ConvexGeometry_1 = require("three/examples/jsm/geometries/ConvexGeometry");
// Better logging on Errors.
(0, game_test_1.set_panic_hook)();
var CAMERA_DISTANCE = 500;
var CAMERA_WIDTH = 40;
// Create the 3js scene.
var scene = new THREE.Scene();
var camara = new THREE.OrthographicCamera(-3.2 * CAMERA_WIDTH, //Left
3.2 * CAMERA_WIDTH, //Ruight
2.4 * CAMERA_WIDTH, //Top
-2.4 * CAMERA_WIDTH, //Bottom
0.01, //Near
5000);
// Move camera to look at center.
camara.position.set(-100, 100, -100);
camara.lookAt(0, 0, 0);
// Grid
scene.add(new THREE.GridHelper(10, 10));
var light = new THREE.HemisphereLight(0x8CC8D2, 0x5A6A70, 1);
scene.add(light);
var renderer = new THREE.WebGLRenderer();
document.getElementById('root').appendChild(renderer.domElement);
renderer.setSize(window.innerWidth, window.innerHeight);
// Create the game structure
var game_structure = game_test_1.GameContainer.create();
var map_heightmap = [
    [0.0, 0.3, 0.3, 0.3, 0.0],
    [0.0, 0.3, 0.3, 0.3, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0],
];
// Create the map with the heightmap
load_map(map_heightmap);
// Store keys
var keys_pressed = game_test_1.GameKeysContainer["new"]();
// Debug value for logging stuff on a key press.
var debug_value;
var renderLoop = function () {
    // Run the game systems.
    game_structure.run_systems(keys_pressed);
    // Compared threejs objects with Rust GameObjects
    var gameObjects = game_structure.log_entities();
    // Create meshes if a gameObject without an id is found, update
    // the positions of the others.
    for (var i = 0; i < gameObjects.len(); i++) {
        var gameObject = catch_gameObject(gameObjects, i);
        var entID = gameObject.id();
        var entName = gameObject.name();
        // Check if an object exists with entID and entName
        var object = scene.getObjectByName(entID + entName);
        if (object === undefined && entName != "map00") {
            // Create that object!
            var newObject = create_object(entName);
            // Create its identifier.
            newObject.name = entID + entName;
            // Set the position of that object.
            update_object(newObject, gameObject);
            scene.add(newObject);
            console.log(entID + entName + " built!");
        }
        else {
            // Check if object is dynamic.
            if (gameObject.physics_type() == game_test_1.PhysicsType.Dynamic) {
                // Update that object!
                update_object(object, gameObject);
                // Check if this gameObject is our Player
                if (gameObject.name() == "car00") {
                    // Update the camara's position to ours
                    // but with it's offset.
                    camara.position.setX(object.position.x + -CAMERA_DISTANCE);
                    camara.position.setY(object.position.y + CAMERA_DISTANCE);
                    camara.position.setZ(object.position.z + -CAMERA_DISTANCE);
                    camara.lookAt(object.position);
                }
            }
        }
    }
    // Render the scene.
    renderer.render(scene, camara);
    // Console log the game objects.
    requestAnimationFrame(renderLoop);
};
function catch_gameObject(gameObjects, idx) {
    // Use the GameObjectContainer's .get() inside a catch.
    try {
        return gameObjects.get(idx);
    }
    catch (err) {
        throw new Error("Failed to get gameObject. " + err);
    }
}
function update_object(object, gameObject) {
    // Function to update an objects position and rotation.
    var pos = gameObject.pos();
    object.position.set(pos[0], pos[1], pos[2]);
    var rot = gameObject.rot();
    object.rotation.set(rot[0], rot[1], rot[2]);
}
function load_map(heightmap) {
    // Delete previous map and add a new one.
    var prevoius_map = scene.getObjectByName("map");
    if (prevoius_map !== undefined) {
        scene.remove(prevoius_map);
    }
    // Create the entity and collider in the World.
    game_structure.create_map(heightmap);
    // Create the threejs object from points.
    // Define the boundaries of the map in "game units".
    var map_width = 1000;
    var map_height = 1000;
    // Values of the height map are multiplied by these game units.
    var map_depth_factor = 100;
    var geometry = new THREE.BufferGeometry();
    var mesh;
    // Define how the vertices are connected.
    var indices = [];
    // Position and number of vertices.
    var vertices = [];
    var normals = [];
    // Handles how textures are applied. Example of a plane: https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Faws1.discourse-cdn.com%2Fstandard17%2Fuploads%2Fthreejs%2Foptimized%2F2X%2F6%2F6f3b6cca517d1840321a3bd84bd4eccd01ffa60f_2_1000x1000.jpeg&f=1&nofb=1
    var uv = [];
    /* Here we define the values that we will use for the creation of the new geometry. */
    var size = map_width;
    var segments = heightmap.length - 1;
    var halfSize = size / 2;
    var segmentSize = size / segments;
    // Generate vertices, normals and uv data for a simple grid geometry
    for (var i = 0; i <= segments; i++) {
        var z = (i * segmentSize) - halfSize;
        for (var j = 0; j <= segments; j++) {
            var x = (j * segmentSize) - halfSize;
            vertices.push(x, heightmap[i][j] * map_depth_factor, z);
            normals.push(0, 0, 1);
            // Generate the UV indexes for the vertecies
            var x_factor = i / segments;
            var y_factor = j / segments;
            uv.push(x_factor, y_factor);
        }
    }
    // generate indices (data for element array buffer)
    for (var i = 0; i < segments; i++) {
        for (var j = 0; j < segments; j++) {
            var a = i * (segments + 1) + (j + 1);
            var b = i * (segments + 1) + j;
            var c = (i + 1) * (segments + 1) + j;
            var d = (i + 1) * (segments + 1) + (j + 1);
            // generate two faces (triangles) per iteration
            indices.push(a, b, d); // face one
            indices.push(b, c, d); // face two
        }
    }
    // Apply all of the generated variables to the geometry and index and attributes.
    geometry.setIndex(indices);
    geometry.setAttribute('position', new THREE.Float32BufferAttribute(vertices, 3));
    geometry.setAttribute('normal', new THREE.Float32BufferAttribute(normals, 3));
    geometry.setAttribute('uv', new THREE.Float32BufferAttribute(uv, 2));
    // Create material with texture
    var loader = new THREE.TextureLoader();
    loader.load('resources/textures/texture.jpeg', function (texture) {
        // Wait for the texture to be loaded and then apply the changes.
        texture.wrapS = THREE.RepeatWrapping;
        texture.wrapT = THREE.RepeatWrapping;
        texture.needsUpdate = true;
        // Set the repeating constants
        var imageRepeats = 50;
        texture.repeat.set(imageRepeats, imageRepeats);
        var material = new THREE.MeshBasicMaterial({
            map: texture
        });
        // Create the mesh out of points and add it to
        // the scene
        mesh = new THREE.Mesh(geometry, material);
        mesh.name = "map";
        mesh.rotateY(90 * Math.PI / 180);
        mesh.scale.multiply(new THREE.Vector3(-1, 1, 1));
        // Return the map!
        scene.add(mesh);
    });
}
function create_object(name) {
    // NOTE: these meshes' geometries are just the same values
    // given to the colliders. (but * 2 because those are generated like in a mirror)
    if (name == "car00") {
        var carObject = new THREE.Mesh(new THREE.BoxGeometry(8, 2, 4), new THREE.MeshNormalMaterial());
        //carObject.rotation.order = "XYZ";
        return carObject;
    }
    if (name == "floor") {
        var floorObject = new THREE.Mesh(new THREE.BoxGeometry(200, 0.2, 200), new THREE.MeshBasicMaterial());
        //floorObject.material.wireframe = true;
        return floorObject;
    }
    if (name == "ramp0") {
        var points = [
            /* Floor */
            new THREE.Vector3(5.0, 0.0, 6.0),
            new THREE.Vector3(-5.0, 0.0, 6.0),
            new THREE.Vector3(5.0, 0.0, -6.0),
            new THREE.Vector3(-5.0, 0.0, -6.0),
            /* Top part */
            new THREE.Vector3(5.0, 5.0, 6.0),
            new THREE.Vector3(-5.0, 5.0, 6.0),
        ];
        var rampObject = new THREE.Mesh(new ConvexGeometry_1.ConvexGeometry(points), new THREE.MeshStandardMaterial({
            color: 0xFF00FF
        }));
        return rampObject;
    }
    if (name == "map00") {
        // TEMPORARY
        var floorObject = new THREE.Mesh(new THREE.BoxGeometry(200, 0.2, 200), new THREE.MeshBasicMaterial());
        return floorObject;
    }
    // !!! If nothing matches, it returns undefined so watch out!
    throw new Error("Failed to create an object with " + name + " as the model name.");
}
// Update keys pressed
document.onkeydown = function (e) {
    var key = e.key;
    switch (key) {
        // Depending on the key pressed, toggle it.
        case "w":
            keys_pressed.set(game_test_1.GameKeys.Acceleration, true);
            break;
        case "s":
            keys_pressed.set(game_test_1.GameKeys.Brakes, true);
            break;
        case "a":
            keys_pressed.set(game_test_1.GameKeys.Left, true);
            break;
        case "d":
            keys_pressed.set(game_test_1.GameKeys.Right, true);
            break;
    }
};
document.onkeyup = function (e) {
    var key = e.key;
    switch (key) {
        // Depending on the key withheld, toggle it.
        case "w":
            keys_pressed.set(game_test_1.GameKeys.Acceleration, false);
            break;
        case "s":
            keys_pressed.set(game_test_1.GameKeys.Brakes, false);
            break;
        case "a":
            keys_pressed.set(game_test_1.GameKeys.Left, false);
            break;
        case "d":
            keys_pressed.set(game_test_1.GameKeys.Right, false);
            break;
        case "t":
            console.log(debug_value);
            break;
    }
};
/* IOS Controls */
// Listen to see if the user has touched the screen
window.addEventListener('touchstart', function onFirstTouch() {
    // Make the buttons visible.
    var container = document.getElementById("ios_buttons");
    container.style.display = "block";
    // Create the functionality for all of the buttons.
    var ios_forward = document.getElementById("ios_forward");
    ios_forward.ontouchstart = function () {
        keys_pressed.set(game_test_1.GameKeys.Acceleration, true);
    };
    ios_forward.ontouchend = function () {
        keys_pressed.set(game_test_1.GameKeys.Acceleration, false);
    };
    var ios_left = document.getElementById("ios_left");
    ios_left.ontouchstart = function () {
        keys_pressed.set(game_test_1.GameKeys.Left, true);
    };
    ios_left.ontouchend = function () {
        keys_pressed.set(game_test_1.GameKeys.Left, false);
    };
    var ios_right = document.getElementById("ios_right");
    ios_right.ontouchstart = function () {
        keys_pressed.set(game_test_1.GameKeys.Right, true);
    };
    ios_right.ontouchend = function () {
        keys_pressed.set(game_test_1.GameKeys.Right, false);
    };
    var ios_brakes = document.getElementById("ios_brakes");
    ios_brakes.ontouchstart = function () {
        keys_pressed.set(game_test_1.GameKeys.Brakes, true);
    };
    ios_brakes.ontouchend = function () {
        keys_pressed.set(game_test_1.GameKeys.Brakes, false);
    };
    window.removeEventListener('touchstart', onFirstTouch, false);
});
requestAnimationFrame(renderLoop);
