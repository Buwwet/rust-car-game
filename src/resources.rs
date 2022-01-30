use nalgebra::vector;
use parry3d::math::{Vector, Real};
use rapier3d::prelude::{PhysicsPipeline, RigidBodySet, ColliderSet, IntegrationParameters, IslandManager, BroadPhase, NarrowPhase, JointSet, CCDSolver, PhysicsHooks, EventHandler};
use specs::World;

pub fn insert_resources(world: &mut World) {
    // Insert the physics resources to the world.
    world.insert(RigidBodyContainer::default());
    world.insert(ColliderContainer::default());
    world.insert(PhysicsResource::default());
}

// Custom Structs to hold RigidBodySet & ColliderSet Resources;
pub struct RigidBodyContainer(pub RigidBodySet);
impl Default for RigidBodyContainer {
    fn default() -> Self {
        let rb = RigidBodySet::new();
        RigidBodyContainer(rb)
    }
}
pub struct ColliderContainer(pub ColliderSet);
impl Default for ColliderContainer {
    fn default() -> Self {
        let rb = ColliderSet::new();
        ColliderContainer(rb)
    }
}
// We need to store these or physics step won't take place.
pub struct PhysicsResource {
    pub physics_pipeline: PhysicsPipeline,
    pub gravity: Vector<Real>,
    pub integration_parameters: IntegrationParameters,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub joint_set: JointSet,
    pub ccd_solver: CCDSolver,
    pub physics_hooks: (),
    pub event_handler: (),
}
// Generate all of the Resources needed for physics!
impl Default for PhysicsResource {
    fn default() -> Self {
        /* Create structures necessary for the simulation. */
        let gravity = vector![0.0, -9.822, 0.0];
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let joint_set = JointSet::new();
        let ccd_solver = CCDSolver::new();
        let physics_hooks = ();
        let event_handler = ();

        PhysicsResource {
            gravity,
            integration_parameters,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            joint_set,
            ccd_solver,
            physics_hooks,
            event_handler,
        }
    }
}
impl PhysicsResource {
    pub fn step(&mut self, bodies: &mut RigidBodySet, colliders: &mut ColliderSet) {
        // Run the PhysicsPipeline here.
        self.physics_pipeline.step(
            &self.gravity, 
            &self.integration_parameters, 
            &mut self.island_manager,
            &mut self.broad_phase, 
            &mut self.narrow_phase, 
            bodies, 
            colliders, 
            &mut self.joint_set, 
            &mut self.ccd_solver, 
            &self.physics_hooks, 
            &self.event_handler,
        );
    }
}
