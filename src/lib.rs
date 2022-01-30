mod utils;
mod components;
mod systems;
mod resources;
mod entities;

use components::{ModelName, PhysicsObject};
use nalgebra::{Vector3, vector};
use rapier3d::prelude::{RigidBodySet, ColliderSet};
use resources::RigidBodyContainer;
use serde::{Serialize};
use specs::{World, Builder, WorldExt, System, RunNow, Join};

use js_sys::{Array, Float32Array, JsString, Object};

use systems::{init::InitSystem};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct GameContainer {
    world: World, //specs
}


#[wasm_bindgen]
impl GameContainer {
    pub fn create() -> GameContainer {
        // The Specs world contains our Resources and Entites.
        let mut world = World::new();

        // Insert the resources
        resources::insert_resources(&mut world);

        // Register the components
        components::register_components(&mut world);

        // Run the setup system to spawn our player and floor.
        let mut is = InitSystem {};
        is.run_now(&mut world);

        world.maintain();
        
        GameContainer {
            world,
        }
    }
    pub fn run_systems(&mut self) {
        systems::run_systems(&mut self.world);
    }
    pub fn log_entities(&self) -> Array {
        // For each entity with PhysicsObject and ModelName, return it to our Javascript.
        let mut object_collection = Array::new();
        
        // Fetch Components
        let names = self.world.read_storage::<ModelName>();
        let physics_objects = self.world.read_storage::<PhysicsObject>();

        // Fetch rigidbodies.
        let rigidbody_set = self.world.read_resource::<RigidBodyContainer>();
        

        // Find all entites with these components.
        for (name, ps_object) in (&names, &physics_objects).join() {
            // Use the object's rigidbody handle to find the rigidbody.
            let rigidbody = rigidbody_set.0.get(ps_object.rigidbody).unwrap();

            // Get pos and rot from the rigidbody.
            let pos: Vector3<f32> = rigidbody.position().translation.vector.xyz();
            let rot = rigidbody.rotation().euler_angles();


            let object: GameObject = GameObject {
                name: name.name.clone(),
                pos: vec![pos[0], pos[1], pos[2]],
                rot: vec![rot.0, rot.1, rot.2],
            };
            
            // Append to the object collection.
            object_collection.push(&JsValue::from_serde::<GameObject>(&object).unwrap_or_else(|err| {
                log(&format!("Error while paring JsValue from GameObject: {}", err));
                std::process::exit(1);
            }));
        }

        // Return the collection to javascript

        object_collection
    }
}

#[wasm_bindgen]
#[derive(Serialize)]
pub struct GameObject {
    name: String,
    pos: Vec<f32>,
    rot: Vec<f32>,
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);
}
