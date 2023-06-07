from datetime import timedelta
from typing import TypeVar, Generic

T = TypeVar("T")
Self = TypeVar("Self")

class FixedMapping(Generic[T]):
    """
    A key-based mapping of ``floodgate.JumpingWindow`` that clears itself periodically.

    Every cooldown in the mapping has fixed capacity and period.

    See Also:
        ``floodgate.DynamicMapping`` for mappings of cooldowns with dynamic capacities and periods.
    """

    def __init__(
        self, capacity: int, period: timedelta, cycle_period: timedelta | None = None
    ) -> None:
        """
        Create a new mapping with fixed capacity and period in every cooldown.

        Args:
            capacity: How many triggers can occur in every cooldown.
            period: How long every cooldown lasts for.
            cycle_period: The period between every cycle that clears the mapping.
                If ``None``, the mapping will be cleared automatically every ``period``.

        Notes:
            ``FixedMapping`` uses a background thread to clear itself periodically.
            ``cycle_period`` must not be shorter than ``period``.
        """
    def tokens(self, key: T) -> int:
        """
        How many triggers (tokens) are left in the cooldown for the given key.

        Args:
            key: The key to the cooldown to check.

        Returns:
            The number of triggers (tokens) left in the cooldown for the given key.

        See Also:
            ``floodgate.JumpingWindow.token()``.
        """
    def next_reset(self, key: T) -> timedelta:
        """
        Time left until the next reset in the cooldown for the given key.

        Args:
            key: The key to the cooldown to check.

        Returns:
            The time until the next reset for the given key.

        See Also:
            ``floodgate.JumpingWindow.next_reset()``.
        """
    def retry_after(self, key: T) -> timedelta | None:
        """
        Similar to ``next_reset()``, except ``None`` is returned if you still have triggers
        in the cooldown for the given key.

        Args:
            key: The key to the cooldown to check.

        Returns:
            The time until the next reset for the given key, or ``None``
            if you still have triggers in the cooldown for the given key.

        See Also:
            ``floodgate.JumpingWindow.retry_after()``.
        """
    def can_trigger(self, key: T) -> bool:
        """
        Determine if there are still available triggers in the cooldown for the given key.

        Args:
            key: The key to the cooldown to check.

        Returns:
            Whether there are still available triggers for the given key.

        See Also:
            ``floodgate.JumpingWindow.can_trigger()``.
        """
    def trigger(self, key: T) -> timedelta | None:
        """
        Trigger the cooldown for the given key.

        Args:
            key: The key to the cooldown to trigger.

        Returns:
            How long until the next reset for the given key, or ``None`` if you still have triggers.

        See Also:
            ``floodgate.JumpingWindow.trigger()``.
        """
    def reset(self, key: T) -> None:
        """
        Reset the cooldown for the given key.

        Args:
            key: The key to the cooldown to reset.

        See Also:
            ``floodgate.JumpingWindow.reset()``.
        """

class DynamicMapping(Generic[T]):
    """
    Similar to the ``floodgate.FixedMapping`` class, except that each cooldown can have
    a different capacity and/or period.

    Cooldowns stored in this mapping can have different capacities and periods.

    See Also:
        ``floodgate.FixedMapping`` for mappings of cooldowns with a fixed capacity and period
        in every cooldown.
    """

    def __init__(self, max_period: timedelta) -> None:
        """
        Create a new cooldown mapping with dynamic capacity and period.

        Args:
            max_period:
                Period between every cycle that clears the mapping.
                No cooldown can last for longer than this period.

        Notes:
            ``FixedMapping`` uses a background thread to clear itself periodically.
            ``cycle_period`` must not be shorter than ``period``.

        """
    def tokens(self, key: T, capacity: int, duration: timedelta) -> int:
        """
        How many triggers (tokens) are left in the cooldown for the given key.

        Args:
            key: The key to the cooldown to check.
            capacity: The capacity of the cooldown to check.
            duration: The duration of the cooldown to check. Cannot be longer than ``max_period``.

        Returns:
            The number of triggers (tokens) left in the cooldown for the given key.

        See Also:
            ``floodgate.JumpingWindow.token()``.
        """
    def next_reset(self, key: T, capacity: int, duration: timedelta) -> timedelta:
        """
        Time left until the next reset in the cooldown for the given key.

        Args:
            key: The key to the cooldown to check.
            capacity: The capacity of the cooldown to check.
            duration: The duration of the cooldown to check. Cannot be longer than ``max_period``.

        Returns:
            The time until the next reset for the given key.

        See Also:
            ``floodgate.JumpingWindow.next_reset()``.
        """
    def retry_after(self, key: T, capacity: int, duration: timedelta) -> timedelta | None:
        """
        Similar to ``next_reset()``, except ``None`` is returned if you still have triggers
        in the cooldown for the given key.

        Args:
            key: The key to the cooldown to check.
            capacity: The capacity of the cooldown to check.
            duration: The duration of the cooldown to check. Cannot be longer than ``max_period``.

        Returns:
            The time until the next reset for the given key, or ``None``
            if you still have triggers in the cooldown for the given key.

        See Also:
            ``floodgate.JumpingWindow.retry_after()``.
        """
    def can_trigger(self, key: T, capacity: int, duration: timedelta) -> bool:
        """
        Determine if there are still available triggers in the cooldown for the given key.

        Args:
            key: The key to the cooldown to check.
            capacity: The capacity of the cooldown to check.
            duration: The duration of the cooldown to check. Cannot be longer than ``max_period``.

        Returns:
            Whether there are still available triggers for the given key.

        See Also:
            ``floodgate.JumpingWindow.can_trigger()``.
        """
    def trigger(self, key: T, capacity: int, duration: timedelta) -> timedelta | None:
        """
        Trigger the cooldown for the given key.

        Args:
            key: The key to the cooldown to trigger.
            capacity: The capacity of the cooldown to trigger.
            duration: The duration of the cooldown to trigger. Cannot be longer than ``max_period``.

        Returns:
            How long until the next reset for the given key, or ``None`` if you still have triggers.

        See Also:
            ``floodgate.JumpingWindow.trigger()``.
        """
    def reset(self, key: T, capacity: int, duration: timedelta) -> None:
        """
        Reset the cooldown for the given key.

        Args:
            key: The key to the cooldown to reset.
            capacity: The capacity of the cooldown to reset.
            duration: The duration of the cooldown to reset. Cannot be longer than ``max_period``.

        See Also:
            ``floodgate.JumpingWindow.reset()``.
        """

