import {GameContainer, set_panic_hook, GameObjectContainer, PhysicsType} from "game-test";
import * as THREE from 'three';
import { GameObject } from '../pkg/game_test';

// Better logging on Errors.
set_panic_hook();

// Create the 3js scene.
const scene = new THREE.Scene();
const camara = new THREE.OrthographicCamera(
    -3.2 * 3, //Left
    3.2 * 3,  //Ruight
    2.4 * 3,  //Top
    -2.4 * 3, //Bottom
    0.01, //Near
    100,  //Far
);
// Move camera to look at center.
camara.position.set(12, 12, 12);
camara.lookAt(0, 0, 0);

// Grid
scene.add(new THREE.GridHelper(10, 10));

var renderer = new THREE.WebGLRenderer();
document.getElementById('root').appendChild(renderer.domElement);
renderer.setSize(640, 480);


// Create a game structure
let game_structure: GameContainer = GameContainer.create();

const renderLoop = () => {
    // Run the game systems.
    game_structure.run_systems();

    // Compared threejs objects with Rust GameObjects
    let gameObjects: GameObjectContainer = game_structure.log_entities();
    
    // Create meshes if a gameObject without an id is found, update
    // the positions of the others.
    for (var i = 0; i < gameObjects.len(); i++) {
        let gameObject = gameObjects.get(i);
        
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
            new THREE.BoxGeometry(4, 2, 8),
            new THREE.MeshNormalMaterial()
        );
        return carObject;
    }
    if (name == "floor") {
        let floorObject = new THREE.Mesh(
            new THREE.BoxGeometry(200, 0.2, 200),
            new THREE.MeshBasicMaterial()
        );
        return floorObject;
    }

    // !!! If nothing matches, it returns undefined so watch out!
    throw new Error("Failed to create an object with " + name + " as the model name.");
}



requestAnimationFrame(renderLoop);