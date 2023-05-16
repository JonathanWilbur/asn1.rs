use rose_stream::RoseClient;
use tokio::net::TcpStream;
use idm::IdmStream;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use crate::Command;

pub struct SessionState {
    pub client: RoseClient,
}

pub struct ServerSideState {
    pub inner: Mutex<mpsc::Sender<Command>>,
    pub sessions: Vec<RoseClient>,
}
