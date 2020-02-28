mod simulation;
use specs::prelude::*;

use simulation::factories::{
    LIFParameters,
    NeuroSimulation,
    create_lif_ensemble
};
use simulation::systems::*;

fn main() {
    let num_timesteps = 10000;
    let mut sim = NeuroSimulation::new("test_simulation.hdf5", 0.1, num_timesteps);
    let lif_parameters = LIFParameters::from_scalars(0.1, 0.01, -70.0, -50.0, -60.0);

    create_lif_ensemble(&mut sim, "test_lif_ensemble", lif_parameters, 100);

    let mut dispatcher = DispatcherBuilder::new()
        .with(StaticConductanceHandler, "static_conductance_handler", &[])
        .with(HardThresholdHandler, "hard_threshold_handler", &[])
        .with(SpikeMonitorHandler, "spike_monitor_handler", &[])
        .with(VoltageMonitorHandler, "voltage_monitor_handler", &[])
        .with(ConstantInputHandler, "constant_input_handler", &[])
        .build();

    sim.run(&mut dispatcher);
}
