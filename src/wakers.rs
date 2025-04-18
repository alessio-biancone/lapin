use std::{
    sync::{Arc, Mutex, MutexGuard},
    task::Waker,
};

type Inner = Vec<Waker>;

#[derive(Default, Clone)]
pub(crate) struct Wakers(Arc<Mutex<Inner>>);

impl Wakers {
    pub(crate) fn register(&self, waker: &Waker) {
        let mut inner = self.lock_inner();
        for w in inner.iter() {
            if w.will_wake(waker) {
                return;
            }
        }
        inner.push(waker.clone());
    }

    pub(crate) fn wake(&self) {
        for w in self.lock_inner().drain(..) {
            w.wake();
        }
    }

    fn lock_inner(&self) -> MutexGuard<'_, Inner> {
        self.0.lock().unwrap_or_else(|e| e.into_inner())
    }
}
