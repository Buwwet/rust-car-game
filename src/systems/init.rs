use std::ops::Range;

use nalgebra::vector;
use specs::{System, Entity, LazyUpdate, Write, Read, Entities};
use js_sys::Math::random;


use crate::{resources::{RigidBodyContainer, ColliderContainer}, entities::{create_floor, create_player, create_ramp, create_ground_mesh}};


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

        /* 
        // Create the floor.
        let floor_pos = vector!(0.0, 0.0, 0.0);
        create_floor(&entities, &lazy, floor_pos, &mut rigidbodies, &mut colliders);
        */

        // Heightmap
        create_ground_mesh(&entities, &lazy, &mut rigidbodies, &mut colliders);

        // Create our player.
        let player_pos = vector!(0.0, 5.0, 0.0);
        create_player(&entities, &lazy, player_pos, &mut rigidbodies, &mut colliders);
    
        // Create ramps.
        for index in 0..12 {
            
            fn get_random(range: Range<f32>, neg: bool) -> f32 {
                // Calculate if sign if we want it.
                let positivity = match neg {
                    true => {
                        if random() > 0.5 {
                            1.0
                        } else {
                            -1.0
                        }
                    }
                    false => {1.0}
                };

                // To apply a range, we multiply our random 0-1 value
                // by the difference of the min and max. Then we add
                // the min.
                let multiply_by = range.end - range.start;

                return random() as f32 * multiply_by * positivity + range.start;
            }

            //TODO: generate number to see if we should make a number negative.

            // Generate a random position.
            let ramp_pos = vector!(
                get_random(10.0..100.0, true),
                0.0,
                get_random(10.0..100.0, true)
            );


            let ramp_rot = vector![
                0.0,
                get_random(0.0..6.28, false),
                0.0
            ];
            create_ramp(
                &entities, 
                &lazy, 
                ramp_pos, 
                ramp_rot,
                &mut rigidbodies, 
                &mut colliders
            );
        };
        // Create a test ramp.
        
        
    
    }
}