mod utils;
mod components;
mod systems;
mod resources;
mod entities;

use components::{ModelName, PhysicsObject, PhysicsType};
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

    pub fn log_entities(&self) -> GameObjectContainer {
        // For each entity with PhysicsObject and ModelName, return it to our Javascript inside this container.
        let mut gameobject_container = GameObjectContainer::default();
        
        // Fetch Components
        let names = self.world.read_storage::<ModelName>();
        let physics_objects = self.world.read_storage::<PhysicsObject>();
        let entities = self.world.entities();

        // Fetch rigidbodies.
        let rigidbody_set = self.world.read_resource::<RigidBodyContainer>();
        

        // Find all entites with these components.
        for (name, ps_object, entity) in (&names, &physics_objects, &entities).join() {
            // Use the object's rigidbody handle to find the rigidbody.
            let rigidbody = rigidbody_set.0.get(ps_object.rigidbody).unwrap();

            // Get pos and rot from the rigidbody.
            let pos: Vector3<f32> = rigidbody.position().translation.vector.xyz();
            let rot = rigidbody.rotation().euler_angles();

            // Form the Object
            let object: GameObject = GameObject {
                name: name.name,
                physics: ps_object.object_type.clone(),
                id: entity.id(),
                pos: [pos[0], pos[1], pos[2]],
                rot: [rot.0, rot.1, rot.2],
            };
            
            // Append to the object collection.
            gameobject_container.push(object);
        }

        // Return the collection to javascript

        gameobject_container
    }
}



#[wasm_bindgen]
#[derive(Default, Debug)]
pub struct GameObjectContainer {
    data: [Option<GameObject>; 32],
    length: usize,
}

#[wasm_bindgen]
impl GameObjectContainer {
    pub fn len(&self) -> u32 {
        // Get length of container for looping
        self.length as u32
    }
    pub fn get(&self, idx: usize) -> GameObject {
        // Get GameObject from list.
        //log(&format!("{:#?}", self.data[idx]));
        self.data[idx].unwrap()
    }
    pub fn push(&mut self, object: GameObject) {
        self.data[self.length] = Some(object);
        self.length += 1;
        //log(&format!("{:#?}", self.data));
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct GameObject {
    name: [char; 5],
    physics: PhysicsType,
    id: u32,
    pos: [f32; 3],
    rot: [f32; 3],
}

// Implement getter fuctions.
#[wasm_bindgen]
impl GameObject {
    pub fn name(&self) -> JsString {
        // transform chars into string
        let string: String = self.name.iter().collect();
        string.into()
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn physics_type(&self) -> PhysicsType {
        self.physics
    }
    pub fn pos(&self) -> Array {
        let pos_array = Array::new_with_length(3);
        // The array shouldn't have more than 3 items.
        for index in 0..=2 {
            pos_array.set(index as u32, self.pos[index].into());
        };
        pos_array
    }
    pub fn rot(&self) -> Array {
        let rot_array = Array::new_with_length(3);
        // The array shouldn't have more than 3 items.
        for index in 0..=2 {
            rot_array.set(index as u32, self.rot[index].into());
        };
        rot_array
    }
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);
}
