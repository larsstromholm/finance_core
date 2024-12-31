use pyo3::prelude::*;

use crate::traits::{Next, Reset};

use crate::ExponentialMovingAverage;


#[pyclass]
pub struct MovingAverageConvergenceDivergence {
    long_ema: ExponentialMovingAverage,
    short_ema: ExponentialMovingAverage,
}

#[pymethods]
impl MovingAverageConvergenceDivergence {
    #[new]
    pub fn new(
        long_period: usize, 
        short_period: usize,
    ) -> PyResult<Self> {
        Ok( Self {
            long_ema: ExponentialMovingAverage::new(long_period)?,
            short_ema: ExponentialMovingAverage::new(short_period)?,
        })
    }

    pub fn next(&mut self, input: f64) -> f64 {
        Next::next(self, input)
    }

    pub fn reset(&mut self) {
        Reset::reset(self)
    }
}

impl Next<f64> for MovingAverageConvergenceDivergence {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let long_val = self.long_ema.next(input);
        let short_val = self.short_ema.next(input);
        
        let macd = short_val - long_val;

        macd
    }

}

impl Reset for MovingAverageConvergenceDivergence {

    fn reset(&mut self) {
        self.long_ema.reset();
        self.short_ema.reset();
    }

}

#[cfg(test)]
mod tests {

    use crate::MovingAverageConvergenceDivergence;

    #[test]
    fn test_new() {
        assert!(MovingAverageConvergenceDivergence::new(0, 12).is_err());
        assert!(MovingAverageConvergenceDivergence::new(26, 0).is_err());
        assert!(MovingAverageConvergenceDivergence::new(26, 12).is_ok());
    }

    #[test]
    fn test_next() {
        let mut macd = MovingAverageConvergenceDivergence::new(26, 12).unwrap();


        assert_eq!(macd.next(3.0), 0.0);
        assert_eq!(macd.next(4.0), 0.07977207977207978);
        assert_eq!(macd.next(5.0), 0.22113456871291648);
        assert_eq!(macd.next(6.0), 0.40914068319690067);

    }

    #[test]
    fn test_reset() {
        let mut macd = MovingAverageConvergenceDivergence::new(26, 12).unwrap();

        assert_eq!(macd.next(3.0), 0.0);
        assert_eq!(macd.next(4.0), 0.07977207977207978);

        macd.reset();

        assert_eq!(macd.next(3.0), 0.0);
        assert_eq!(macd.next(4.0), 0.07977207977207978);
    }
}