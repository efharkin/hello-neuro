use specs::prelude::*;
use crate::simulation::components::*;
use crate::simulation::dynamical_variable::NeuroFloat;
use crate::simulation::resources::{add_time_resources, DeltaTime};

pub fn create_simulation_world(time_step_length: NeuroFloat) -> World {
    let mut world = World::new();
    add_time_resources(&mut world, DeltaTime(time_step_length));
    return world;
}

pub struct LIFParameters {
    capacitance: NeuroFloat,
    leak_conductance: NeuroFloat,
    leak_reversal: NeuroFloat,
    threshold: NeuroFloat,
    reset: NeuroFloat
}

impl LIFParameters {
    pub fn from_scalars(
        capacitance: NeuroFloat,
        leak_conductance: NeuroFloat,
        leak_reversal: NeuroFloat,
        threshold: NeuroFloat,
        reset: NeuroFloat
    ) -> LIFParameters
    {
        LIFParameters {
            capacitance: capacitance,
            leak_conductance: leak_conductance,
            leak_reversal: leak_reversal,
            threshold: threshold,
            reset: reset
        }
    }
}
pub fn create_lif_ensemble(world: &mut World, model_parameters: LIFParameters, num_neurons: usize) {
    world.register::<Capacitance>();
    world.register::<StaticConductance>();
    world.register::<HardThreshold>();
    world.register::<VoltageReset>();
    world.register::<Voltage>();

    for _ in 0..num_neurons {
        world.create_entity()
            .with(Capacitance(model_parameters.capacitance))
            .with(StaticConductance::new(model_parameters.leak_conductance, model_parameters.leak_reversal))
            .with(HardThreshold(model_parameters.threshold))
            .with(VoltageReset(model_parameters.reset))
            .with(Voltage::new(model_parameters.leak_reversal))
            .build();
    }
}
