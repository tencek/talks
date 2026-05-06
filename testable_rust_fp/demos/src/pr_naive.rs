//! The naive implementation of a pull request state machine

#[derive(Debug, Clone)]
pub struct PullRequest {
    pub repo: String,
    pub src_branch: String,
    pub dst_branch: String,
    pub state: State,
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
        self.state = State::Open;
    }

    pub fn approve(&mut self, approver: String, message: String) {
        self.state = State::Approved(Approval { approver, message });
    }

    pub fn close(&mut self, reason: String) {
        self.state = State::Closed(Reason { message: reason });
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
mod tests {
    use super::*;

    #[test]
    fn test_new_pr_is_in_new_state() {
        let pr = PullRequest::new("repo".to_string(), "src".to_string(), "dst".to_string());
        assert_eq!(pr.state, State::New);
    }

    #[test]
    fn test_pr_open_opens_the_pr() {
        let mut pr = PullRequest::new("repo".to_string(), "src".to_string(), "dst".to_string());
        pr.open();
        assert_eq!(pr.state, State::Open);
    }

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
}
