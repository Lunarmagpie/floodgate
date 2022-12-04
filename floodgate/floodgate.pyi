from datetime import timedelta
from typing import TypeVar, Generic

T = TypeVar("T")
Self = TypeVar("Self")

class FixedMapping(Generic[T]):
    def __init__(self, capacity: int, period: timedelta) -> None: ...
    def tokens(self, key: T) -> int: ...
    def next_reset(self, key: T) -> timedelta: ...
    def retry_after(self, key: T) -> timedelta | None: ...
    def can_trigger(self, key: T) -> bool: ...
    def trigger(self, key: T) -> timedelta | None: ...
    def reset(self, key: T) -> None: ...
    def start(self: Self, cycle_period: timedelta | None = None) -> Self: ...

class DynamicMapping(Generic[T]):
    def __init__(self, max_period: timedelta) -> None: ...
    def tokens(self, key: T, capacity: int, duration: timedelta) -> int: ...
    def next_reset(self, key: T, capacity: int, duration: timedelta) -> timedelta: ...
    def retry_after(self, key: T, capacity: int, duration: timedelta) -> timedelta | None: ...
    def can_trigger(self, key: T, capacity: int, duration: timedelta) -> bool: ...
    def trigger(self, key: T, capacity: int, duration: timedelta) -> timedelta | None: ...
    def reset(self, key: T, capacity: int, duration: timedelta) -> None: ...
    def start(self: Self) -> Self: ...

class JumpingWindow:
    def __init__(self, capacity: int, period: timedelta) -> None: ...
    def token(self) -> int: ...
    def next_reset(self) -> timedelta: ...
    def retry_after(self) -> timedelta | None: ...
    def can_trigger(self) -> bool: ...
    def trigger(self) -> timedelta | None: ...
    def reset(self) -> None: ...
