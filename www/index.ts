import {GameContainer, set_panic_hook} from "game-test";
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
    let gameObjects: Array<any> = game_structure.log_entities();
    
    // Create meshes if a gameObject without an id is found, update
    // the positions of the others.
    for (var i = 0; i < gameObjects.length; i++) {
        let entID = gameObjects[i].id;
        let entName = gameObjects[i].name;
        // Check if an object exists with entID and entName
        let object = scene.getObjectByName(entID + entName);

        if (object === undefined) {
            // Create that object!
            let newObject = create_object(entName);
            
            newObject.name = entID + entName;
            scene.add(newObject);

        } else {
            // Update that object!
            let pos: Array<number> = gameObjects[i].pos;
            object.position.set(pos[0], pos[1], pos[2]);

            let rot: Array<number> = gameObjects[i].rot;
            object.rotation.set(rot[0], rot[1], rot[2]);
        }
    }

    // Render the scene.
    renderer.render(scene, camara);
    // Console log the game objects.
    

    requestAnimationFrame(renderLoop);
}

function create_object(name: string) {
    // NOTE: these meshes' geometries are just the same values
    // given to the colliders. (but * 2 because those are generated like in a mirror)
    if (name == "Car") {
        let carObject = new THREE.Mesh(
            new THREE.BoxGeometry(4, 2, 8),
            new THREE.MeshNormalMaterial()
        );
        return carObject;
    }
    if (name == "Floor") {
        let floorObject = new THREE.Mesh(
            new THREE.BoxGeometry(200, 0.2, 200),
            new THREE.MeshBasicMaterial()
        );
        return floorObject;
    }
}



requestAnimationFrame(renderLoop);