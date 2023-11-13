use crate::{ServiceResult, OSIApplicationServiceObject, session::OSIConnectionOrientedSessionService};

pub trait OSIConnectionOrientedPresentationService <S, A>
    where S: OSIConnectionOrientedSessionService,
    A: OSIApplicationServiceObject {
    fn receive_ssdu(&mut self, s: &mut S, a: &mut A, ssdu: Vec<u8>) -> ServiceResult;
}

pub trait OSIConnectionOrientedPresentationEntity : OSIConnectionOrientedPresentationService {

}
