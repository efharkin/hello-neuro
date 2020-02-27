pub type NeuroFloat = f32;
pub type TimeStep = usize;

pub trait DynamicalVariable<T>
    where T: std::ops::AddAssign + std::marker::Copy
{
    fn dynamical_get(&self, current_time: TimeStep) -> T;
    fn dynamical_set(&mut self, value: T, current_time: TimeStep);
    fn dynamical_increment(&mut self, amount: T, current_time: TimeStep);
}

#[derive(Copy, Clone)]
pub struct DynamicalScalar<T>
    where T: std::ops::AddAssign + std::marker::Copy
{
    value: [T; 2]
}

impl<T> DynamicalScalar<T>
    where T: std::ops::AddAssign + std::marker::Copy
{
    pub fn new(value: T) -> DynamicalScalar<T> {
        DynamicalScalar{
            value: [value; 2]
        }
    }
}

impl<T> DynamicalVariable<T> for DynamicalScalar<T>
    where T: std::ops::AddAssign + std::marker::Copy
{
    fn dynamical_get(&self, current_time: TimeStep) -> T {
        let ind = (current_time - 1) % 2;
        return self.value[ind];
    }
    fn dynamical_set(&mut self, value: T, current_time: TimeStep) {
        let ind = current_time % 2;
        self.value[ind] = value;
    }
    fn dynamical_increment(&mut self, amount: T, current_time: TimeStep) {
        let ind = current_time % 2;
        self.value[ind] += amount;
    }
}

#[cfg(test)]
mod dynamical_scalar_tests {
    use super::*;

    #[test]
    fn get_returns_value_passed_at_init() {
        let expected = 5.0;
        let test_scalar = DynamicalScalar::new(expected);
        let time: TimeStep = 1;
        assert_eq!(test_scalar.dynamical_get(time), expected);
        assert_eq!(test_scalar.dynamical_get(time + 1), expected);
    }

    #[test]
    fn dynamical_set_only_alters_future() {
        let initial = 5.0;
        let increment = 1.0;
        let time: TimeStep = 1;

        let mut test_scalar = DynamicalScalar::new(initial);
        test_scalar.dynamical_set(initial + increment, time);

        assert_eq!(test_scalar.dynamical_get(time), initial);
        assert_eq!(test_scalar.dynamical_get(time + 1), initial + increment);
    }

    #[test]
    fn dynamical_increment_only_alters_future() {
        let initial = 5.0;
        let increment = 1.0;
        let time: TimeStep = 1;

        let mut test_scalar = DynamicalScalar::new(initial);
        test_scalar.dynamical_increment(increment, time);

        assert_eq!(test_scalar.dynamical_get(time), initial);
        assert_eq!(test_scalar.dynamical_get(time + 1), initial + increment);
    }
}