class JumpingWindow:
    """
    A simple ratelimit implementation.
    Might be referred to as "cooldown".

    Examples:
        >>> from datetime import timedelta
        >>> from floodgate import JumpingWindow

        >>> cooldown = JumpingWindow(capacity=2, period=timedelta(seconds=10))

        >>> assert cooldown.trigger() is None  # first trigger is fine
        >>> assert cooldown.trigger() is None  # second trigger is fine

        Once the triggers are used up, calling ``trigger()`` will return a "retry after" -
        - that is, how long before there will be more triggers available.
        >>> assert cooldown.trigger() is not None  # datetime.timedelta(microseconds=9999992)
    """

    def __init__(self, capacity: int, period: timedelta) -> None:
        """
        Create a new ``JumpingWindow``.

        Args:
            capacity: How many triggers can occur per cooldown.
            period: How long the cooldown is.
        """
    def token(self) -> int:
        """
        How many triggers (tokens) are left in this cooldown.

        Examples:
            >>> from datetime import timedelta
            >>> from floodgate import JumpingWindow

            >>> cooldown = JumpingWindow(capacity=1, period=timedelta(seconds=10))

            >>> assert cooldown.token() == 1
            >>> cooldown.trigger()
            >>> assert cooldown.token() == 0

        Returns:
            The number of triggers (tokens) left in the current cooldown.
        """
    def next_reset(self) -> timedelta:
        """
        Time left until the next reset.

        Examples:
            >>> from datetime import timedelta
            >>> from floodgate import JumpingWindow

            >>> cooldown = JumpingWindow(capacity=1, period=timedelta(seconds=10))
            >>> next_reset = cooldown.next_reset()

            >>> assert next_reset > timedelta(seconds=9)
            >>> assert next_reset < timedelta(seconds=11)

        Returns:
            The time until the next reset.
        """
    def retry_after(self) -> timedelta | None:
        """
        Similar to ``next_reset()``, except ``None`` is returned if you still have triggers.

        Examples:
            >>> from datetime import timedelta
            >>> from floodgate import JumpingWindow

            >>> cooldown = JumpingWindow(capacity=1, period=timedelta(seconds=10))

            >>> assert cooldown.retry_after() is None
            >>> cooldown.trigger()
            >>> assert cooldown.retry_after() is not None

        Returns:
            The time until the next reset, or ``None`` if you still have triggers.
        """
    def can_trigger(self) -> bool:
        """
        Determine if there are still available triggers.

        Examples:
            >>> from datetime import timedelta
            >>> from floodgate import JumpingWindow

            >>> cooldown = JumpingWindow(capacity=1, period=timedelta(seconds=10))

            >>> assert cooldown.can_trigger()
            >>> cooldown.trigger()
            >>> assert not cooldown.can_trigger()

        Returns:
            Whether there are still available triggers.
        """
    def trigger(self) -> timedelta | None:
        """
        Trigger this cooldown.

        Examples:
            >>> from datetime import timedelta
            >>> from floodgate import JumpingWindow

            >>> cooldown = JumpingWindow(capacity=1, period=timedelta(seconds=10))

            >>> assert cooldown.trigger() is None  # first trigger is fine
            >>> assert cooldown.trigger() is not None  # retry after the returned timedelta

        Returns:
            The period to retry after, or ``None`` if you still have triggers.
        """
    def reset(self) -> None:
        """
        Reset this cooldown.

        Examples:
            >>> from datetime import timedelta
            >>> from floodgate import JumpingWindow

            >>> cooldown = JumpingWindow(capacity=1, period=timedelta(seconds=10))

            >>> cooldown.trigger()
            >>> assert not cooldown.can_trigger()
            >>> cooldown.reset()
            >>> assert cooldown.can_trigger()
        """
