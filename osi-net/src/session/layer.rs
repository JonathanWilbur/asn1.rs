use crate::session::X225SessionEntity;

/// For now, we just have a single-entity layer. This may change in the future,
/// but I see no reason to. The concept of multiple N-entities per layer is
/// defined in the ITU specifications, but it is not clear what value it
/// provides, and even less clear how to implement it.
pub type OSIConnectionOrientedSessionLayer = X225SessionEntity;
