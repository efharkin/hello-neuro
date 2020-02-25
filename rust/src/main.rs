mod simulation;
use specs::prelude::*;

use simulation::factories::{
    LIFParameters,
    create_simulation_world,
    create_lif_ensemble
};
use simulation::systems::*;

fn main() {
    let mut world = create_simulation_world(0.1);
    let lif_parameters = LIFParameters::from_scalars(0.1, 0.01, -70.0, -50.0, -60.0);
    create_lif_ensemble(&mut world, lif_parameters, 10);

    let mut dispatcher = DispatcherBuilder::new()
        .with(StaticConductanceHandler, "static_conductance_handler", &[])
        .with(HardThresholdHandler, "hard_threshold_handler", &[])
        .build();

    dispatcher.dispatch(&mut world);
    world.maintain();
}
