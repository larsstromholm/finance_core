# Finance core

Financial indicators implemented in Rust with a Python interface.

## Install
```bash
pip install finance_core
```

## Example usage
```python
from finance_core import Maximum, Minimum, SimpleMovingAverage


three_day_max = Maximum(3)

print(three_day_max.next(1)) # >> 1
print(three_day_max.next(4)) # >> 4
print(three_day_max.next(2)) # >> 4
print(three_day_max.next(5)) # >> 5


three_day_min = Minimum(3)

print(three_day_min.next(1)) # >> 1
print(three_day_min.next(4)) # >> 1
print(three_day_min.next(2)) # >> 1
print(three_day_min.next(5)) # >> 2


sma_three_day = SimpleMovingAverage(3)

print(sma_three_day.next(1)) # >> 1
print(sma_three_day.next(4)) # >> 2.5
print(sma_three_day.next(2)) # >> 2.33
print(sma_three_day.next(5)) # >> 3.66
```

## Contribute

Install Docker and open this repo in DevContainer.

```bash
# create virtual environment
python -m venv .venv

# activate virtual environment
source .venv/bin/activate

# install dependencies
pip install -r requirements.txt
```

Useful commands:
- `make build-dev` to build the package during development
- `make build-prod` to perform an optimised build for benchmarking