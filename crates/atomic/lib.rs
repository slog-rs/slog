//! Slog atomic switching drain
//!
//! Using `AtomicSwitch` it's possible to change drain
//! hierarchy behavior atomically, race-free, in runtime,
//! which can be useful eg. for turning debugging logging
//! in production.
//!
//! See [`slog` `signal.rs`
//! example](https://github.com/dpc/slog-rs/blob/master/examples/signal.rs)
#![warn(missing_docs)]

#[macro_use]
extern crate slog;
extern crate crossbeam;

use std::io;
use slog::*;
use std::sync::Arc;
use crossbeam::sync::ArcCell;

/// Handle to `AtomicSwitch` allowing switching it's sub-drain
pub struct AtomicSwitchCtrl(Arc<ArcCell<Box<Drain>>>);

/// Drain allowing atomically switching a sub-drain in runtime
pub struct AtomicSwitch(Arc<ArcCell<Box<Drain>>>);

impl AtomicSwitchCtrl {
    /// Create new `AtomicSwitchCtrl`
    pub fn new<D: Drain + 'static>(d: D) -> Self {
        let a = Arc::new(ArcCell::new(Arc::new(Box::new(d) as Box<Drain>)));
        AtomicSwitchCtrl(a)
    }

    /// Create new `AtomicSwitchCtrl` from an existing `Arc<...>`
    pub fn new_from_arc(d: Arc<ArcCell<Box<Drain>>>) -> Self {
        AtomicSwitchCtrl(d)
    }

    /// Get a `AtomicSwitch` drain controlled by this `AtomicSwitchCtrl`
    pub fn drain(&self) -> AtomicSwitch {
        AtomicSwitch(self.0.clone())
    }

    /// Set the drain
    pub fn set<D: Drain>(&self, drain: D) {
        let _ = self.0.set(Arc::new(Box::new(drain)));
    }

    /// Swap the existing drain with a new one
    pub fn swap(&self, drain: Arc<Box<Drain>>) -> Arc<Box<Drain>> {
        self.0.set(drain)
    }
}

impl Drain for AtomicSwitch {
    fn log(&self,
           info: &Record,
           logger_values: &OwnedKeyValueList)
           -> io::Result<()> {
        self.0.get().log(info, logger_values)
    }
}


