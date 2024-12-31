use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::traits::{Next, Period, Reset};


#[pyclass]
pub struct ExponentialMovingAverage {
    period: usize,
    s: f64,
    current: f64,
    is_new: bool,
}

#[pymethods]
impl ExponentialMovingAverage {
    #[new]
    pub fn new(period: usize) -> PyResult<Self> {
        match period {
            0 => Err(PyValueError::new_err("Period cannot be 0.")),
            _ => Ok(Self {
                period,
                s: 2.0 / (1 + period) as f64,
                current: 0.0,
                is_new: true,
            })
        }
    }

    pub fn period(&mut self) -> usize {
        Period::period(self)
    }

    pub fn next(&mut self, input: f64) -> f64 {
        Next::next(self, input)
    }

    pub fn reset(&mut self) {
        Reset::reset(self)
    }
}

impl Period for ExponentialMovingAverage {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<f64> for ExponentialMovingAverage {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        if self.is_new {
            self.is_new = false;
            self.current = input;
        } else {
            self.current = input * self.s + self.current * (1.0 - self.s)
        }

        return self.current;
    }
}

impl Reset for ExponentialMovingAverage {
    fn reset(&mut self) {
        self.current = 0.0;
        self.is_new = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::ExponentialMovingAverage;

    #[test]
    fn test_new() {
        assert!(ExponentialMovingAverage::new(0).is_err());
        assert!(ExponentialMovingAverage::new(3).is_ok());
    }

    #[test]
    fn test_next() {
        let mut ema = ExponentialMovingAverage::new(3).unwrap();

        assert_eq!(ema.next(2.0), 2.0);
        assert_eq!(ema.next(5.0), 3.5);
        assert_eq!(ema.next(7.0), 5.25);
        assert_eq!(ema.next(6.0), 5.625);
    }

    #[test]
    fn test_reset() {
        let mut ema = ExponentialMovingAverage::new(3).unwrap();

        assert_eq!(ema.next(2.0), 2.0);
        ema.next(5.0);
        ema.next(7.0);
        ema.next(6.0);

        ema.reset();
        assert_eq!(ema.next(2.0), 2.0);
        
        ema.reset();
        assert_eq!(ema.next(4.0), 4.0);
    }
}
