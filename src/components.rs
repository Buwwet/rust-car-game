use rapier3d::prelude::{ColliderHandle, RigidBodyHandle};
use shred::World;
use specs::{Component, VecStorage, WorldExt};

pub fn register_components(world: &mut World) {
    world.register::<PlayerCar>();
    world.register::<DynamicObject>();

    world.register::<StaticObject>();
    world.register::<ModelName>();
}

#[derive(Component)]
#[storage(VecStorage)]
struct StaticObject {
    pub colliders: Vec<ColliderHandle>
}

#[derive(Component)]
#[storage(VecStorage)]
struct DynamicObject {
    pub rigidbody: RigidBodyHandle,
    pub colliders: Vec<ColliderHandle>,
}

#[derive(Component)]
#[storage(VecStorage)]
struct PlayerCar {}

#[derive(Component)]
#[storage(VecStorage)]
struct ModelName {
    pub name: String,
}