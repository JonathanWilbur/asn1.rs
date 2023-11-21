#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::net::IpAddr;

use network::{NSProvider, NSUser};
use transport::{COTSProvider, COTSUser};

pub mod network;
pub mod transport;
pub mod session;
pub mod presentation;
pub mod acse;
pub mod rtse;
pub mod stack;

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
/// ITU X.207, Section 5.4.4 An AE is itself an ASO.
pub trait OSIApplicationServiceElement : OSIApplicationServiceObject {}
pub trait OSIApplicationServiceObject {}
pub trait OSIApplicationServiceObjectInvocation {}
pub trait OSIApplicationServiceElementInvocation : OSIApplicationServiceObjectInvocation {}

// NOTE: X.217 says: "A one-to-one correspondence exists between an application-association and a presentation-connection."
// pub trait OSIApplicationAssociation {}


// pub struct Stack <N, T, S>
//     where
//         N: NSProvider, // TODO: Does the network even need to be generic?
//         T: NSUser<N> + COTSProvider<S>,
//         S: COTSUser<T>
//     {
//     network: N,
//     transport: T,
//     session: S,
// }

// X.224 can run over ITOT, XOT, TP4 etc.
// X.225 always runs over X.224
// X.226 always runs over X.225
// I am kind of thinking of using hard-coded types for all layers, except the
// network layer, which will use dynamic dispatch.
// Something to think about: users may want to use this library to build
// more efficient stacks than the default ones, perhaps by using static
// dispatch + Tokio TCP streams.


// pub trait OSI


pub type PeerId = IpAddr;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PortSpace {
    TCP,
    UDP,
    SCTP,
}

/// This type uniquely identifies a network connection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NetworkConnId2 {
    pub addr: PeerId,
    pub port: Option<u16>,
    pub portspace: PortSpace,
}
