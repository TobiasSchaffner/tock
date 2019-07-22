//! Hardware agnostic interfaces for counter-like resources.

use crate::ReturnCode;

pub trait Time {
    type Frequency: Frequency;

    /// Disable any outstanding alarm or timer
    fn disable(&self);

    /// Returns whether a timer or alarm is currently armed
    fn is_armed(&self) -> bool;

    /// Returns the current time in hardware clock units.
    fn now(&self) -> u32;
}

pub trait Counter: Time {
    fn start(&self) -> ReturnCode;
    fn stop(&self) -> ReturnCode;
    fn is_running(&self) -> bool;
}

/// Trait to represent clock frequency in Hz
///
/// This trait is used as an associated type for `Alarm` so clients can portably
/// convert native cycles to real-time values.
pub trait Frequency {
    fn frequency() -> u32;
}

/// 16MHz `Frequency`
#[derive(Debug)]
pub struct Freq16MHz;
impl Frequency for Freq16MHz {
    fn frequency() -> u32 {
        16000000
    }
}

/// 32KHz `Frequency`
#[derive(Debug)]
pub struct Freq32KHz;
impl Frequency for Freq32KHz {
    fn frequency() -> u32 {
        32768
    }
}

/// 16KHz `Frequency`
#[derive(Debug)]
pub struct Freq16KHz;
impl Frequency for Freq16KHz {
    fn frequency() -> u32 {
        16000
    }
}

/// 1KHz `Frequency`
#[derive(Debug)]
pub struct Freq1KHz;
impl Frequency for Freq1KHz {
    fn frequency() -> u32 {
        1000
    }
}

/// The `Alarm` trait models a wrapping counter capapable of notifying when the
/// counter reaches a certain value.
///
/// Alarms represent a resource that keeps track of time in some fixed unit
/// (usually clock tics). Implementors should use the
/// [`Client`](trait.Client.html) trait to signal when the counter has
/// reached a pre-specified value set in [`set_alarm`](#tymethod.set_alarm).
pub trait Alarm: Time {
    /// Sets a one-shot alarm fire when the clock reaches `tics`.
    ///
    /// [`Client#fired`](trait.Client.html#tymethod.fired) is signaled
    /// when `tics` is reached.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let delta = 1337;
    /// let tics = alarm.now().wrapping_add(delta);
    /// alarm.set_alarm(tics);
    /// ```
    fn set_alarm(&self, tics: u32);

    /// Returns the value set in [`set_alarm`](#tymethod.set_alarm)
    fn get_alarm(&self) -> u32;

    fn set_client(&self, client: &'static AlarmClient);

    fn is_enabled(&self) -> bool;

    // Q(alevy): Does the client need to call `enable` after `set_alarm`? Or is `enable` implicit
    // after calling `set_alarm`? If not, maybe we _just_ need `disable` since re-enabling can be
    // implemented as `get_alarm -> set_alarm`.
    fn enable(&self) -> ReturnCode;

    // Q(alevy): this just disables the alarm, right, it doesn't stop the clock
    fn disable(&self) -> ReturnCode;
}

/// A client of an implementor of the [`Alarm`](trait.Alarm.html) trait.
pub trait AlarmClient {
    /// Callback signaled when the alarm's clock reaches the value set in
    /// [`Alarm#set_alarm`](trait.Alarm.html#tymethod.set_alarm).
    fn fired(&self);
}

/// The `Timer` trait models a timer that can notify when a particular interval
/// has elapsed.
pub trait Timer: Time {
    fn set_client(&self, client: &'static TimerClient);

    /// Sets a one-shot timer to fire in `interval` clock-tics.
    fn oneshot(&self, interval: u32);
    /// Sets repeating timer to fire every `interval` clock-tics.
    fn repeat(&self, interval: u32);

    // Q(alevy): Implementing this might require an additional, unnecessary field if a repeating
    // timer is distinguished by having a non-zero value in the reload register. What if the return
    // value is `Option<u32>` and `None` means it's oneshot, `Some(interval)` means it's repeating.
    // Side benefit, `is_oneshot` and `is_repeating` can have default implementations.
    fn interval(&self) -> u32;

    fn is_oneshot(&self) -> bool;
    fn is_repeating(&self) -> bool;

    // This should return an option. Again, `is_enabled` can have a default implementation
    fn time_remaining(&self) -> u32; // Returns 0 if disabled

    fn is_enabled(&self) -> bool;

    // Q(alevy): what are possible return values? why would you not be able to cancel a timer?
    fn cancel(&self) -> ReturnCode;
}

/// A client of an implementor of the [`Timer`](trait.Timer.html) trait.
pub trait TimerClient {
    /// Callback signaled when the timer's clock reaches the specified interval.
    fn fired(&self);
}
