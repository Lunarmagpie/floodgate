# Floodgate-rs

Python bindings for [floodgate](https://github.com/CircuitSacul/floodgate/), a ratelimiting library written in rust.

## Example

```python
from datetime import timedelta
from floodgate import FixedMapping

cooldown = FixedMapping(capacity=5, period=timedelta(seconds=20))

def handle_event(sender):
    retry_after = cooldown.trigger(sender)
    if retry_after is None:
        print("Event succeeded!")
    else:
        print(f"Too many events from {sender}. Retry in {retry_after} seconds.")
```
