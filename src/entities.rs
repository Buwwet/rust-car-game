
use parry3d::math::{Vector, Real};
use rapier3d::prelude::{RigidBodyBuilder, ColliderBuilder};
use specs::{Entities, Read, world::EntitiesRes, LazyUpdate, Builder};

use crate::{components::{PlayerCar, PhysicsObject, ModelName}, resources::{ColliderContainer, RigidBodyContainer}};


// Create entity from Read<Lazy> and Entities

pub fn create_player<'a>(
    // Get the Builders of the entity:
    ent: &Read<'a, EntitiesRes>,
    lazy: &Read<'a, LazyUpdate>,

    // Insert to RigidBodyContainer and ColliderContainer
    pos: Vector<Real>,
    rigidbodies: &mut RigidBodyContainer,
    colliders: &mut ColliderContainer,

) {
    // Create the rigidbody and colliders.
    /* Create our rigid body */
    let rigidbody = RigidBodyBuilder::new_dynamic()
        .translation(pos)
        .build();
    
    /* Create the colliders (1 for now) */
    let collider = ColliderBuilder::cuboid(2.0, 1.0, 4.0)
        .restitution(0.7)
        .build();
    
    // These are stored in the entity.
    let rigidbody_handle = rigidbodies.0.insert(rigidbody);
    // Remember to insert the collider with the parent.
    let collider_handle = colliders.0.insert_with_parent(collider, rigidbody_handle, &mut rigidbodies.0);

    // Create the specs entity.
    lazy.create_entity(&ent)
        .with(PlayerCar {})
        .with(ModelName {
            name: "Car".to_string(),
        })
        .with(PhysicsObject {
            rigidbody: rigidbody_handle,
            colliders: vec![collider_handle],
        })
        .build();
}

pub fn create_floor<'a>(
    // Get the Builders of the entity:
    ent: &Read<'a, EntitiesRes>,
    lazy: &Read<'a, LazyUpdate>,

    // Insert to RigidBodyContainer and ColliderContainer
    pos: Vector<Real>,
    rigidbodies: &mut RigidBodyContainer,
    colliders: &mut ColliderContainer,

) {
    // Create the rigidbody and colliders.
    /* Create our rigid body */
    let rigidbody = RigidBodyBuilder::new_static()
        .translation(pos)
        .build();

    
    /* Create the floor collider */
    let collider = ColliderBuilder::cuboid(100.0, 0.1, 100.0)
        .build();
    
    // These are stored in the entity.
    let rigidbody_handle = rigidbodies.0.insert(rigidbody);
        // Remember to insert the collider with the parent.
        let collider_handle = colliders.0.insert_with_parent(collider, rigidbody_handle, &mut rigidbodies.0);

    // Create the specs entity.
    lazy.create_entity(&ent)
        .with(ModelName {
            name: "Floor".to_string(),
        })
        .with(PhysicsObject {
            rigidbody: rigidbody_handle,
            colliders: vec![collider_handle],
        })
        .build();
}