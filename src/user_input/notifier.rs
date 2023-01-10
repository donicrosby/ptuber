use super::DrainFilter;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex, Weak};

#[derive(Debug)]
pub struct CallbackGuard<Callback: ?Sized> {
    pub(crate) _callback: Arc<Callback>,
}

pub type NotifierCallback<E> = dyn Fn(&E) + Sync + Send + 'static;
pub struct Notifier<E> {
    subscribers: Mutex<Vec<Weak<NotifierCallback<E>>>>,
}

impl<E> Notifier<E> {
    pub fn new() -> Self {
        Self {
            subscribers: Mutex::new(Vec::new()),
        }
    }

    pub fn register<F>(&mut self, callback: Arc<NotifierCallback<E>>) {
        if let Ok(mut subscribers) = self.subscribers.lock() {
            let callback = Arc::downgrade(&callback);
            subscribers.push(callback);
        }
    }

    pub fn notify(&self, event: &E) {
        if let Ok(mut callbacks) = self.subscribers.lock() {
            DrainFilter::drain_filter(callbacks.deref_mut(), |callback| {
                callback.upgrade().is_none()
            });
            for callback in callbacks.iter() {
                if let Some(callback) = callback.upgrade() {
                    callback(event)
                }
            }
        }
    }
}
