use specs::prelude::*;
use crate::simulation::components::*;
use crate::simulation::resources;
use crate::simulation::dynamical_variable::DynamicalVariable;

struct StaticConductanceHandler;

impl<'a> System<'a> for StaticConductanceHandler {
    type SystemData = (
        Read<'a, resources::TimeStep>,
        WriteStorage<'a, Voltage>,
        ReadStorage<'a, Capacitance>,
        ReadStorage<'a, StaticConductance>
    );

    fn run(&mut self, (timestep, mut voltage, capacitance, static_conductance): Self::SystemData) {
        let current_time = timestep.0;
        for (voltage, capacitance, static_conductance)
            in (&mut voltage, &capacitance, &static_conductance).join()
        {

            let current = static_conductance.get_current(voltage, current_time);
            let voltage_increment = current / capacitance.0;
            voltage.0.dynamical_increment(voltage_increment, current_time);
        }
    }
}

struct HardThresholdHandler;

impl<'a> System<'a> for HardThresholdHandler {
    type SystemData = (
        Read<'a, resources::TimeStep>,
        WriteStorage<'a, Voltage>,
        ReadStorage<'a, HardThreshold>,
        ReadStorage<'a, VoltageReset>
    );

    /// Handle hard reset for components with a hard threshold.
    ///
    /// If current voltage is >= threshold, decrement towards hard reset by
    /// (reset_potential - current_voltage) mV.
    fn run(&mut self, (timestep, mut voltage, threshold, reset): Self::SystemData) {
        let current_time = timestep.0;
        for (voltage, threshold, reset)
            in (&mut voltage, &threshold, &reset).join()
        {
            if voltage.0.dynamical_get(current_time) >= threshold.0 {
                let distance_to_reset = reset.0 - voltage.0.dynamical_get(current_time);
                voltage.0.dynamical_increment(distance_to_reset, current_time);
            }
        }
    }
}
