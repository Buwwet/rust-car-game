
use wasm_bindgen::prelude::wasm_bindgen;
use specs::{Dispatcher, DispatcherBuilder, System, World, RunNow, WorldExt};

use crate::GameKeysContainer;

use self::{run_physics::PhysicsSystem, movement::MovementSystem};
// Import our systems and create a
// function out of it

mod run_physics;
mod movement;
pub mod init;

pub fn run_systems(world: &mut World) {
    // We cannot use a Dispatcher in WebAssembly :(
    {  
        // Run Movement System
        let mut ms = MovementSystem {};
        ms.run_now(world);
    }
    { 
        // Run Physics Step System
        let mut ps = PhysicsSystem {};
        ps.run_now(world);
        
    }

    world.maintain();
}
