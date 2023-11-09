#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub mod network;
pub mod transport;
pub mod session;
pub mod presentation;
pub mod acse;
pub mod rtse;

pub type OsiSelector = Vec<u8>;

/// It is assumed in this library that a pair of remote and local selectors can
/// uniquely identify a network connection, at least. This may be changed in the
/// future.
///
/// Remote comes first in this tuple so that comparisons can short-circuit
/// faster, since remote selectors are higher cardinality.
pub type RemoteAndLocalSels = (OsiSelector, OsiSelector);
pub type RemoteAndLocalSelRefs <'a> = (&'a OsiSelector, &'a OsiSelector);
pub type WakeTime = Option<std::time::SystemTime>;

/// A result returned from a service primitive invocation.
pub type ServiceResult = std::io::Result<WakeTime>;

pub trait OSILayer {}
pub trait OSIService {}
pub trait OSIEntity {}
pub trait OSIConnection {}
pub trait OSIConnectionOrientedService : OSIService {}
pub trait OSIConnectionOrientedLayer : OSILayer {}
pub trait OSIConnectionOrientedEntity : OSIEntity {}
pub trait OSIApplicationServiceElement {}
pub trait OSIApplicationServiceObject {}
pub trait OSIApplicationServiceObjectInvocation {}
pub trait OSIApplicationServiceElementInvocation : OSIApplicationServiceObjectInvocation {}

/// NOTE: X.217 says: "A one-to-one correspondence exists between an application-association and a presentation-connection."
pub trait OSIApplicationAssociation {}
