use specs::prelude::*;
use crate::simulation::components::*;
use crate::simulation::dynamical_variable::{NeuroFloat};
use crate::simulation::resources::{add_time_resources, DeltaTime, TimeStep};

pub struct NeuroSimulation {
    world: World,
    monitor_file: hdf5::File,
    num_timesteps: usize
}

impl NeuroSimulation {
    pub fn new(output_file_name: &str, time_step_length: NeuroFloat, num_timesteps: usize) -> NeuroSimulation {
        let mut simulation = NeuroSimulation {
            world: World::new(),
            monitor_file: hdf5::File::create(output_file_name).expect("Could not create monitor file."),
            num_timesteps: num_timesteps
        };
        add_time_resources(&mut simulation.world, DeltaTime(time_step_length));
        return simulation;
    }

    pub fn run(&mut self, dispatcher: &mut Dispatcher) {
        for timestep in 1..self.num_timesteps {
            dispatcher.dispatch(&self.world);
            self.world.maintain();
            let mut current_time = self.world.write_resource::<TimeStep>();
            *current_time = TimeStep(timestep + 1);
        }
    }
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
pub fn create_lif_ensemble(simulation: &mut NeuroSimulation, ensemble_name: &str, model_parameters: LIFParameters, num_neurons: usize) {
    simulation.world.register::<Capacitance>();
    simulation.world.register::<StaticConductance>();
    simulation.world.register::<HardThreshold>();
    simulation.world.register::<VoltageReset>();
    simulation.world.register::<Voltage>();
    simulation.world.register::<Spike>();
    simulation.world.register::<VoltageMonitor>();
    simulation.world.register::<SpikeMonitor>();

    let lif_group = simulation.monitor_file.create_group(ensemble_name)
        .expect(&format!("Could not create hdf5 group {}", ensemble_name));
    let voltage_group = lif_group.create_group("voltage")
        .expect(&format!("Could not create voltage data group for {}", ensemble_name));

    for neuron_num in 0..num_neurons {
        let voltage_dataset = voltage_group.new_dataset::<NeuroFloat>()
            .create(&format!("{}", neuron_num), simulation.num_timesteps)
            .expect(&format!("Could not create voltage dataset {} for {}", neuron_num, ensemble_name));

        simulation.world.create_entity()
            .with(Capacitance(model_parameters.capacitance))
            .with(StaticConductance::new(model_parameters.leak_conductance, model_parameters.leak_reversal))
            .with(HardThreshold(model_parameters.threshold))
            .with(VoltageReset(model_parameters.reset))
            .with(Voltage::new(model_parameters.leak_reversal))
            .with(Spike::new())
            .with(
                VoltageMonitor::new(voltage_dataset, neuron_num)
            )
            .with(
                SpikeMonitor::new(
                    std::fs::File::create(
                        format!("neuron_{}_spike_monitor.ras", neuron_num)
                    ).expect("Failed to create spike monitor file.")
                )
            )
            .build();
    }
}
