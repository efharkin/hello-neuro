use specs::prelude::*;

#[derive(Default)]
pub struct TimeStep(pub usize);

#[derive(Default)]
pub struct DeltaTime(pub f32);

pub fn add_time_resources(world: &mut World, time_step_length: DeltaTime) {
    world.insert(TimeStep(0));
    world.insert(time_step_length);
}
