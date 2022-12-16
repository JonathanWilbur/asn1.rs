#![allow(non_upper_case_globals)]

pub mod idm;

pub struct ROSEClient<TransportType> {
    pub transport: TransportType,
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