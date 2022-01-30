use rapier3d::prelude::{ColliderHandle, RigidBodyHandle};
use specs::{Component, VecStorage, WorldExt, World};

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
    pub rigidbody: RigidBodyHandle,
    pub colliders: Vec<ColliderHandle>,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct PlayerCar {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct ModelName {
    pub name: String,
}