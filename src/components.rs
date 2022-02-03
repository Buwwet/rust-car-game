use rapier3d::prelude::{ColliderHandle, RigidBodyHandle};
use specs::{Component, VecStorage, WorldExt, World};
use wasm_bindgen::prelude::wasm_bindgen;

pub fn register_components(world: &mut World) {
    world.register::<PlayerCar>();
    world.register::<PhysicsObject>();

    world.register::<StaticObject>();
    world.register::<ModelName>();
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct StaticObject {
    pub colliders: Vec<ColliderHandle>
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct PhysicsObject {
    pub object_type: PhysicsType,
    pub rigidbody: RigidBodyHandle,
    pub colliders: Vec<ColliderHandle>,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum PhysicsType {
    Static,     // Never update position.
    Dynamic,    // Always update position.
}


#[derive(Component)]
#[storage(VecStorage)]
pub struct PlayerCar {
    pub touching_ground: bool,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct ModelName {
    pub name: [char; 5],
}