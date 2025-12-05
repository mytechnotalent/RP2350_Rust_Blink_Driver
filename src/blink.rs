/*
 * @file blink.rs
 * @brief LED blink state machine
 * @author Kevin Thomas
 * @date 2025
 *
 * MIT License
 *
 * Copyright (c) 2025 Kevin Thomas
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

//! FILE: blink.rs
//!
//! DESCRIPTION:
//! RP2350 LED Blink State Machine.
//!
//! BRIEF:
//! Implements LED state tracking and toggle logic.
//! Provides testable state machine for blink functionality.
//!
//! AUTHOR: Kevin Thomas
//! CREATION DATE: November 28, 2025
//! UPDATE DATE: December 4, 2025

use crate::config::{BLINK_DELAY_MS, MAX_BLINK_DELAY_MS, MIN_BLINK_DELAY_MS};

/// LED state enumeration.
///
/// # Details
/// Represents the current state of the LED.
/// Used for state tracking and transitions.
///
/// # Variants
/// * `On` - LED is currently on (high)
/// * `Off` - LED is currently off (low)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LedState {
    On,
    Off,
}

/// Blink controller with state tracking.
///
/// # Details
/// Maintains LED state and blink timing configuration.
/// Provides methods for state transitions and queries.
///
/// # Fields
/// * `state` - Current LED state
/// * `delay_ms` - Blink delay in milliseconds
/// * `toggle_count` - Number of state transitions
#[derive(Debug)]
pub struct BlinkController {
    state: LedState,
    delay_ms: u64,
    toggle_count: u64,
}

/// Default implementation for BlinkController.
impl Default for BlinkController {
    fn default() -> Self {
        Self {
            state: LedState::Off,
            delay_ms: BLINK_DELAY_MS,
            toggle_count: 0,
        }
    }
}

/// Public methods for BlinkController
impl BlinkController {
    /// Creates new blink controller with default settings.
    ///
    /// # Details
    /// Initializes controller with LED off using Default trait.
    /// Ready to start blinking immediately.
    ///
    /// # Returns
    /// * `Self` - New BlinkController instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates new blink controller with custom delay.
    ///
    /// # Details
    /// Initializes controller with specified delay, clamped to valid range.
    /// LED starts in off state.
    ///
    /// # Arguments
    /// * `delay_ms` - Desired blink delay in milliseconds
    ///
    /// # Returns
    /// * `Self` - New BlinkController with configured delay
    #[allow(dead_code)]
    pub fn with_delay(delay_ms: u64) -> Self {
        Self {
            state: LedState::Off,
            delay_ms: clamp_delay(delay_ms),
            toggle_count: 0,
        }
    }

    /// Toggles LED state and returns new state.
    ///
    /// # Details
    /// Transitions LED from On to Off or Off to On.
    /// Increments toggle counter for tracking.
    ///
    /// # Returns
    /// * `LedState` - New LED state after toggle
    pub fn toggle(&mut self) -> LedState {
        self.state = match self.state {
            LedState::On => LedState::Off,
            LedState::Off => LedState::On,
        };
        self.toggle_count += 1;
        self.state
    }

    /// Returns current LED state.
    ///
    /// # Returns
    /// * `LedState` - Current LED state
    #[allow(dead_code)]
    pub fn state(&self) -> LedState {
        self.state
    }

    /// Returns current blink delay.
    ///
    /// # Returns
    /// * `u64` - Delay in milliseconds
    pub fn delay_ms(&self) -> u64 {
        self.delay_ms
    }

    /// Returns total toggle count.
    ///
    /// # Returns
    /// * `u64` - Number of toggles
    #[allow(dead_code)]
    pub fn toggle_count(&self) -> u64 {
        self.toggle_count
    }

    /// Checks if LED is currently on.
    ///
    /// # Returns
    /// * `bool` - true if LED is on
    #[allow(dead_code)]
    pub fn is_on(&self) -> bool {
        self.state == LedState::On
    }

    /// Checks if LED is currently off.
    ///
    /// # Returns
    /// * `bool` - true if LED is off
    #[allow(dead_code)]
    pub fn is_off(&self) -> bool {
        self.state == LedState::Off
    }

    /// Sets new blink delay, clamped to valid range.
    ///
    /// # Arguments
    /// * `delay_ms` - New delay in milliseconds
    #[allow(dead_code)]
    pub fn set_delay(&mut self, delay_ms: u64) {
        self.delay_ms = clamp_delay(delay_ms);
    }
}

/// Clamps delay value to valid range.
///
/// # Details
/// Ensures delay falls within MIN_BLINK_DELAY_MS and MAX_BLINK_DELAY_MS.
///
/// # Arguments
/// * `delay_ms` - Delay to clamp
///
/// # Returns
/// * `u64` - Clamped delay value
#[allow(dead_code)]
fn clamp_delay(delay_ms: u64) -> u64 {
    delay_ms.clamp(MIN_BLINK_DELAY_MS, MAX_BLINK_DELAY_MS)
}

/// Converts LedState to boolean for GPIO control.
///
/// # Details
/// Maps On state to true (high), Off state to false (low).
///
/// # Arguments
/// * `state` - LED state to convert
///
/// # Returns
/// * `bool` - true for On, false for Off
pub fn state_to_level(state: LedState) -> bool {
    matches!(state, LedState::On)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_controller() {
        let ctrl = BlinkController::new();
        assert_eq!(ctrl.state(), LedState::Off);
        assert_eq!(ctrl.delay_ms(), BLINK_DELAY_MS);
        assert_eq!(ctrl.toggle_count(), 0);
    }

    #[test]
    fn test_with_delay() {
        let ctrl = BlinkController::with_delay(1000);
        assert_eq!(ctrl.delay_ms(), 1000);
    }

    #[test]
    fn test_with_delay_clamps_low() {
        let ctrl = BlinkController::with_delay(1);
        assert_eq!(ctrl.delay_ms(), MIN_BLINK_DELAY_MS);
    }

    #[test]
    fn test_with_delay_clamps_high() {
        let ctrl = BlinkController::with_delay(100000);
        assert_eq!(ctrl.delay_ms(), MAX_BLINK_DELAY_MS);
    }

    #[test]
    fn test_toggle_off_to_on() {
        let mut ctrl = BlinkController::new();
        assert_eq!(ctrl.toggle(), LedState::On);
        assert!(ctrl.is_on());
    }

    #[test]
    fn test_toggle_on_to_off() {
        let mut ctrl = BlinkController::new();
        ctrl.toggle();
        assert_eq!(ctrl.toggle(), LedState::Off);
        assert!(ctrl.is_off());
    }

    #[test]
    fn test_toggle_count_increments() {
        let mut ctrl = BlinkController::new();
        ctrl.toggle();
        ctrl.toggle();
        ctrl.toggle();
        assert_eq!(ctrl.toggle_count(), 3);
    }

    #[test]
    fn test_is_on_false_initially() {
        let ctrl = BlinkController::new();
        assert!(!ctrl.is_on());
    }

    #[test]
    fn test_is_off_true_initially() {
        let ctrl = BlinkController::new();
        assert!(ctrl.is_off());
    }

    #[test]
    fn test_set_delay() {
        let mut ctrl = BlinkController::new();
        ctrl.set_delay(250);
        assert_eq!(ctrl.delay_ms(), 250);
    }

    #[test]
    fn test_set_delay_clamps() {
        let mut ctrl = BlinkController::new();
        ctrl.set_delay(1);
        assert_eq!(ctrl.delay_ms(), MIN_BLINK_DELAY_MS);
    }

    #[test]
    fn test_state_to_level_on() {
        assert!(state_to_level(LedState::On));
    }

    #[test]
    fn test_state_to_level_off() {
        assert!(!state_to_level(LedState::Off));
    }

    #[test]
    fn test_clamp_delay_within_range() {
        assert_eq!(clamp_delay(500), 500);
    }

    #[test]
    fn test_clamp_delay_below_min() {
        assert_eq!(clamp_delay(1), MIN_BLINK_DELAY_MS);
    }

    #[test]
    fn test_clamp_delay_above_max() {
        assert_eq!(clamp_delay(100000), MAX_BLINK_DELAY_MS);
    }
}
