use specs::prelude::*;
use crate::simulation::dynamical_variable::*;

pub struct Voltage(pub DynamicalScalar<NeuroFloat>);

impl Voltage {
    pub fn new(initial_voltage: NeuroFloat) -> Voltage {
        Voltage(DynamicalScalar::new(initial_voltage))
    }
}

impl Component for Voltage {
    type Storage = VecStorage<Self>;
}

pub struct Capacitance(pub NeuroFloat);

impl Component for Capacitance {
    type Storage = VecStorage<Self>;
}

/// Voltage-independent conductance.
pub struct StaticConductance {
    conductance: NeuroFloat,
    reversal_voltage: NeuroFloat
}

impl StaticConductance {
    pub fn new(conductance: NeuroFloat, reversal_voltage: NeuroFloat) -> StaticConductance {
        StaticConductance {
            conductance: conductance,
            reversal_voltage: reversal_voltage
        }
    }

    pub fn get_current(&self, voltage: &Voltage, current_time: TimeStep) -> NeuroFloat {
        let driving_force = voltage.0.dynamical_get(current_time) - self.reversal_voltage;
        let current = self.conductance * driving_force;
        return current;
    }
}

impl Component for StaticConductance {
    type Storage = VecStorage<Self>;
}

pub struct HardThreshold(pub NeuroFloat);

impl Component for HardThreshold {
    type Storage = VecStorage<Self>;
}

pub struct VoltageReset(pub NeuroFloat);

impl Component for VoltageReset {
    type Storage = VecStorage<Self>;
}

pub struct Spike(pub DynamicalScalar<usize>);

impl Spike {
    pub fn new() -> Spike {
        Spike(DynamicalScalar::new(0))
    }
}

impl Component for Spike {
    type Storage = VecStorage<Self>;
}


// MONITORS
pub trait Monitor<T> {
   fn write(&self, monitored_variable: T, current_time: TimeStep);
}

pub struct SpikeMonitor {
    bufwriter: std::io::BufWriter<std::fs::File>,
}

impl SpikeMonitor {
    pub fn new(output_file: std::fs::File) -> SpikeMonitor {
        SpikeMonitor {
            bufwriter: std::io::BufWriter::new(output_file)
        }
    }
}

impl Monitor<Spike> for SpikeMonitor {
    fn write(&self, monitored_variable: Spike, current_time: TimeStep) {
        use std::io::Write;
        if monitored_variable.0.dynamical_get(current_time) > 0 {
            writeln!(self.bufwriter, "{}", current_time);
        }
    }
}

impl Component for SpikeMonitor {
    type Storage = VecStorage<Self>;
}

pub struct VoltageMonitor {
    bufwriter: std::io::BufWriter<std::fs::File>,
}

impl VoltageMonitor {
    pub fn new(output_file: std::fs::File) -> VoltageMonitor {
        VoltageMonitor {
            bufwriter: std::io::BufWriter::new(output_file)
        }
    }
}

impl Monitor<Voltage> for VoltageMonitor {
    fn write(&self, monitored_variable: Voltage, current_time: TimeStep) {
        use std::io::Write;
        writeln!(self.bufwriter, "{}", monitored_variable.0.dynamical_get(current_time));
    }
}

impl Component for VoltageMonitor {
    type Storage = VecStorage<Self>;
}
