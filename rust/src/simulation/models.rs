trait IntegrateAndFire{
    fn integrate(&mut self, current : f32, time_step : f32);
    fn sample_spiking_process(&mut self);
}

struct LeakyIntegrateAndFire{
    voltage: f32,
    is_firing: bool,
    threshold: f32,
    reset_voltage: f32,
    leak_conductance: f32,
    capacitance: f32,
    equilibrium_voltage: f32
}

impl IntegrateAndFire for LeakyIntegrateAndFire {
    fn integrate(&mut self, current: f32, time_step: f32) {
        if self.is_firing {
            self.voltage = self.reset_voltage;
            self.is_firing = false;
        } else {
            let voltage_derivative = 
                (
                    - self.leak_conductance * (self.voltage - self.equilibrium_voltage)
                    + current
                ) 
                / self.capacitance;
            self.voltage += voltage_derivative * time_step;
        }
    }

    fn sample_spiking_process(&mut self) {
        if self.voltage >= self.threshold {
            self.is_firing = true;
        } else {
            self.is_firing = false;
        }
    }
}

#[cfg(test)]
mod lif_tests {
    use super::*;

    fn setup_lif() -> LeakyIntegrateAndFire {
        LeakyIntegrateAndFire{
            voltage: -70.0,
            is_firing: false,
            threshold: -50.0,
            reset_voltage: -60.0,
            leak_conductance: 0.1,
            capacitance: 0.2,
            equilibrium_voltage: -70.0
        }
    }

    #[test]
    fn test_integrate() {
        let mut _lif = setup_lif();
        let initial_voltage = _lif.voltage;
        _lif.integrate(0.100, 0.1);
        assert_eq!(initial_voltage, -70.0); // Sanity check.
        assert!(
            _lif.voltage > initial_voltage, 
            "Voltage not increased by positive current."
        );
    }

    #[test]
    fn test_spiking_process() {
        let mut lif = setup_lif();

        // Don't spike below threshold.
        for _i in 0..10 {
            lif.sample_spiking_process();
            assert!(! lif.is_firing, "Spiking without input.");
        }

        // Do spike above threshold.
        lif.voltage = lif.threshold + 5.0;
        lif.sample_spiking_process();
        assert!(lif.is_firing, "Not spiking when above threshold.");
    }
}
