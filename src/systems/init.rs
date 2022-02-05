use nalgebra::vector;
use specs::{System, Entity, LazyUpdate, Write, Read, Entities};

use crate::{resources::{RigidBodyContainer, ColliderContainer}, entities::{create_floor, create_player, create_ramp}};


// Create player and floor at game start.
pub struct InitSystem {}
impl <'a>System<'a> for InitSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,

        // Phyisics
        Write<'a, RigidBodyContainer>,
        Write<'a, ColliderContainer>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, lazy, mut rigidbodies, mut colliders) = data;

        // Create the floor.
        let floor_pos = vector!(0.0, 0.0, 0.0);
        create_floor(&entities, &lazy, floor_pos, &mut rigidbodies, &mut colliders);
    
        // Create our player.
        let player_pos = vector!(0.0, 5.0, 0.0);
        create_player(&entities, &lazy, player_pos, &mut rigidbodies, &mut colliders);
    
    
        // Create a test ramp.
        let ramp_pos = vector!(0.0, 0.0, 10.0);
        create_ramp(&entities, &lazy, ramp_pos, &mut rigidbodies, &mut colliders);
    
    }
}