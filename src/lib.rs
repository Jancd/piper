//! Async pipes, channels, mutexes, and more.

#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]

mod chan;
mod event;
mod lock;
mod mutex;
mod pipe;
mod shared;

pub use chan::{chan, Receiver, Sender};
pub use lock::{Lock, LockGuard};
pub use mutex::{Mutex, MutexGuard};
pub use pipe::{pipe, Reader, Writer};
pub use shared::Shared;
