#![allow(non_upper_case_globals)]
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::task::Waker;

use rose_transport::RosePDU;
use x690::X690Element;

pub mod idm;

// Taken from: https://rust-lang.github.io/async-book/02_execution/03_wakeups.html#applied-build-a-timer
pub struct FutureState {
    pub waker: Option<Waker>,
}

impl Default for FutureState {
    fn default() -> Self {
        FutureState { waker: None }
    }
}

// TODO: Move to ROSE library?
pub struct RoseStream<TransportType> {
    pub transport: TransportType,
    pub received_pdus: VecDeque<RosePDU<X690Element>>,
    /// Tokio says you can use the std Mutex in most cases.
    /// See: https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html#which-kind-of-mutex-should-you-use
    future_state: Arc<Mutex<FutureState>>,
}

impl<TransportType> RoseStream<TransportType> {
    pub fn new(transport: TransportType) -> Self {
        RoseStream {
            transport,
            received_pdus: VecDeque::new(), // TODO: Configurable capacity for PDUs.
            future_state: Arc::new(Mutex::new(FutureState::default())),
        }
    }
}

// pub struct DirectoryROSETransport<TransportType> {
//     transport
// }

// impl ROSETransmitter for DirectoryROSETransport {

// }

/*
I have these ideas for how to implement this:

- impl ROSETransmitter directly on IDMSocket
  - Connections would use Box<dyn ROSETransmitter + ROSEReceiver>
- Make DirectoryClient<TransportType>
  - Maybe this would be fine, but I don't see it working well once you have to
    keep all connections in a list.
- Make a TransportType enum, which will cost runtime performance, probably above
  and beyond Box-dyn
  - I actually think this might be alright, you could only use the enum when you
    need it. Function signatures could be like
    fn listen_for_requests(rose, idm)

- https://rust-unofficial.github.io/patterns/patterns/behavioural/strategy.html
- https://rust-unofficial.github.io/patterns/patterns/behavioural/strategy.html

I am not almost _certain_ that I have to use trait objects / dynamic dispatch.
The question is just whether. Functions will take the signature
fn attempt_bind (conn: &mut impl Association). (This is just syntactic sugar.)

- https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bound-syntax

*/

#[cfg(test)]
mod tests {

    use super::*;
    use ::idm::IdmStream;
    use rose_transport::{BindParameters, ROSEReceiver, ROSETransmitter};
    use std::net::ToSocketAddrs;
    use std::time::Duration;
    use tokio::net::TcpSocket;
    use tokio::time::sleep;
    use x500::DirectoryAbstractService::{DirectoryBindArgument, _encode_DirectoryBindArgument};
    use x500::DirectoryIDMProtocols::id_idm_dap;

    #[tokio::test]
    async fn test_bind_to_x500_dsa_via_idm() {
        let mut addrs = "dsa01.root.mkdemo.wildboar.software:4632"
            .to_socket_addrs()
            .unwrap();
        let socket = TcpSocket::new_v4().unwrap();
        let stream = socket.connect(addrs.next().unwrap()).await.unwrap();

        let idm = IdmStream::new(stream);
        let mut rose = RoseStream::new(idm);
        let dba = DirectoryBindArgument::new(None, None, vec![]);
        let encoded_dba = _encode_DirectoryBindArgument(&dba).unwrap();
        let bytes_written = rose
            .write_bind(BindParameters {
                protocol_id: id_idm_dap(),
                timeout: 5,
                parameter: encoded_dba,
                calling_ae_title: None,
                calling_ap_invocation_identifier: None,
                calling_ae_invocation_identifier: None,
                called_ae_title: None,
                called_ap_invocation_identifier: None,
                called_ae_invocation_identifier: None,
                implementation_information: None,
            })
            .await
            .unwrap();
        sleep(Duration::new(5, 0)).await;
        assert!(bytes_written.gt(&0));
        tokio::time::timeout(Duration::from_millis(10000), async {
            while let Some(rose_pdu) = rose.read_pdu().await.unwrap() {
                match &rose_pdu {
                    RosePDU::BindResult(_br) => {
                        println!("Made it, big dawg.");
                        return;
                    }
                    _ => panic!(),
                };
            }
        })
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_bind_to_x500_dsa_via_idms() {
        use tokio_rustls::TlsConnector;
        use tokio_rustls::rustls::{
            ClientConfig,
            RootCertStore,
            ServerName,
            Certificate,
            Error as TLSError,
        };
        use tokio_rustls::rustls::client::{
            ServerCertVerifier,
            ServerCertVerified,
        };
        use std::time::SystemTime;

        struct NoCertVerifier {}

        impl ServerCertVerifier for NoCertVerifier {
            fn verify_server_cert(
                &self,
                _: &Certificate,
                _: &[Certificate],
                _: &ServerName,
                _: &mut dyn Iterator<Item = &[u8]>,
                _: &[u8],
                _: SystemTime,
            ) -> Result<ServerCertVerified, TLSError> {
                Ok(ServerCertVerified::assertion())
            }
        }

        let root_store = RootCertStore::empty();
        let mut config = ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth()
            ;
        config
            .dangerous()
            .set_certificate_verifier(Arc::new(NoCertVerifier {}));

        let tls_connector = TlsConnector::from(Arc::new(config));
        let server_name = ServerName::try_from("dsa01.root.mkdemo.wildboar.software").unwrap();
        let mut addrs = "dsa01.root.mkdemo.wildboar.software:44632"
            .to_socket_addrs()
            .unwrap();
        let socket = TcpSocket::new_v4().unwrap();
        let stream = socket.connect(addrs.next().unwrap()).await.unwrap();
        let tls_stream = tls_connector.connect(server_name, stream).await.unwrap();
        let idm = IdmStream::new(tls_stream);
        let mut rose = RoseStream::new(idm);
        let dba = DirectoryBindArgument::new(None, None, vec![]);
        let encoded_dba = _encode_DirectoryBindArgument(&dba).unwrap();
        let bytes_written = rose
            .write_bind(BindParameters {
                protocol_id: id_idm_dap(),
                timeout: 5,
                parameter: encoded_dba,
                calling_ae_title: None,
                calling_ap_invocation_identifier: None,
                calling_ae_invocation_identifier: None,
                called_ae_title: None,
                called_ap_invocation_identifier: None,
                called_ae_invocation_identifier: None,
                implementation_information: None,
            })
            .await
            .unwrap();
        sleep(Duration::new(5, 0)).await;
        assert!(bytes_written.gt(&0));
        tokio::time::timeout(Duration::from_millis(10000), async {
            while let Some(rose_pdu) = rose.read_pdu().await.unwrap() {
                match &rose_pdu {
                    RosePDU::BindResult(_br) => {
                        println!("Made it, big dawg.");
                        return;
                    }
                    _ => panic!(),
                };
            }
        })
        .await
        .unwrap();
    }
}
