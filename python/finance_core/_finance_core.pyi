__all__ = [
    "Bar",
    "ExponentialMovingAverage",
    "Maximum",
    "Minimum",
    "MovingAverageConvergenceDivergence",
    "SimpleMovingAverage",
    "TrueRange"
]


class Bar:
    def __init__(
            self,
            open: float,
            high: float,
            low: float,
            close: float,
            volume: int
    ) -> None:
        """Bar data item."""


class ExponentialMovingAverage:
    def __init__(self, period: int) -> None:
        """Exponential moving average."""

    def period(self) -> int:
        """Return the number of periods."""

    def next(self, input: float) -> float:
        """Calculate the exponential moving average of the current periods."""

    def reset(self) -> None:
        """Reset the current calculations."""


class Maximum:
    def __init__(self, period: int) -> None:
        """Create a new maximum indicator."""

    def period(self) -> int:
        """Return the number of periods."""

    def next(self, input: float) -> float:
        """Calculate the maximum of the current periods."""

    def reset(self) -> None:
        """Reset the current calculations."""


class Minimum:
    def __init__(self, period: int) -> None:
        """Create a minimum indicator."""

    def period(self) -> int:
        """Return the number of periods."""

    def next(self, input: float) -> float:
        """Calculate the minimm of the current periods."""

    def reset(self) -> None:
        """Reset the current calculations."""


class MovingAverageConvergenceDivergence:
    def __init__(self, long_period: int, short_period: int, signal_period: int) -> None:
        """Moving average convergence divergence."""

    def next(self, input: float) -> tuple[float, float, float]:
        """Calculate the moving average convergence divergence of the current periods.

        Returns the MACD (0), signal (1), and histogram (2).
        """

    def reset(self) -> None:
        """Reset the current calculations."""


class SimpleMovingAverage:
    def __init__(self, period: int) -> None:
        """Create a simple moving average indicator."""

    def period(self) -> int:
        """Return the number of periods."""

    def next(self, input: float) -> float:
        """Calculate the simple moving average of the current periods."""

    def reset(self) -> None:
        """Reset the current calculations."""


class TrueRange:
    def __init__(self) -> None:
        """Create a true range indicator."""

    def next(self, input: Bar) -> float:
        """Calculate the true range."""

    def reset(self) -> None:
        """Reset the current true range."""
