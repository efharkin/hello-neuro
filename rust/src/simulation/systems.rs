use specs::prelude::*;
use crate::simulation::components::*;
use crate::simulation::resources;
use crate::simulation::dynamical_variable::DynamicalVariable;

pub struct StaticConductanceHandler;

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

pub struct HardThresholdHandler;

impl<'a> System<'a> for HardThresholdHandler {
    type SystemData = (
        Read<'a, resources::TimeStep>,
        WriteStorage<'a, Voltage>,
        WriteStorage<'a, Spike>,
        ReadStorage<'a, HardThreshold>,
        ReadStorage<'a, VoltageReset>
    );

    /// Handle hard reset for components with a hard threshold.
    ///
    /// If current voltage is >= threshold, decrement towards hard reset by
    /// (reset_potential - current_voltage) mV.
    fn run(&mut self, (timestep, mut voltage, mut spike, threshold, reset): Self::SystemData) {
        let current_time = timestep.0;
        for (voltage, spike, threshold, reset)
            in (&mut voltage, &mut spike, &threshold, &reset).join()
        {
            if voltage.0.dynamical_get(current_time) >= threshold.0 {
                let distance_to_reset = reset.0 - voltage.0.dynamical_get(current_time);
                voltage.0.dynamical_increment(distance_to_reset, current_time);

                spike.0.dynamical_set(1, current_time);
            } else {
                spike.0.dynamical_set(0, current_time);
            }
        }
    }
}

pub struct SpikeMonitorHandler;

impl<'a> System<'a> for SpikeMonitorHandler {
    type SystemData = (
        Read<'a, resources::TimeStep>,
        ReadStorage<'a, Spike>,
        WriteStorage<'a, SpikeMonitor>
    );

    fn run(&mut self, (timestep, spike, mut monitor): Self::SystemData) {
        let current_time = timestep.0;
        for (spike, monitor) in (&spike, &mut monitor).join() {
            monitor.write(*spike, current_time);
        }
    }
}

pub struct VoltageMonitorHandler;

impl<'a> System<'a> for VoltageMonitorHandler {
    type SystemData = (
        Read<'a, resources::TimeStep>,
        ReadStorage<'a, Voltage>,
        WriteStorage<'a, VoltageMonitor>
    );

    fn run(&mut self, (timestep, voltage, mut monitor): Self::SystemData) {
        let current_time = timestep.0;
        for (voltage, monitor) in (&voltage, &mut monitor).join() {
            monitor.write(*voltage, current_time);
        }
    }
}
