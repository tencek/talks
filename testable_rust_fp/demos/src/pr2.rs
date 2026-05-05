//! The naive implementation of a pull request state machine

use thiserror::Error;
use tracing::{debug, error};

#[derive(Debug, Clone)]
pub struct PullRequest {
    pub repo: String,
    pub src_branch: String,
    pub dst_branch: String,
    pub state: State,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Open,
    Approve { approver: String, message: String },
    Close { reason: String },
}

#[derive(Debug, Clone)]
pub struct PullRequestStateMachine;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum PullRequestStateMachineError {
    #[error("Invalid transition from state {0:?} with action {1:?}")]
    InvalidTransition(State, Action),
}

impl PullRequestStateMachine {
    /// This may panic!
    pub fn apply_unchecked(state: State, action: Action) -> State {
        match (state, action) {
            (State::New, Action::Open) => State::Open,
            (State::Open, Action::Approve { approver, message }) => {
                State::Approved(Approval { approver, message })
            }
            (State::Open, Action::Close { reason }) => State::Closed(Reason { message: reason }),
            (State::Approved(_approval), Action::Close { reason }) => {
                State::Closed(Reason { message: reason })
            }
            (State::Closed(_reason), Action::Open) => State::Open,
            (state, action) => panic!(
                "Invalid transition from state {:?} with action {:?}",
                state, action
            ),
        }
    }
}

impl PullRequestStateMachine {
    pub fn apply(state: &State, action: Action) -> Result<State, PullRequestStateMachineError> {
        match (state, action) {
            (State::New, Action::Open) => Ok(State::Open),
            (State::Open, Action::Approve { approver, message }) => {
                Ok(State::Approved(Approval { approver, message }))
            }
            (State::Open, Action::Close { reason }) => {
                Ok(State::Closed(Reason { message: reason }))
            }
            (State::Approved(_approval), Action::Close { reason }) => {
                Ok(State::Closed(Reason { message: reason }))
            }
            (State::Closed(_reason), Action::Open) => Ok(State::Open),
            (state, action) => Err(PullRequestStateMachineError::InvalidTransition(
                state.clone(),
                action,
            )),
        }
    }

    pub fn apply_exhaustive(
        state: &State,
        action: Action,
    ) -> Result<State, PullRequestStateMachineError> {
        match (state, action) {
            // state: New
            (State::New, Action::Open) => Ok(State::Open),
            (State::New, action @ (Action::Approve { .. } | Action::Close { .. })) => Err(
                PullRequestStateMachineError::InvalidTransition(State::New, action),
            ),

            // state: Open
            (State::Open, Action::Approve { approver, message }) => {
                Ok(State::Approved(Approval { approver, message }))
            }
            (State::Open, Action::Close { reason }) => {
                Ok(State::Closed(Reason { message: reason }))
            }
            (State::Open, action @ Action::Open) => Err(
                PullRequestStateMachineError::InvalidTransition(State::Open, action),
            ),

            // state: Approved
            (State::Approved(_approval), Action::Close { reason }) => {
                Ok(State::Closed(Reason { message: reason }))
            }
            (State::Approved(approval), action @ (Action::Open | Action::Approve { .. })) => {
                Err(PullRequestStateMachineError::InvalidTransition(
                    State::Approved(approval.clone()),
                    action,
                ))
            }

            // state: Closed
            (State::Closed(_reason), Action::Open) => Ok(State::Open),
            (State::Closed(reason), action @ (Action::Approve { .. } | Action::Close { .. })) => {
                Err(PullRequestStateMachineError::InvalidTransition(
                    State::Closed(reason.clone()),
                    action,
                ))
            }
        }
    }
}

impl PullRequest {
    pub fn new(repo: String, src_branch: String, dst_branch: String) -> Self {
        PullRequest {
            repo,
            src_branch,
            dst_branch,
            state: State::New,
        }
    }

    pub fn open(&mut self) {
        self.apply(Action::Open);
    }

    pub fn approve(&mut self, approver: String, message: String) {
        self.apply(Action::Approve { approver, message });
    }

    pub fn close(&mut self, reason: String) {
        self.apply(Action::Close { reason });
    }

    fn apply(&mut self, action: Action) {
        debug!("Performing action {:?} in state {:?}", action, self.state);

        match PullRequestStateMachine::apply(&self.state, action) {
            Ok(new_state) => self.state = new_state,
            Err(e) => {
                error!(
                    "Error applying action: {}, keeping state {:?}",
                    e, self.state
                );
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    New,
    Open,
    Approved(Approval),
    Closed(Reason),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Approval {
    pub approver: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reason {
    pub message: String,
}

#[cfg(test)]
mod tests {}
