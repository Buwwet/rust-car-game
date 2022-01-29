mod utils;
mod components;
mod systems;

use rapier3d::prelude::{RigidBodySet, ColliderSet};
use specs::{World, Builder, WorldExt, System, RunNow};

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
        world.insert::<RigidBodySet>(RigidBodySet::new());
        world.insert::<ColliderSet>(ColliderSet::new());

        // Register the components
        components::register_components(&mut world);

        // Dispatcher contains all of the Systems, here we'll set it up, 
        //later we'll need to use
        // dispatcher.dispatch(&mut world). 
        let mut dispatcher = systems::system_dispatcher();
        dispatcher.setup(&mut world);
        
        GameContainer {
            world,
        }
    }
    pub fn run_systems(&mut self) {
        // We would have stored the dispatcher
        // inside of the GameContainer but lifetimes
        // aren't supported by wasm.
        let mut dispatcher = systems::system_dispatcher();
        dispatcher.dispatch(&self.world);

    }
}

