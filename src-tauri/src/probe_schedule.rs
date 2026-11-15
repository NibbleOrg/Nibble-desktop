//! Single pending delayed open; shared by the native menu and its callback.
#[derive(Default)]
pub struct PendingOpen {
    generation: u64,
    pending: Option<u64>,
}

impl PendingOpen {
    pub fn request(&mut self) -> Option<u64> {
        if self.pending.is_some() {
            return None;
        }
        self.generation += 1;
        self.pending = Some(self.generation);
        self.pending
    }

    pub fn complete(&mut self, request: u64) -> bool {
        if self.pending == Some(request) {
            self.pending = None;
            true
        } else {
            false
        }
    }

    pub fn cancel(&mut self) {
        self.pending = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_shortcuts_do_not_postpone_first_open() {
        let mut schedule = PendingOpen::default();
        let first = schedule.request().unwrap();
        for _ in 0..10 {
            assert_eq!(schedule.request(), None, "retry must not restart the timer");
        }
        assert!(schedule.complete(first), "first deadline must still open");
        assert!(
            !schedule.complete(first),
            "a deadline is consumed only once"
        );
    }

    #[test]
    fn close_then_reopen_rejects_old_callback_without_canceling_new_one() {
        let mut schedule = PendingOpen::default();
        let old = schedule.request().unwrap();
        schedule.cancel();
        let new = schedule.request().unwrap();
        assert!(!schedule.complete(old));
        assert!(schedule.complete(new));
    }
}

/// The native window delivers focus events asynchronously, including after hide.
pub fn on_focus_lost<E>(
    visible_now: impl FnOnce() -> Result<bool, E>,
    close: impl FnOnce() -> Result<(), E>,
) -> Result<(), E> {
    if visible_now()? {
        close()?;
    }
    Ok(())
}

#[cfg(test)]
mod focus_tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn hide_event_after_esc_cannot_cancel_the_next_open() {
        let mut pending = PendingOpen::default();
        pending.request();
        pending.cancel(); // Esc hides the window, queuing Focused(false).
        let next = pending.request().unwrap(); // User presses Cmd+1 again.
        on_focus_lost(
            || Ok::<_, ()>(false),
            || {
                pending.cancel();
                Ok(())
            },
        )
        .unwrap();
        assert!(
            pending.complete(next),
            "old hide event canceled the next open"
        );
    }

    #[test]
    fn actual_external_blur_still_closes_the_visible_window() {
        let hidden = Cell::new(false);
        on_focus_lost(
            || Ok::<_, ()>(true),
            || {
                hidden.set(true);
                Ok(())
            },
        )
        .unwrap();
        assert!(hidden.get());
    }
}
