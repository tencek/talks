---
marp: true
footer: github.com/tencek/talks
---

<!-- _footer: "" -->

![bg 50%](./img/ferris.svg)

------------------------------------------------------------------

## Why do we all write tests that...

(*Select all that apply to your tests*)

☑ Are hard to maintain
☑ Are hard to understand
☑ Are hard to reproduce failures
☑ Break during refactoring
☑ Have lots of duplicated code
☑ Create unnecessary stress
☑ Other (specify)

------------------------------------------------------------------

## Because...

(*Select your favourite excuse*)

◯ It's a natural inevitability, there's no other way
◯ We like it that way (if it was hard to write, it should be hard to understand)
◯ Our production code is not test-friendly
◯ Other (specify)

------------------------------------------------------------------

![bg right:45%](./img/me.jpg)

# Test-friendly (Rust) code – FP approach

Pavel Kučera (*1983)

- C++ developer since ~2006
- C#/.NET developer since ~2015
- Fan of FP since ~2018 (F#)
- Fan of Rust since 2023

------------------------------------------------------------------

## How to make our production code more test-friendly?

We, FP enthusiasts, believe that the key is to make our production code more functional :-)

What does that mean?  

------------------------------------------------------------------

## What is a Function?

```text
      +-------+
      |       |
x --> |   F   | --> y 
      |       |
      +-------+
```

- A "box"
- Single input, Single output
- Every input produces an output (**totality**)
- Same input => same output (**stateless**)
- No side effects (**immutability**)

Quite an limitation! (For good reasons, we (FPs) believe)

------------------------------------------------------------------

## Example - PullRequest - naive implementation

```rust
#[derive(Debug, Clone)]
pub struct PullRequest {
    pub repo: String,
    pub src_branch: String,
    pub dst_branch: String,
    pub state: State,
}
```

------------------------------------------------------------------

## Example - PullRequest - naive implementation - State

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    New,
    Open,
    Approved(Approval),
    Closed(Reason),
}
```

------------------------------------------------------------------

## Example - PullRequest - naive implementation - Approval

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Approval {
    pub approver: String,
    pub message: String,
}
```

------------------------------------------------------------------

## Example - PullRequest - naive implementation - Reason

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reason {
    pub message: String,
}
```

------------------------------------------------------------------

## Example - PullRequest - naive implementation - impl new

```rust
impl PullRequest {
    pub fn new(repo: String, src_branch: String, dst_branch: String) -> Self {
        PullRequest {
            repo,
            src_branch,
            dst_branch,
            state: State::New,
        }
    }
}
```

------------------------------------------------------------------

## Example - PullRequest - naive implementation - impl open

```rust
impl PullRequest {
    pub fn open(&mut self) {
        self.state = State::Open;
    }
}
```

------------------------------------------------------------------

## Example - PullRequest - naive implementation - impl approve

```rust
impl PullRequest {
    pub fn approve(&mut self, approver: String, message: String) {
        self.state = State::Approved(Approval { approver, message });
    }
}
```

------------------------------------------------------------------

## Example - PullRequest - naive implementation - impl close

```rust
impl PullRequest {
    pub fn close(&mut self, reason: String) {
        self.state = State::Closed(Reason { message: reason });
    }
}
```

------------------------------------------------------------------

## Example - PullRequest - naive implementation - tests - new

```rust
    #[test]
    fn test_new_pr_is_in_new_state() {
        let pr = PullRequest::new("repo".to_string(), "src".to_string(), "dst".to_string());
        assert_eq!(pr.state, State::New);
    }
```

------------------------------------------------------------------

## Example - PullRequest - naive implementation - tests - open

```rust
    #[test]
    fn test_pr_open_opens_the_pr() {
        let mut pr = PullRequest::new("repo".to_string(), "src".to_string(), "dst".to_string());
        pr.open();
        assert_eq!(pr.state, State::Open);
    }
```

------------------------------------------------------------------

## Example - PullRequest - naive implementation - tests - approve

```rust
    #[test]
    fn test_pr_approve_approves_the_pr() {
        let mut pr = PullRequest::new("repo".to_string(), "src".to_string(), "dst".to_string());
        pr.approve("Alice".to_string(), "LGTM".to_string());
        assert_eq!(
            pr.state,
            State::Approved(Approval {
                approver: "Alice".to_string(),
                message: "LGTM".to_string()
            })
        );
    }
```

------------------------------------------------------------------

## Example - PullRequest - naive implementation - tests - close

```rust
    #[test]
    fn test_pr_close_closes_the_pr() {
        let mut pr = PullRequest::new("repo".to_string(), "src".to_string(), "dst".to_string());
        pr.close("No longer needed".to_string());
        assert_eq!(
            pr.state,
            State::Closed(Reason {
                message: "No longer needed".to_string()
            })
        );
    }
```

------------------------------------------------------------------

```
running 4 tests
test pr1::tests::test_new_pr_is_in_new_state ... ok
test pr1::tests::test_pr_close_closes_the_pr ... ok
test pr1::tests::test_pr_open_opens_the_pr ... ok
test pr1::tests::test_pr_approve_approves_the_pr ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

------------------------------------------------------------------

## Example - PullRequest - naive implementation - problems?

☐ hard to maintain
☐ hard to understand
☐ hard to reproduce failures
☐ Break during refactoring
☐ Have lots of duplicated code
☐ Create unnecessary stress
☐ Other (specify)

