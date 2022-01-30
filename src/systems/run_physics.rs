use rapier3d::{prelude::{JointSet, ColliderSet, RigidBodySet, IntegrationParameters, PhysicsPipeline, IslandManager, BroadPhase, NarrowPhase, CCDSolver}};
use specs::{System, Write, Join};

use nalgebra::{Vector3, vector};
use crate::resources::{ColliderContainer, RigidBodyContainer, PhysicsResource};

pub struct PhysicsSystem {}
impl <'a>System<'a> for PhysicsSystem {
    type SystemData = (
        Write<'a, RigidBodyContainer>,
        Write<'a, ColliderContainer>,
        Write<'a, PhysicsResource>
    );
    fn run(&mut self, data: Self::SystemData) {
        // DO PHYSICS
        let (
            mut rigidbodies, 
            mut colliders,
            mut physics_structures
        ) = data;

        // Run the simulation with the physics_structure's tick.
        physics_structures.step(&mut rigidbodies.0, &mut colliders.0);

        /*
        /* Create other structures necessary for the simulation. */
        let gravity = vector![0.0, -9.822, 0.0];
        let integration_parameters = IntegrationParameters::default();
        let mut physics_pipeline = PhysicsPipeline::new();
        let mut island_manager = IslandManager::new();
        let mut broad_phase = BroadPhase::new();
        let mut narrow_phase = NarrowPhase::new();
        let mut joint_set = JointSet::new();
        let mut ccd_solver = CCDSolver::new();
        let physics_hooks = ();
        let event_handler = ();

        // Step the simulation.
        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigidbodies.0,
            &mut colliders.0,
            &mut joint_set,
            &mut ccd_solver,
            &physics_hooks,
            &event_handler,
          );
          */
    }
}