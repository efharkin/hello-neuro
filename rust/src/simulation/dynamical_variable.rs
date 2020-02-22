pub type NeuroFloat = f32;
pub type TimeStep = usize;

trait DynamicalVariable<T>
    where T: std::ops::AddAssign + std::marker::Copy
{
    fn get(&self, current_time: TimeStep) -> T;
    fn set(&mut self, value: T, current_time: TimeStep);
    fn increment(&mut self, amount: T, current_time: TimeStep);
}

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
    fn get(&self, current_time: TimeStep) -> T {
        let ind = (current_time - 1) % 2;
        return self.value[ind];
    }
    fn set(&mut self, value: T, current_time: TimeStep) {
        let ind = current_time % 2;
        self.value[ind] = value;
    }
    fn increment(&mut self, amount: T, current_time: TimeStep) {
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
        assert_eq!(test_scalar.get(time), expected);
        assert_eq!(test_scalar.get(time + 1), expected);
    }

    #[test]
    fn set_only_alters_future() {
        let initial = 5.0;
        let increment = 1.0;
        let time: TimeStep = 1;

        let mut test_scalar = DynamicalScalar::new(initial);
        test_scalar.set(initial + increment, time);

        assert_eq!(test_scalar.get(time), initial);
        assert_eq!(test_scalar.get(time + 1), initial + increment);
    }

    #[test]
    fn increment_only_alters_future() {
        let initial = 5.0;
        let increment = 1.0;
        let time: TimeStep = 1;

        let mut test_scalar = DynamicalScalar::new(initial);
        test_scalar.increment(increment, time);

        assert_eq!(test_scalar.get(time), initial);
        assert_eq!(test_scalar.get(time + 1), initial + increment);
    }
}
