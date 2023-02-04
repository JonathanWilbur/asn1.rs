use tokio::net::TcpStream;
use idm::IdmStream;
use tokio::sync::Mutex;
use x500_client::RoseStream;
use tokio::sync::mpsc;

// pub struct SessionState {
//     pub client: RoseStream<IdmStream<TcpStream>>,
// }

// pub struct ServerSideState {
//     pub inner: Mutex<mpsc::Sender<Command>>,
//     pub sessions: Vec<RoseStream<IdmStream<TcpStream>>>,
// }
