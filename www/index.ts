import {GameContainer, set_panic_hook, GameObjectContainer, PhysicsType, GameKeys, GameKeysContainer} from "game-test";
import * as THREE from 'three';
import { ConvexGeometry } from 'three/examples/jsm/geometries/ConvexGeometry'

import { GameObject } from '../pkg/game_test';


// Better logging on Errors.
set_panic_hook();

// Create the 3js scene.
const scene = new THREE.Scene();
const camara = new THREE.OrthographicCamera(
    -3.2 * 30, //Left
    3.2 * 30,  //Ruight
    2.4 * 30,  //Top
    -2.4 * 30, //Bottom
    0.01, //Near
    400,  //Far
);
// Move camera to look at center.
camara.position.set(-100, 100, -100);
camara.lookAt(0, 0, 0);

// Grid
scene.add(new THREE.GridHelper(10, 10));
const light = new THREE.AmbientLight(0xFFFFFF, 1);
scene.add(light);


var renderer = new THREE.WebGLRenderer();
document.getElementById('root').appendChild(renderer.domElement);
renderer.setSize(window.innerWidth, window.innerHeight);


// Create a game structure
let game_structure: GameContainer = GameContainer.create();
// Store keys
let keys_pressed: GameKeysContainer = GameKeysContainer.new();


const renderLoop = () => {
    // Run the game systems.
    game_structure.run_systems(keys_pressed);

    // Compared threejs objects with Rust GameObjects
    let gameObjects: GameObjectContainer = game_structure.log_entities();
    
    // Create meshes if a gameObject without an id is found, update
    // the positions of the others.
    for (var i = 0; i < gameObjects.len(); i++) {
        let gameObject = catch_gameObject(gameObjects, i);
        
        let entID = gameObject.id();
        let entName = gameObject.name();

        // Check if an object exists with entID and entName
        let object = scene.getObjectByName(entID + entName);

        if (object === undefined) {
            // Create that object!
            var newObject = create_object(entName);
            
            // Create its identifier.
            newObject.name = entID + entName;

            // Set the position of that object.
            update_object(newObject, gameObject);

            scene.add(newObject);

            console.log(entName + " built!")
        } else {
            
            // Check if object is dynamic.
            if (gameObject.physics_type() == PhysicsType.Dynamic) {
                // Update that object!
                update_object(object, gameObject);
            }
            
        }
    }

    // Render the scene.
    renderer.render(scene, camara);
    // Console log the game objects.
    

    requestAnimationFrame(renderLoop);
}

function catch_gameObject(gameObjects: GameObjectContainer, idx: number): GameObject {
    // Use the GameObjectContainer's .get() inside a catch.
    try {
        return gameObjects.get(idx);
    } catch(err) {
        throw new Error("Failed to get gameObject. " + err);
    }

}

function update_object(object: THREE.Object3D, gameObject: GameObject) {
    // Function to update an objects position and rotation.

    let pos: Array<number> = gameObject.pos();
    object.position.set(pos[0], pos[1], pos[2]);

    let rot: Array<number> = gameObject.rot();
    object.rotation.set(rot[0], rot[1], rot[2]);
}

function create_object(name: string) {
    // NOTE: these meshes' geometries are just the same values
    // given to the colliders. (but * 2 because those are generated like in a mirror)
    if (name == "car00") {
        let carObject = new THREE.Mesh(
            new THREE.BoxGeometry(8, 2, 4),
            new THREE.MeshNormalMaterial()
        );
        return carObject;
    }
    if (name == "floor") {
        let floorObject = new THREE.Mesh(
            new THREE.BoxGeometry(200, 0.2, 200),
            new THREE.MeshBasicMaterial()
        );

        //floorObject.material.wireframe = true;

        return floorObject;
    }
    if (name == "ramp0") {
        let points = [
        /* Floor */
        new THREE.Vector3(5.0, 0.0, 6.0),
        new THREE.Vector3(-5.0, 0.0, 6.0),
        new THREE.Vector3(5.0, 0.0, -6.0),
        new THREE.Vector3(-5.0, 0.0, -6.0),
        /* Top part */
        new THREE.Vector3(5.0, 5.0, 6.0),
        new THREE.Vector3(-5.0, 5.0, 6.0),
        ];


        let rampObject = new THREE.Mesh(
            new ConvexGeometry(points),
            new THREE.MeshStandardMaterial({
                color: 0xFF00FF,
            }),
        );
        
        return rampObject;
    }

    // !!! If nothing matches, it returns undefined so watch out!
    throw new Error("Failed to create an object with " + name + " as the model name.");
}

// Update keys pressed
document.onkeydown = (e) => {
    var key = e.key;
    switch(key)
    {
        // Depending on the key pressed, toggle it.
        case "w":
            keys_pressed.set(GameKeys.Acceleration, true);
            break;
        case "s":
            keys_pressed.set(GameKeys.Brakes, true);
            break;
        case "a":
            keys_pressed.set(GameKeys.Left, true);
            break;
        case "d":
            keys_pressed.set(GameKeys.Right, true);
            break;
    }
}
document.onkeyup = (e) => {
    var key = e.key;
    switch(key)
    {
        // Depending on the key withheld, toggle it.
        case "w":
            keys_pressed.set(GameKeys.Acceleration, false);
            break;
        case "s":
            keys_pressed.set(GameKeys.Brakes, false);
            break;
        case "a":
            keys_pressed.set(GameKeys.Left, false);
            break;
        case "d":
            keys_pressed.set(GameKeys.Right, false);
            break;
    }
}



/* IOS Controls */
// Listen to see if the user has touched the screen
window.addEventListener('touchstart', function onFirstTouch() {
    // Make the buttons visible.
    let container = document.getElementById("ios_buttons");
    container.style.display = "block"

    // Create the functionality for all of the buttons.
    const ios_forward = document.getElementById("ios_forward");
    ios_forward.ontouchstart = () => {
        keys_pressed.set(GameKeys.Acceleration, true);
    }
    ios_forward.ontouchend = () => {
        keys_pressed.set(GameKeys.Acceleration, false);
    }
    const ios_left = document.getElementById("ios_left");
    ios_left.ontouchstart = () => {
        keys_pressed.set(GameKeys.Left, true);
    }
    ios_left.ontouchend = () => {
        keys_pressed.set(GameKeys.Left, false);
    }
    const ios_right = document.getElementById("ios_right");
    ios_right.ontouchstart = () => {
        keys_pressed.set(GameKeys.Right, true);
    }
    ios_right.ontouchend = () => {
        keys_pressed.set(GameKeys.Right, false);
    }
    const ios_brakes = document.getElementById("ios_brakes");
    ios_brakes.ontouchstart = () => {
        keys_pressed.set(GameKeys.Brakes, true);
    }
    ios_brakes.ontouchend = () => {
        keys_pressed.set(GameKeys.Brakes, false);
    }
    window.removeEventListener('touchstart', onFirstTouch, false);
})


requestAnimationFrame(renderLoop);