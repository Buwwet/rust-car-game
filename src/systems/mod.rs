
use wasm_bindgen::prelude::wasm_bindgen;
use specs::{Dispatcher, DispatcherBuilder, System, World, RunNow, WorldExt};

use self::run_physics::PhysicsSystem;
// Import our systems and create a
// function out of it

mod run_physics;
pub mod init;

pub fn run_systems(world: &mut World) {
    // We cannot use a Dispatcher in WebAssembly :(
    { 
        // Run Physics Step System
        let mut ps = PhysicsSystem {};
        ps.run_now(world);
        
    }

    world.maintain();
}
