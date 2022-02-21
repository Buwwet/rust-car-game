
use nalgebra::{point, Matrix, DMatrix, dmatrix, vector};
use parry3d::math::{Vector, Real, Rotation, AngVector};
use rapier3d::prelude::{RigidBodyBuilder, ColliderBuilder};
use specs::{Entities, Read, world::EntitiesRes, LazyUpdate, Builder};

use crate::{components::{PlayerCar, PhysicsObject, ModelName, PhysicsType}, resources::{ColliderContainer, RigidBodyContainer}};


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
        .additional_mass(120.0)
        .linear_damping(1.0)
        .angular_damping(0.2)
        .build();
    
    /* Create the colliders (1 for now) */
    let collider = ColliderBuilder::cuboid(4.0, 1.0, 2.0)
        .restitution(0.2)
        .build();
    
    // These are stored in the entity.
    let rigidbody_handle = rigidbodies.0.insert(rigidbody);
    // Remember to insert the collider with the parent.
    let collider_handle = colliders.0.insert_with_parent(collider, rigidbody_handle, &mut rigidbodies.0);

    // Create the specs entity.
    lazy.create_entity(&ent)
        .with(PlayerCar { touching_ground: false })
        .with(ModelName {
            name: ['c', 'a', 'r', '0', '0'],
        })
        .with(PhysicsObject {
            object_type: PhysicsType::Dynamic,
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
        .friction(1.0)
        
        .build();
    
    // These are stored in the entity.
    let rigidbody_handle = rigidbodies.0.insert(rigidbody);
        // Remember to insert the collider with the parent.
        let collider_handle = colliders.0.insert_with_parent(collider, rigidbody_handle, &mut rigidbodies.0);

    // Create the specs entity.
    lazy.create_entity(&ent)
        .with(ModelName {
            name: ['f', 'l', 'o', 'o', 'r'],
        })
        .with(PhysicsObject {
            object_type: PhysicsType::Static,
            rigidbody: rigidbody_handle,
            colliders: vec![collider_handle],
        })
        .build();
}

pub fn create_ramp<'a>(
    // Get the Builders of the entity:
    ent: &Read<'a, EntitiesRes>,
    lazy: &Read<'a, LazyUpdate>,

    // Insert to RigidBodyContainer and ColliderContainer
    pos: Vector<Real>,
    rot: AngVector<Real>,
    rigidbodies: &mut RigidBodyContainer,
    colliders: &mut ColliderContainer,

) {
    // Create the rigidbody and colliders.
    /* Create our rigid body */
    let rigidbody = RigidBodyBuilder::new_static()
        .translation(pos)
        .rotation(rot)
        .build();

    
    /* Create the ramp collider */
    let points = [
        /* Floor */
        point!(5.0, 0.0, 6.0),
        point!(-5.0, 0.0, 6.0),
        point!(5.0, 0.0, -6.0),
        point!(-5.0, 0.0, -6.0),
        /* Top part */
        point!(5.0, 5.0, 6.0),
        point!(-5.0, 5.0, 6.0),
    ];

    let collider = ColliderBuilder::convex_hull(&points)
        .unwrap()
        .build();
    
    // These are stored in the entity.
    let rigidbody_handle = rigidbodies.0.insert(rigidbody);
        // Remember to insert the collider with the parent.
        let collider_handle = colliders.0.insert_with_parent(collider, rigidbody_handle, &mut rigidbodies.0);

    // Create the specs entity.
    lazy.create_entity(&ent)
        .with(ModelName {
            name: ['r', 'a', 'm', 'p', '0'],
        })
        .with(PhysicsObject {
            object_type: PhysicsType::Static,
            rigidbody: rigidbody_handle,
            colliders: vec![collider_handle],
        })
        .build();
}

pub fn create_ground_mesh<'a> (
        // Get the Builders of the entity:
        ent: &Read<'a, EntitiesRes>,
        lazy: &Read<'a, LazyUpdate>,
    
        // Insert to RigidBodyContainer and ColliderContainer
        rigidbodies: &mut RigidBodyContainer,
        colliders: &mut ColliderContainer,
) {
    let rigidbody = RigidBodyBuilder::new_static().build();

    // The heights require a custom dmatrix
    let heights = dmatrix![
        0.0, 0.0, 0.0, 0.0, 0.0;
        0.0, 0.0, 0.0, 0.0, 0.0;
        0.0, 0.0, 0.0, 0.0, 0.0;
        1.0, 1.0, 1.0, 1.0, 1.0;
        1.0, 1.0, 1.0, 1.0, 1.0;
    ];

    // Use the heights to create the heightmap collider
    let collider = ColliderBuilder::heightfield(heights, vector![1000.0, 100.0, 1000.0]).build();

    // Create the handles for the entity.
    let rigidbody_handle = rigidbodies.0.insert(rigidbody);
        // Remember to insert the collider with the parent.
    let collider_handle = colliders.0.insert_with_parent(collider, rigidbody_handle, &mut rigidbodies.0);

    // Create the specs entity.
    lazy.create_entity(&ent)
        .with(ModelName {
            name: ['m', 'a', 'p', '0', '0'],
        })
        .with(PhysicsObject {
            object_type: PhysicsType::Static,
            rigidbody: rigidbody_handle,
            colliders: vec![collider_handle],
        })
        .build();
}