------------------------------------------------------------------

## Example - PullRequest - naive implementation - problems!

☑ hard to maintain
☐ hard to understand
☑ hard to reproduce failures
☐ Break during refactoring
☑ Have lots of duplicated code
☑ Create unnecessary stress
☑ Other (specify)

------------------------------------------------------------------

## Example - PullRequest - naive implementation - can we do better?

------------------------------------------------------------------

## Example - PullRequest - FP implementation

```rust
#[derive(Debug, Clone)]
pub struct PullRequestStateMachine;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Open,
    Approve { approver: String, message: String },
    Close { reason: String },
}
```

------------------------------------------------------------------

## Example - PullRequest - FP implementation - impl

```rust
impl PullRequestStateMachine {
    pub fn apply(state: State, action: Action) -> State {
        match (state, action) {
            (State::New, Action::Open) => State::Open,
            (State::Open, Action::Approve { approver, message }) => {
                State::Approved(Approval { approver, message })
            }
            (State::Open, Action::Close { reason }) => State::Closed(Reason { message: reason }),
        }
    }
}
```

------------------------------------------------------------------

## Example - PullRequest - FP implementation - cargo build

```bash
> cargo build


error[E0004]: non-exhaustive patterns: `(pr2::State::Approved(_), _)` and `(pr2::State::Closed(_), _)` not covered
  --> src\pr2.rs:31:15
   |
31 |         match (state, action) {
   |               ^^^^^^^^^^^^^^^ patterns `(pr2::State::Approved(_), _)` and `(pr2::State::Closed(_), _)` not covered
   |
   = note: the matched value is of type `(pr2::State, pr2::Action)`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern, a match arm with 
multiple or-patterns as shown, or multiple match arms
   |
36 ~             (State::Open, Action::Close { reason }) => State::Closed(Reason { message: reason }),
37 ~             (pr2::State::Approved(_), _) | (pr2::State::Closed(_), _) => todo!(),
   |
```

------------------------------------------------------------------

## Example - PullRequest - FP implementation - panic!

```rust
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
```

------------------------------------------------------------------

## Example - PullRequest - FP implementation - error handling

```rust
#[derive(Error, Debug, Clone, PartialEq)]
pub enum PullRequestStateMachineError {
    #[error("Invalid transition from state {0:?} with action {1:?}")]
    InvalidTransition(State, Action),
}
```

------------------------------------------------------------------

## Example - PullRequest - FP implementation - apply - Result

```rust
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
}
```

------------------------------------------------------------------

## Example - PullRequest - FP implementation - apply_exhaustive

```rust
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
```

------------------------------------------------------------------

```rust
            // state: Approved
            (State::Approved(_approval), Action::Close { reason }) => {
                Ok(State::Closed(Reason { message: reason }))
            }
            (State::Approved(approval), action @ (Action::Open | Action::Approve { .. }) ) => {
                Err(PullRequestStateMachineError::InvalidTransition(
                    State::Approved(approval.clone()),
                    action,
                ))
            }

            // state: Closed
            (State::Closed(_reason), Action::Open) => Ok(State::Open),
            (State::Closed(reason), action @ (Action::Approve { .. } | Action::Close { .. }) ) => {
                Err(PullRequestStateMachineError::InvalidTransition(
                    State::Closed(reason.clone()),
                    action,
                ))
            }
        }
    }
```

------------------------------------------------------------------

## Example - PullRequest - FP implementation - PullRequest API

```rust
impl PullRequest {

    pub fn open(&mut self) {
        self.apply(Action::Open);
    }

    pub fn approve(&mut self, approver: String, message: String) {
        self.apply(Action::Approve { approver, message });
    }

    pub fn close(&mut self, reason: String) {
        self.apply(Action::Close { reason });
    }
}
```

------------------------------------------------------------------

## Example - PullRequest - FP implementation - PullRequest apply

```rust
impl PullRequest {
     fn apply(&mut self, action: Action) {
        debug!("Applying action {:?} in state {:?}", action, self.state);

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
```

------------------------------------------------------------------

## Example - PullRequest - FP implementation - tests

```rust

#[cfg(test)]
mod tests {

???

}

```

------------------------------------------------------------------

## Why do we bother with tests?

- Find regressions sooner ("shift left")
- Be more confident when refactoring
- Help us develop new functionality
- Examples how to use API of our code
- Other (specify)

------------------------------------------------------------------

## Why do I bother with tests?

They forces me to think about the production code

- Edge cases
- Error handling
- API

------------------------------------------------------------------

## Why to bother with tests of PullRequestStateMachine?

☑ Edge cases (enforced by the compiler)
☑ Error handling (enforced by the compiler)
☑ API (used by `fn PullRequest::apply`)

### How would the tests look like anyway?

- _parametrized  tests_ covering all the combinations of states and actions => Which is what we already do
in the production code (in `apply_exhaustive`)!
- The impure (mutable) part of the code (`fn PullRequest::apply`) – just a thin wrapper around the pure (functional) part, so there is not much to test there.

------------------------------------------------------------------

## Conclusion

### The best tests are those that we don't need to write!

FP + Rust can halp us to write code that is easy to reason about
even without tests.

### TLDR
Separate the logic into pure functions.