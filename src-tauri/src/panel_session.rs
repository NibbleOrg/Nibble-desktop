//! Content-free lifecycle. All callbacks carry an epoch; closing invalidates it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ticket {
    pub session: u32,
    pub request: u32,
}
#[derive(Default)]
pub struct Session {
    serial: u32,
    active: Option<u32>,
    request: u32,
    pending: Option<Ticket>,
}
impl Session {
    pub fn current(&self) -> Option<u32> {
        self.active
    }
    pub fn open(&mut self) -> u32 {
        if let Some(active) = self.active {
            return active;
        }
        self.serial = self
            .serial
            .checked_add(1)
            .expect("session counter exhausted");
        self.active = Some(self.serial);
        self.serial
    }
    pub fn close(&mut self) {
        self.active = None;
        self.pending = None;
    }
    pub fn begin(&mut self, session: u32) -> Option<Ticket> {
        if self.active != Some(session) || self.pending.is_some() {
            return None;
        }
        self.request = self
            .request
            .checked_add(1)
            .expect("request counter exhausted");
        let ticket = Ticket {
            session,
            request: self.request,
        };
        self.pending = Some(ticket);
        Some(ticket)
    }
    pub fn finish(&mut self, ticket: Ticket) -> bool {
        if self.active != Some(ticket.session) || self.pending != Some(ticket) {
            return false;
        }
        self.pending = None;
        true
    }
}
/// Preserve the captured target before hide can synchronously deliver blur.
/// Failure still consumes that target, so recovery never repeatedly steals focus.
pub fn close_with_restore<T, E>(
    take_original: impl FnOnce() -> Option<T>,
    hide: impl FnOnce() -> Result<(), E>,
    restore: impl FnOnce(T) -> Result<(), E>,
    should_restore: bool,
) -> Result<(), E> {
    let original = take_original();
    hide()?;
    if should_restore {
        if let Some(original) = original {
            restore(original)?;
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn close_and_reopen_rejects_old_resize_and_completion() {
        let mut state = Session::default();
        let first = state.open();
        let request = state.begin(first).unwrap();
        state.close();
        let second = state.open();
        assert_ne!(first, second);
        assert!(!state.finish(request));
        assert!(state.begin(first).is_none());
        assert_eq!(state.current(), Some(second));
    }
    #[test]
    fn duplicate_transition_does_not_extend_deadline() {
        let mut state = Session::default();
        let epoch = state.open();
        let first = state.begin(epoch).unwrap();
        assert!(state.begin(epoch).is_none());
        assert!(state.finish(first));
        assert!(!state.finish(first));
        assert!(state.begin(epoch).is_some());
    }
    #[test]
    fn timed_out_transition_cannot_finish_a_new_transition() {
        let mut state = Session::default();
        let epoch = state.open();
        let expired = state.begin(epoch).unwrap();
        assert!(state.finish(expired));
        let current = state.begin(epoch).unwrap();
        assert!(!state.finish(expired));
        assert!(state.finish(current));
    }
    #[test]
    fn repeated_open_preserves_session() {
        let mut state = Session::default();
        let first = state.open();
        assert_eq!(first, state.open());
        state.close();
        assert_eq!(state.current(), None);
    }
}

#[cfg(test)]
mod close_tests {
    use super::*;
    use std::cell::RefCell;
    #[test]
    fn close_takes_target_before_hide_events_and_restores_once() {
        let events = RefCell::new(Vec::new());
        let result = close_with_restore(
            || {
                events.borrow_mut().push("take");
                Some(7)
            },
            || {
                events.borrow_mut().push("hide");
                Ok::<_, &str>(())
            },
            |target| {
                assert_eq!(target, 7);
                events.borrow_mut().push("restore");
                Ok(())
            },
            true,
        );
        assert_eq!(result, Ok(()));
        assert_eq!(*events.borrow(), ["take", "hide", "restore"]);
    }
    #[test]
    fn hide_failure_never_restores_or_reuses_original_target() {
        let mut original = Some(7);
        let result = close_with_restore(
            || original.take(),
            || Err("hide failed"),
            |_| panic!("must not restore"),
            true,
        );
        assert_eq!(result, Err("hide failed"));
        assert!(original.is_none());
    }
    #[test]
    fn restore_failure_cannot_leave_an_active_session() {
        let mut session = Session::default();
        session.open();
        let hidden = RefCell::new(false);
        let result = close_with_restore(
            || {
                session.close();
                Some(7)
            },
            || {
                *hidden.borrow_mut() = true;
                Ok(())
            },
            |_| Err("system rejected restoration"),
            true,
        );
        assert!(result.is_err());
        assert!(*hidden.borrow());
        assert_eq!(session.current(), None);
    }
    #[test]
    fn external_click_or_missing_original_does_not_activate_another_app() {
        assert_eq!(
            close_with_restore(
                || Some(7),
                || Ok::<_, &str>(()),
                |_| panic!("external click must not steal focus"),
                false
            ),
            Ok(())
        );
        assert_eq!(
            close_with_restore(
                || None::<u32>,
                || Ok::<_, &str>(()),
                |_| panic!("no replacement target"),
                true
            ),
            Ok(())
        );
    }
}
