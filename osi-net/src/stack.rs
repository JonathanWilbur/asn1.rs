use crate::ServiceResult;
use crate::network::{NSProvider, NetworkConnId};
use crate::presentation::X226PresentationConnection;
use crate::session::X225SessionConnection;
use crate::transport::{X224TransportConnection, TransportRef};
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct TransportConnInfo {
    /// The application assocation.
    pub stack: Weak<RefCell<OSIApplicationAssociation>>,

    /// The network connections used by this transport connection.
    pub netconns: Vec<Rc<RefCell<dyn NSProvider>>>,
}

pub struct OSISystemStack {
    pub netconns: HashMap<NetworkConnId, Weak<RefCell<dyn NSProvider>>>,
    pub associations: HashMap<TransportRef, OSIApplicationAssociation>,
    pub next_ref: TransportRef,
}

pub struct OSIApplicationAssociation {
    pub transport_ref: TransportRef,
    // pub network_layer: Rc<RefCell<dyn NSProvider>>,
    pub transport: X224TransportConnection,
    pub session: X225SessionConnection,
    pub presentation: X226PresentationConnection,
}

impl OSIApplicationAssociation {

    pub fn receive_nsdu (&mut self, system: Rc<RefCell<OSISystemStack>>, nsdu: &[u8]) -> ServiceResult {
        Ok(None)
    }

}

impl OSISystemStack {

    pub fn associate (&mut self) -> OSIApplicationAssociation {
        let tref = self.next_ref;
        self.next_ref += 1;
        OSIApplicationAssociation {
            transport_ref: tref,
            // network_layer:
            transport: X224TransportConnection::default(),
            session: X225SessionConnection::default(),
            presentation: X226PresentationConnection::default(),
        }
    }

}

impl Default for OSISystemStack {

    fn default() -> Self {
        OSISystemStack {
            netconns: HashMap::new(),
            associations: HashMap::new(),
            next_ref: 5,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    // #[tokio::test]
    // async fn works_like_i_think_it_will () -> std::io::Result<()> {
    //     let listener = TcpListener::bind("127.0.0.1:6142").await?;
    //     let stack = OSISystemStack::default();
    //     loop {
    //         let (mut socket, _) = listener.accept().await?;
    //         let assn = stack.associate();

    //         tokio::spawn(async move {
    //             let mut buf = vec![0; 1024];

    //             loop {
    //                 match socket.read(&mut buf).await {
    //                     // Return value of `Ok(0)` signifies that the remote has
    //                     // closed
    //                     Ok(0) => return,
    //                     Ok(n) => {
    //                         assn.receive_nsdu(stack, buf.as_slice());
    //                         // Copy the data back to socket
    //                         if socket.write_all(&buf[..n]).await.is_err() {
    //                             // Unexpected socket error. There isn't much we can
    //                             // do here so just stop processing.
    //                             return;
    //                         }
    //                     }
    //                     Err(_) => {
    //                         // Unexpected socket error. There isn't much we can do
    //                         // here so just stop processing.
    //                         return;
    //                     }
    //                 }
    //             }
    //         });
    //     }
    // }

}
