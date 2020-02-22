use specs::prelude::*;
use crate::simulation::components::*;

struct LeakyIntegrationHandler;

impl<'a> System<'a> for LeakyIntegrationHandler {
    type SystemData = (WriteStorage<'a, Voltage>, ReadStorage<'a, Capacitance>, ReadStorage<'a, PassiveConductance>)
}

