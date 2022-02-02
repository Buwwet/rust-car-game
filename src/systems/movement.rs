use nalgebra::{vector, Point3, Quaternion, Matrix, Vector3, Rotation2, Rotation, Rotation3, Matrix3, Const, ArrayStorage, Matrix2, point};
use rapier3d::prelude::RigidBody;
use specs::{System, Write, Read, Entities, ReadStorage, WriteStorage, Join};

use crate::{resources::{RigidBodyContainer, ColliderContainer}, GameKeysContainer, components::{PlayerCar, PhysicsObject}, log, GameKeys};



pub struct MovementSystem {}

impl <'a>System<'a> for MovementSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, PlayerCar>,
        WriteStorage<'a, PhysicsObject>,

        Write<'a, RigidBodyContainer>,
        Read<'a, GameKeysContainer>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            player,
            physics_objects,
            mut rigidbody_set,
            keys,
        ) = data;

        // Get the physics_object and of all players
        for (physics_object, _player, _ent) in (&physics_objects, &player, &entities).join() {
            let rigidbody_handle = physics_object.rigidbody;
            
            
            let rigidbody: Option<&mut RigidBody> = rigidbody_set.0.get_mut(rigidbody_handle);
            if rigidbody.is_none() {
                log("None when fetching rigidbody from rigidbody_set.")
            }
            let rigidbody = rigidbody.unwrap();
            
            // Variables to change
            let mut forward_force = vector![0.0, 0.0, 0.0];
            let mut torque = vector![0.0, 0.0, 0.0];
            
            if keys.get(GameKeys::Acceleration as usize) {
                // Slam on the pedal.
                forward_force = vector![160.0, 0.0, 0.0];
            }
            if keys.get(GameKeys::Brakes as usize) {
                // Slam on the reverse.
                forward_force = vector![-80.0, 0.0, 0.0];
            }

            // TODO: Make it so torque's magnitude changes with current speed.
            if keys.get(GameKeys::Left as usize) {
                // Go left.
                torque = vector![0.0, 630.0, 0.0];
            }
            if keys.get(GameKeys::Right as usize) {
                // Go left.
                torque = vector![0.0, -630.0, 0.0];
            }

            /* Do these only when touching the ground */

            // Change the rotation to be relative to where the
            // Car is looking at.
            forward_force = rigidbody.rotation().transform_vector(&forward_force);

            // Apply velocity.
            rigidbody.apply_impulse(forward_force, true);
            rigidbody.apply_torque(torque, true);

            // Simulate


        }
    }
}