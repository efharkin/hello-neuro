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
