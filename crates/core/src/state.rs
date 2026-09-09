use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppState {
    Idle,
    Scheduled,
    PreparingBreak,
    Countdown,
    Playing,
    Paused,
    Results,
    ReturningToWork,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum StateTransitionError {
    #[error("Invalid state transition from {from:?} to {to:?}")]
    InvalidTransition { from: AppState, to: AppState },
}

pub struct StateMachine {
    current: AppState,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            current: AppState::Idle,
        }
    }

    pub fn current(&self) -> AppState {
        self.current
    }

    pub fn transition(&mut self, next: AppState) -> Result<(), StateTransitionError> {
        let valid = match (self.current, next) {
            // Normal break flow
            (AppState::Idle, AppState::Scheduled) => true,
            (AppState::Idle, AppState::PreparingBreak) => true,
            (AppState::Idle, AppState::Countdown) => true,
            (AppState::Scheduled, AppState::PreparingBreak) => true,
            (AppState::Scheduled, AppState::Countdown) => true,
            (AppState::PreparingBreak, AppState::Countdown) => true,
            (AppState::Countdown, AppState::Playing) => true,
            (AppState::Playing, AppState::Paused) => true,
            (AppState::Paused, AppState::Playing) => true,
            (AppState::Playing, AppState::Results) => true,
            (AppState::Paused, AppState::Results) => true,
            (AppState::Results, AppState::ReturningToWork) => true,
            (AppState::ReturningToWork, AppState::Idle) => true,
            (AppState::ReturningToWork, AppState::Scheduled) => true,

            // Emergency Escape / Cancellation from anywhere to ReturningToWork
            (AppState::Countdown, AppState::ReturningToWork) => true,
            (AppState::Playing, AppState::ReturningToWork) => true,
            (AppState::Paused, AppState::ReturningToWork) => true,
            (AppState::PreparingBreak, AppState::Idle) => true,

            // Re-triggering or resetting
            (current, target) if current == target => true,
            _ => false,
        };

        if valid {
            self.current = next;
            Ok(())
        } else {
            Err(StateTransitionError::InvalidTransition {
                from: self.current,
                to: next,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_break_flow() {
        let mut sm = StateMachine::new();
        assert_eq!(sm.current(), AppState::Idle);

        sm.transition(AppState::Scheduled).unwrap();
        sm.transition(AppState::PreparingBreak).unwrap();
        sm.transition(AppState::Countdown).unwrap();
        sm.transition(AppState::Playing).unwrap();
        sm.transition(AppState::Results).unwrap();
        sm.transition(AppState::ReturningToWork).unwrap();
        sm.transition(AppState::Idle).unwrap();
    }

    #[test]
    fn test_emergency_escape_during_play() {
        let mut sm = StateMachine::new();
        sm.transition(AppState::Countdown).unwrap();
        sm.transition(AppState::Playing).unwrap();
        // Esc hit: emergency transition
        sm.transition(AppState::ReturningToWork).unwrap();
        assert_eq!(sm.current(), AppState::ReturningToWork);
        sm.transition(AppState::Idle).unwrap();
        assert_eq!(sm.current(), AppState::Idle);
    }

    #[test]
    fn test_invalid_transition() {
        let mut sm = StateMachine::new();
        let err = sm.transition(AppState::Playing).unwrap_err();
        assert!(matches!(
            err,
            StateTransitionError::InvalidTransition { .. }
        ));
    }
}
