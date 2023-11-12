use std::borrow::Cow;
use smallvec::{SmallVec, smallvec};
use crate::transport::pdu::{
    TPDU,
    DT_TPDU,
    CR_TPDU,
    CC_TPDU,
    DR_TPDU,
    DC_TPDU,
    RJ_TPDU,
    AK_TPDU,
    EA_TPDU,
    ER_TPDU,
    ED_TPDU,
    PC_CHECKSUM,
    TPDU_CODE_DR,
    DR_TPDU_PC_ADDITIONAL_INFO,
    TPDU_CODE_DC, PC_SUBSEQUENCE_NUMBER,
    PC_FLOW_CONTROL_CONFIRM,
    PC_SELECTIVE_ACK_PARAMS,
    TPDU_CODE_EA,
    TPDU_CODE_ER,
    PC_INVALID_TPDU,
    TPDU_CODE_ED,
    PC_CALLING_TRANSPORT_SELECTOR,
    PC_CALLED_TRANSPORT_SELECTOR,
    PC_TPDU_VERSION_NUMBER,
    PC_TPDU_SIZE,
};
use std::io::IoSlice;
use crate::transport::checksum::Checksummer;

use super::{PC_TPDU_PREF_MAX_TPDU_SIZE, PC_TPDU_PROTECTION_PARAMETERS, PC_TPDU_ADDITIONAL_OPTION_SELECTION, PC_TPDU_ALT_PROTOCOL_CLASSES, PC_TPDU_ACK_TIME, PC_TPDU_PRIORITY, PC_TPDU_REASSIGNMENT_TIME, PC_INACTIVITY_TIMER, PC_TPDU_TRANSIT_DELAY, PC_TPDU_RESIDUAL_ERROR_RATE, PC_RESPONDING_TRANSPORT_SELECTOR};

fn tpdu_size_to_code (size: usize) -> Option<u8> {
    match size {
        8192 => Some(0b0000_1101),
        4096 => Some(0b0000_1100),
        2048 => Some(0b0000_1011),
        1024 => Some(0b0000_1010),
        512 => Some(0b0000_1001),
        256 => Some(0b0000_1000),
        128 => Some(0b0000_0111),
        _ => None,
    }
}

// 0 = TPDU header, 1 = SPDU header, 2 = PPDU header (APP 0 / 1 tag)
/// This is defined using SmallVec because there will usually be no more than
/// four byte slices to write out to the NSDU. There is usually just a TPDU
/// header, SPDU header, PPDU header, and the presentation data value.
/// Plus, this API can easily change to be larger if this changes.
///
/// TODO: Couldn't there be a Cow that uses SmallVec<u8> instead of Vec<u8>?
/// That would be really handy, since most PDUs are really only going to have
/// a few bytes. The first vec could be a SmallVec, since that will always be
/// allocated.
// pub type NSDUParts<'a> = SmallVec<[Cow<'a, [u8]>; 6]>;

// I have this fear in my gut that this is one of those "preoptimizations" that
// are the root of all evil, but it makes so much sense.
pub struct NSDUParts<'a> {
    // A CR-TPDU with all known fixed-length parameters supplied will have the
    // fixed length of 7 bytes + 74 bytes of params.
    //
    // A DT-TPDU with all known parameters except the checksum and using the
    // most extended format will only be 16 bytes.
    pub tpdu_header: SmallVec<[u8; 16]>,
    pub tpdu_checksum: Option<[u8; 4]>,
    // DT-TPDU will be 0x01 0x00 0x01 0x00, followed by possible three bytes of the enclosure item parameter.
    // Every SPDU except CONNECT and ACCEPT is pretty small.
    pub spdu_header: SmallVec<[u8; 8]>,
    pub ppdu_tag_and_len: SmallVec<[u8; 6]>, // 1 octet for tag, 5 for length encoding.

    /// TODO: Couldn't there be a Cow that uses SmallVec<u8> instead of Vec<u8>?
    /// That would be really handy, since most PDUs are really only going to have
    /// a few bytes. The first vec could be a SmallVec, since that will always be
    /// allocated.
    pub remaining_parts: SmallVec<[Cow<'a, [u8]>; 4]>,
}

impl <'a> NSDUParts<'a> {

    pub fn to_io_slice_vec (&'a self) -> SmallVec<[IoSlice<'a>; 16]> {
        let mut ret: SmallVec<[IoSlice<'a>; 16]> = smallvec![
            IoSlice::new(self.tpdu_header.as_slice())
        ];
        if let Some(cs) = self.tpdu_checksum.as_ref() {
            ret.push(IoSlice::new(cs.as_slice()));
        }
        if self.spdu_header.len() > 0 {
            ret.push(IoSlice::new(self.spdu_header.as_slice()));
        }
        if self.ppdu_tag_and_len.len() > 0 {
            ret.push(IoSlice::new(self.ppdu_tag_and_len.as_slice()));
        }
        for rp in self.remaining_parts.iter() {
            ret.push(IoSlice::new(rp.as_ref()));
        }
        ret
    }

    pub fn to_vec (&self) -> Vec<u8> {
        [
            self.tpdu_header.as_slice(),
            if let Some(cs) = self.tpdu_checksum.as_ref() {
                cs.as_slice()
            } else {
                &[]
            },
            self.spdu_header.as_slice(),
            self.ppdu_tag_and_len.as_slice(),
            // TODO: Can you avoid this sub-concatenation?
            self.remaining_parts.iter().map(|p| p.as_ref()).collect::<Vec<&[u8]>>().concat().as_slice(),
        ].concat()
    }

    pub fn set_checksum(&mut self) {
        let mut cs = Checksummer::new();
        cs.update(self.tpdu_header.as_slice());
        cs.update(&[ PC_CHECKSUM, 2, 0, 0 ]); // This starts off with zeros and gets filled in.
        cs.update(self.spdu_header.as_slice());
        cs.update(self.ppdu_tag_and_len.as_slice());
        for rp in self.remaining_parts.iter() {
            cs.update(rp.as_ref());
        }
        let sum = cs.digest(self.tpdu_header.len());
        self.tpdu_checksum = Some([ PC_CHECKSUM, 2, (sum & 0xFF00 >> 8) as u8, (sum & 0xFF) as u8 ]);
    }

}

// TODO: Should these functions handle splitting?
// My thinking is no, since really only the DT-TPDU can be split anyway.
pub trait IntoNSDU<'a> {
    /// NOTE: This MUST generate the checksum. The checksum will be added
    fn to_nsdu_parts (&'a self, class: u8, ext: bool, add_checksum: bool) -> NSDUParts<'a>;
}

impl<'a> IntoNSDU<'a> for TPDU<'a> {

    fn to_nsdu_parts (&'a self, class: u8, ext: bool, add_checksum: bool) -> NSDUParts<'a> {
        match self {
            TPDU::DT(tpdu) => tpdu.to_nsdu_parts(class, ext, add_checksum),
            TPDU::CR(tpdu) => tpdu.to_nsdu_parts(class, ext, add_checksum),
            TPDU::CC(tpdu) => tpdu.to_nsdu_parts(class, ext, add_checksum),
            TPDU::DR(tpdu) => tpdu.to_nsdu_parts(class, ext, add_checksum),
            TPDU::DC(tpdu) => tpdu.to_nsdu_parts(class, ext, add_checksum),
            TPDU::RJ(tpdu) => tpdu.to_nsdu_parts(class, ext, add_checksum),
            TPDU::AK(tpdu) => tpdu.to_nsdu_parts(class, ext, add_checksum),
            TPDU::EA(tpdu) => tpdu.to_nsdu_parts(class, ext, add_checksum),
            TPDU::ER(tpdu) => tpdu.to_nsdu_parts(class, ext, add_checksum),
            TPDU::ED(tpdu) => tpdu.to_nsdu_parts(class, ext, add_checksum),
        }
    }

}

impl<'a> IntoNSDU<'a>  for DT_TPDU<'a> {

    fn to_nsdu_parts (&'a self, class: u8, ext: bool, add_checksum: bool) -> NSDUParts<'a> {
        let mut tpdu_header: SmallVec<[u8; 16]> = SmallVec::new();
        let ed_tpdu_nr_len: u8 = if self.ed_tpdu_nr.is_some() {
            if ext { 6 } else { 4 }
        } else {
            0
        };
        if [ 2, 3, 4 ].contains(&class) {
            debug_assert!(self.dst_ref.is_some());
            // Checksum is only generated in class 4.
            let checksum_len: u8 = if add_checksum { 4 } else { 0 };
            let fixed_len: u8 = if ext { 7 } else { 4 };
            let li = fixed_len + ed_tpdu_nr_len + checksum_len;
            let dst_ref_bytes = self.dst_ref.unwrap_or(0).to_be_bytes();
            let nr_bytes = self.nr.to_be_bytes();
            tpdu_header.push(li);
            tpdu_header.push(if self.roa { 0b1111_0001 } else { 0b1111_0000 });
            tpdu_header.push(dst_ref_bytes[0]);
            tpdu_header.push(dst_ref_bytes[1]);
            let eot_mask: u8 = if self.eot { 0b1000_0000 } else { 0 };
            if ext {
                tpdu_header.push(nr_bytes[0] | eot_mask);
                tpdu_header.push(nr_bytes[1]);
                tpdu_header.push(nr_bytes[2]);
                tpdu_header.push(nr_bytes[3]);
            } else {
                debug_assert!(self.nr <= u8::MAX.into());
                debug_assert!((self.nr & 0b1000_0000) == 0);
                tpdu_header.push(nr_bytes[3] | eot_mask);
            }
        }
        else {
            debug_assert!(self.nr <= u8::MAX.into());
            debug_assert!((self.nr & 0b1000_0000) == 0);
            let li = 2;
            tpdu_header.push(li);
            tpdu_header.push(if self.roa { 0b1111_0001 } else { 0b1111_0000 });
            let nr_bytes = self.nr.to_be_bytes();
            let eot_mask: u8 = if self.eot { 0b1000_0000 } else { 0 };
            tpdu_header.push(nr_bytes[3] | eot_mask);
        }
        let mut ret = NSDUParts{
            tpdu_header,
            tpdu_checksum: None,
            spdu_header: smallvec![],
            ppdu_tag_and_len: smallvec![],
            remaining_parts: smallvec![
                Cow::Borrowed(self.user_data.as_ref()),
            ],
        };
        if add_checksum {
            ret.set_checksum();
        }
        ret
    }

}

impl<'a> IntoNSDU<'a> for CR_TPDU {

    fn to_nsdu_parts (&'a self, class: u8, _ext: bool, add_checksum: bool) -> NSDUParts<'a> {
        let mut tpdu_header: SmallVec<[u8; 16]> = SmallVec::with_capacity(250); // 254 = Max minus 4 for checksum.
        let checksum_len: u8 = if add_checksum { 4 } else { 0 };
        let li = 0; // This will be changed later.
        tpdu_header.push(li);
        tpdu_header.push(0b1110_0000 | (self.cdt & 0x0F));
        tpdu_header.push(0);
        tpdu_header.push(0);
        tpdu_header.extend_from_slice(self.src_ref.to_be_bytes().as_slice());
        tpdu_header.push(self.class_option);
        if let Some(calling_transport_selector) = self.calling_transport_selector.as_ref() {
            tpdu_header.push(PC_CALLING_TRANSPORT_SELECTOR);
            tpdu_header.push(calling_transport_selector.len() as u8);
            tpdu_header.extend_from_slice(calling_transport_selector.as_ref());
        }
        if let Some(called_or_responding_transport_selector) = self.called_or_responding_transport_selector.as_ref() {
            tpdu_header.push(PC_CALLED_TRANSPORT_SELECTOR);
            tpdu_header.push(called_or_responding_transport_selector.len() as u8);
            tpdu_header.extend_from_slice(called_or_responding_transport_selector.as_ref());
        }
        if let Some(tpdu_size) = self.tpdu_size {
            tpdu_header.push(PC_TPDU_SIZE);
            tpdu_header.push(1);
            tpdu_header.push(tpdu_size_to_code(tpdu_size).unwrap_or(0b0000_01111));
        }
        if let Some(preferred_max_tpdu_size) = self.preferred_max_tpdu_size {
            let max = preferred_max_tpdu_size.min(u32::MAX as usize);
            tpdu_header.push(PC_TPDU_PREF_MAX_TPDU_SIZE);
            tpdu_header.push(4);
            // The spec does not say that this has to be encoded on the fewest octets.
            tpdu_header.extend_from_slice(max.to_be_bytes().as_slice());
        }
        if self.version_number != 1 && class > 0 {
            tpdu_header.push(PC_TPDU_VERSION_NUMBER);
            tpdu_header.push(1);
            tpdu_header.push(self.version_number);
        }
        if let Some(protection_parameters) = self.protection_parameters.as_ref() {
            tpdu_header.push(PC_TPDU_PROTECTION_PARAMETERS);
            tpdu_header.push(protection_parameters.len() as u8);
            tpdu_header.extend_from_slice(protection_parameters.as_ref());
        }
        if self.additional_option_selection != 0b0000_0001 && class > 0 {
            tpdu_header.push(PC_TPDU_ADDITIONAL_OPTION_SELECTION);
            tpdu_header.push(1);
            tpdu_header.push(self.additional_option_selection);
        }
        if self.alternative_protocol_classes > 0 {
            let p0: bool = (self.alternative_protocol_classes & 0b0000_0001) > 0;
            let p1: bool = (self.alternative_protocol_classes & 0b0000_0010) > 0;
            let p2: bool = (self.alternative_protocol_classes & 0b0000_0100) > 0;
            let p3: bool = (self.alternative_protocol_classes & 0b0000_1000) > 0;
            let p4: bool = (self.alternative_protocol_classes & 0b0001_0000) > 0;
            let mut len: u8 = 0;
            [ p0, p1, p2, p3, p4 ].iter().for_each(|p| if *p { len += 1 });
            tpdu_header.push(PC_TPDU_ALT_PROTOCOL_CLASSES);
            tpdu_header.push(len);
            if p0 {
                tpdu_header.push(0b0000_0000);
            }
            if p1 {
                tpdu_header.push(0b0001_0000);
            }
            if p2 {
                tpdu_header.push(0b0010_0000);
            }
            if p3 {
                tpdu_header.push(0b0011_0000);
            }
            if p4 {
                tpdu_header.push(0b0100_0000);
            }
        }
        if let Some(acknowledgement_time) = self.acknowledgement_time {
            tpdu_header.push(PC_TPDU_ACK_TIME);
            tpdu_header.push(2);
            tpdu_header.extend_from_slice(acknowledgement_time.to_be_bytes().as_slice());
        }
        if let Some(throughput) = self.throughput {
            tpdu_header.push(PC_TPDU_RESIDUAL_ERROR_RATE);
            tpdu_header.push(if throughput.average.is_some() { 24 } else { 12 });
            tpdu_header.extend_from_slice(&throughput.max.calling_to_called.target.to_be_bytes()[1..]);
            tpdu_header.extend_from_slice(&throughput.max.calling_to_called.min_acceptable.to_be_bytes()[1..]);
            tpdu_header.extend_from_slice(&throughput.max.called_to_calling.target.to_be_bytes()[1..]);
            tpdu_header.extend_from_slice(&throughput.max.called_to_calling.min_acceptable.to_be_bytes()[1..]);
            if let Some(avg) = throughput.average {
                tpdu_header.extend_from_slice(&avg.calling_to_called.target.to_be_bytes()[1..]);
                tpdu_header.extend_from_slice(&avg.calling_to_called.min_acceptable.to_be_bytes()[1..]);
                tpdu_header.extend_from_slice(&avg.called_to_calling.target.to_be_bytes()[1..]);
                tpdu_header.extend_from_slice(&avg.called_to_calling.min_acceptable.to_be_bytes()[1..]);
            }
        }
        if let Some(residual_error_rate) = self.residual_error_rate {
            tpdu_header.push(PC_TPDU_RESIDUAL_ERROR_RATE);
            tpdu_header.push(3);
            let target: u8 = residual_error_rate.target.ilog10() as u8;
            let min_acceptable: u8 = residual_error_rate.minimum_acceptable.ilog10() as u8;
            let tsdu_size: u8 = residual_error_rate.tsdu_size_of_interest.ilog2() as u8;
            tpdu_header.push(target);
            tpdu_header.push(min_acceptable);
            tpdu_header.push(tsdu_size);
        }
        if let Some(priority) = self.priority {
            tpdu_header.push(PC_TPDU_PRIORITY);
            tpdu_header.push(2);
            tpdu_header.extend_from_slice(priority.to_be_bytes().as_slice());
        }
        if let Some(transit_delay) = self.transit_delay {
            tpdu_header.push(PC_TPDU_TRANSIT_DELAY);
            tpdu_header.push(8);
            tpdu_header.extend_from_slice(transit_delay.calling_to_called.target.to_be_bytes().as_slice());
            tpdu_header.extend_from_slice(transit_delay.calling_to_called.max_acceptable.to_be_bytes().as_slice());
            tpdu_header.extend_from_slice(transit_delay.called_to_calling.target.to_be_bytes().as_slice());
            tpdu_header.extend_from_slice(transit_delay.called_to_calling.max_acceptable.to_be_bytes().as_slice());
        }
        if let Some(reassignment_time) = self.reassignment_time {
            tpdu_header.push(PC_TPDU_REASSIGNMENT_TIME);
            tpdu_header.push(2);
            tpdu_header.extend_from_slice(reassignment_time.to_be_bytes().as_slice());
        }
        if let Some(inactivity_timer) = self.inactivity_timer {
            tpdu_header.push(PC_INACTIVITY_TIMER);
            tpdu_header.push(4);
            tpdu_header.extend_from_slice(inactivity_timer.to_be_bytes().as_slice());
        }

        tpdu_header[0] = (tpdu_header.len() - 1) as u8 + checksum_len;
        let mut ret = NSDUParts{
            tpdu_header,
            tpdu_checksum: None,
            spdu_header: smallvec![],
            ppdu_tag_and_len: smallvec![],
            remaining_parts: smallvec![
                Cow::Borrowed(self.user_data.as_ref().as_ref()),
            ],
        };
        if add_checksum  { // TODO: Or if class 4 is the preferred class.
            ret.set_checksum();
        }
        ret
    }

}

impl<'a> IntoNSDU<'a>  for CC_TPDU<'a> {

    fn to_nsdu_parts (&self, class: u8, _ext: bool, add_checksum: bool) -> NSDUParts<'a> {
        let mut tpdu_header: SmallVec<[u8; 16]> = SmallVec::new();
        let checksum_len: u8 = if add_checksum { 4 } else { 0 };
        let li = 0; // This will be changed later.
        tpdu_header.push(li);
        tpdu_header.push(0b1101_0000 | (self.cdt & 0x0F));
        tpdu_header.extend_from_slice(self.dst_ref.to_be_bytes().as_slice());
        tpdu_header.extend_from_slice(self.src_ref.to_be_bytes().as_slice());
        tpdu_header.push(self.class_option);
        if let Some(calling_transport_selector) = self.calling_transport_selector {
            tpdu_header.push(PC_CALLING_TRANSPORT_SELECTOR);
            tpdu_header.push(calling_transport_selector.len() as u8);
            tpdu_header.extend_from_slice(calling_transport_selector);
        }
        if let Some(called_or_responding_transport_selector) = self.called_or_responding_transport_selector {
            tpdu_header.push(PC_RESPONDING_TRANSPORT_SELECTOR);
            tpdu_header.push(called_or_responding_transport_selector.len() as u8);
            tpdu_header.extend_from_slice(called_or_responding_transport_selector);
        }
        if let Some(tpdu_size) = self.tpdu_size {
            tpdu_header.push(PC_TPDU_SIZE);
            tpdu_header.push(1);
            tpdu_header.push(tpdu_size_to_code(tpdu_size).unwrap_or(0b0000_01111));
        }
        if let Some(preferred_max_tpdu_size) = self.preferred_max_tpdu_size {
            let max = preferred_max_tpdu_size.min(u32::MAX as usize);
            tpdu_header.push(PC_TPDU_PREF_MAX_TPDU_SIZE);
            tpdu_header.push(4);
            // The spec does not say that this has to be encoded on the fewest octets.
            tpdu_header.extend_from_slice(max.to_be_bytes().as_slice());
        }
        if let Some(protection_parameters) = self.protection_parameters {
            tpdu_header.push(PC_TPDU_PROTECTION_PARAMETERS);
            tpdu_header.push(protection_parameters.len() as u8);
            tpdu_header.extend_from_slice(protection_parameters);
        }
        if self.additional_option_selection != 0b0000_0001 && class > 0 {
            tpdu_header.push(PC_TPDU_ADDITIONAL_OPTION_SELECTION);
            tpdu_header.push(1);
            tpdu_header.push(self.additional_option_selection);
        }
        if self.alternative_protocol_classes > 0 {
            let p0: bool = (self.alternative_protocol_classes & 0b0000_0001) > 0;
            let p1: bool = (self.alternative_protocol_classes & 0b0000_0010) > 0;
            let p2: bool = (self.alternative_protocol_classes & 0b0000_0100) > 0;
            let p3: bool = (self.alternative_protocol_classes & 0b0000_1000) > 0;
            let p4: bool = (self.alternative_protocol_classes & 0b0001_0000) > 0;
            let mut len: u8 = 0;
            [ p0, p1, p2, p3, p4 ].iter().for_each(|p| if *p { len += 1 });
            tpdu_header.push(PC_TPDU_ALT_PROTOCOL_CLASSES);
            tpdu_header.push(len);
            if p0 {
                tpdu_header.push(0b0000_0000);
            }
            if p1 {
                tpdu_header.push(0b0001_0000);
            }
            if p2 {
                tpdu_header.push(0b0010_0000);
            }
            if p3 {
                tpdu_header.push(0b0011_0000);
            }
            if p4 {
                tpdu_header.push(0b0100_0000);
            }
        }
        if let Some(acknowledgement_time) = self.acknowledgement_time {
            tpdu_header.push(PC_TPDU_ACK_TIME);
            tpdu_header.push(2);
            tpdu_header.extend_from_slice(acknowledgement_time.to_be_bytes().as_slice());
        }
        if let Some(throughput) = self.throughput {
            tpdu_header.push(PC_TPDU_RESIDUAL_ERROR_RATE);
            tpdu_header.push(if throughput.average.is_some() { 24 } else { 12 });
            tpdu_header.extend_from_slice(&throughput.max.calling_to_called.target.to_be_bytes()[1..]);
            tpdu_header.extend_from_slice(&throughput.max.calling_to_called.min_acceptable.to_be_bytes()[1..]);
            tpdu_header.extend_from_slice(&throughput.max.called_to_calling.target.to_be_bytes()[1..]);
            tpdu_header.extend_from_slice(&throughput.max.called_to_calling.min_acceptable.to_be_bytes()[1..]);
            if let Some(avg) = throughput.average {
                tpdu_header.extend_from_slice(&avg.calling_to_called.target.to_be_bytes()[1..]);
                tpdu_header.extend_from_slice(&avg.calling_to_called.min_acceptable.to_be_bytes()[1..]);
                tpdu_header.extend_from_slice(&avg.called_to_calling.target.to_be_bytes()[1..]);
                tpdu_header.extend_from_slice(&avg.called_to_calling.min_acceptable.to_be_bytes()[1..]);
            }
        }
        if let Some(residual_error_rate) = self.residual_error_rate {
            tpdu_header.push(PC_TPDU_RESIDUAL_ERROR_RATE);
            tpdu_header.push(3);
            let target: u8 = residual_error_rate.target.ilog10() as u8;
            let min_acceptable: u8 = residual_error_rate.minimum_acceptable.ilog10() as u8;
            let tsdu_size: u8 = residual_error_rate.tsdu_size_of_interest.ilog2() as u8;
            tpdu_header.push(target);
            tpdu_header.push(min_acceptable);
            tpdu_header.push(tsdu_size);
        }
        if let Some(priority) = self.priority {
            tpdu_header.push(PC_TPDU_PRIORITY);
            tpdu_header.push(2);
            tpdu_header.extend_from_slice(priority.to_be_bytes().as_slice());
        }
        if let Some(transit_delay) = self.transit_delay {
            tpdu_header.push(PC_TPDU_TRANSIT_DELAY);
            tpdu_header.push(8);
            tpdu_header.extend_from_slice(transit_delay.calling_to_called.target.to_be_bytes().as_slice());
            tpdu_header.extend_from_slice(transit_delay.calling_to_called.max_acceptable.to_be_bytes().as_slice());
            tpdu_header.extend_from_slice(transit_delay.called_to_calling.target.to_be_bytes().as_slice());
            tpdu_header.extend_from_slice(transit_delay.called_to_calling.max_acceptable.to_be_bytes().as_slice());
        }
        if let Some(inactivity_timer) = self.inactivity_timer {
            tpdu_header.push(PC_INACTIVITY_TIMER);
            tpdu_header.push(4);
            tpdu_header.extend_from_slice(inactivity_timer.to_be_bytes().as_slice());
        }

        tpdu_header[0] = (tpdu_header.len() - 1) as u8 + checksum_len;
        let mut ret = NSDUParts{
            tpdu_header,
            tpdu_checksum: None,
            spdu_header: smallvec![],
            ppdu_tag_and_len: smallvec![],
            remaining_parts: smallvec![
                Cow::Borrowed(self.user_data.as_ref()),
            ],
        };
        if add_checksum {
            ret.set_checksum();
        }
        ret
    }

}

impl<'a> IntoNSDU<'a>  for DR_TPDU<'a> {

    fn to_nsdu_parts (&'a self, _class: u8, _ext: bool, add_checksum: bool) -> NSDUParts<'a> {
        // TODO: Should the length just be added to LI when the checksum is set?
        let checksum_len: u8 = if add_checksum { 4 } else { 0 };
        let fixed_len: u8 = 7;
        let addl_info_len: u8 = self.additional_info.map(|a| a.len() as u8 + 2).unwrap_or(0);
        let li = fixed_len + checksum_len + addl_info_len;
        let mut tpdu_header: SmallVec<[u8; 16]> = SmallVec::new();
        let dst_ref_bytes = self.dst_ref.to_be_bytes();
        let src_ref_bytes = self.src_ref.to_be_bytes();
        tpdu_header.push(li);
        tpdu_header.push(TPDU_CODE_DR);
        tpdu_header.push(dst_ref_bytes[0]);
        tpdu_header.push(dst_ref_bytes[1]);
        tpdu_header.push(src_ref_bytes[0]);
        tpdu_header.push(src_ref_bytes[1]);
        tpdu_header.push(self.reason);
        if let Some(add_info) = self.additional_info {
            tpdu_header.push(DR_TPDU_PC_ADDITIONAL_INFO);
            tpdu_header.push(add_info.len() as u8);
            // NOTE: Unless this is short, this is likely to cause heap allocation!
            tpdu_header.extend_from_slice(add_info);
        }
        let mut ret = NSDUParts{
            tpdu_header,
            tpdu_checksum: None,
            spdu_header: smallvec![],
            ppdu_tag_and_len: smallvec![],
            remaining_parts: smallvec![
                Cow::Borrowed(self.user_data.as_ref()),
            ],
        };
        if add_checksum {
            ret.set_checksum();
        }
        ret
    }

}

impl<'a> IntoNSDU<'a>  for DC_TPDU {

    fn to_nsdu_parts (&self, _class: u8, _ext: bool, add_checksum: bool) -> NSDUParts<'a> {
        let checksum_len: u8 = if add_checksum { 4 } else { 0 };
        let fixed_len: u8 = 6;
        let li = fixed_len + checksum_len;
        let mut tpdu_header: SmallVec<[u8; 16]> = SmallVec::new();
        let dst_ref_bytes = self.dst_ref.to_be_bytes();
        let src_ref_bytes = self.src_ref.to_be_bytes();
        tpdu_header.push(li);
        tpdu_header.push(TPDU_CODE_DC);
        tpdu_header.push(dst_ref_bytes[0]);
        tpdu_header.push(dst_ref_bytes[1]);
        tpdu_header.push(src_ref_bytes[0]);
        tpdu_header.push(src_ref_bytes[1]);
        let mut ret = NSDUParts{
            tpdu_header,
            tpdu_checksum: None,
            spdu_header: smallvec![],
            ppdu_tag_and_len: smallvec![],
            remaining_parts: smallvec![],
        };
        if add_checksum {
            ret.set_checksum();
        }
        ret
    }

}

impl<'a> IntoNSDU<'a>  for RJ_TPDU {

    fn to_nsdu_parts (&self, _class: u8, ext: bool, _add_checksum: bool) -> NSDUParts<'a> {
        let fixed_len: u8 = if ext { 9 } else { 4 };
        let li = fixed_len;
        let mut tpdu_header: SmallVec<[u8; 16]> = SmallVec::new();
        let dst_ref_bytes = self.dst_ref.to_be_bytes();
        tpdu_header.push(li);
        tpdu_header.push(if ext { 0b0101_0000 } else { 0b0101_0000 | (self.cdt & 0x000F) as u8 });
        tpdu_header.push(dst_ref_bytes[0]);
        tpdu_header.push(dst_ref_bytes[1]);
        let nr_bytes = self.yr_tu_nr.to_be_bytes();
        if ext {
            tpdu_header.push(nr_bytes[0] & 0b0111_1111);
            tpdu_header.push(nr_bytes[1]);
            tpdu_header.push(nr_bytes[2]);
            tpdu_header.push(nr_bytes[3]);
            let cdt_bytes = self.cdt.to_be_bytes();
            tpdu_header.push(cdt_bytes[0]);
            tpdu_header.push(cdt_bytes[1]);
        } else {
            debug_assert!(self.yr_tu_nr <= u8::MAX.into());
            debug_assert!((self.yr_tu_nr & 0b1000_0000) == 0);
            tpdu_header.push(nr_bytes[3] & 0b0111_1111);
        }
        NSDUParts{
            tpdu_header,
            tpdu_checksum: None,
            spdu_header: smallvec![],
            ppdu_tag_and_len: smallvec![],
            remaining_parts: smallvec![],
        }
    }

}

impl<'a> IntoNSDU<'a>  for AK_TPDU {

    fn to_nsdu_parts (&self, _class: u8, ext: bool, add_checksum: bool) -> NSDUParts<'a> {
        let checksum_len: u8 = if add_checksum { 4 } else { 0 };
        let fixed_len: u8 = if ext { 9 } else { 4 };
        let li = fixed_len + checksum_len;
        let mut tpdu_header: SmallVec<[u8; 16]> = SmallVec::new();
        let dst_ref_bytes = self.dst_ref.to_be_bytes();
        tpdu_header.push(li);
        tpdu_header.push(if ext { 0b0110_0000 } else { 0b0110_0000 | (self.cdt & 0x000F) as u8 });
        tpdu_header.push(dst_ref_bytes[0]);
        tpdu_header.push(dst_ref_bytes[1]);
        let nr_bytes = self.nr.to_be_bytes();
        if ext {
            tpdu_header.push(nr_bytes[0] & 0b0111_1111);
            tpdu_header.push(nr_bytes[1]);
            tpdu_header.push(nr_bytes[2]);
            tpdu_header.push(nr_bytes[3]);
            let cdt_bytes = self.cdt.to_be_bytes();
            tpdu_header.push(cdt_bytes[0]);
            tpdu_header.push(cdt_bytes[1]);
        } else {
            debug_assert!(self.nr <= u8::MAX.into());
            debug_assert!((self.nr & 0b1000_0000) == 0);
            tpdu_header.push(nr_bytes[3] & 0b0111_1111);
        }
        if let Some(sub) = self.subsequence_number {
            tpdu_header.push(PC_SUBSEQUENCE_NUMBER);
            tpdu_header.push(2);
            tpdu_header.extend_from_slice(sub.to_be_bytes().as_slice());
        }
        if let Some(fcc) = self.flow_control_confirmation.as_ref() {
            tpdu_header.push(PC_FLOW_CONTROL_CONFIRM);
            tpdu_header.push(8);
            tpdu_header.extend_from_slice(fcc.lower_window_edge.to_be_bytes().as_slice());
            tpdu_header.extend_from_slice(fcc.your_subsequence.to_be_bytes().as_slice());
            tpdu_header.extend_from_slice(fcc.your_credit.to_be_bytes().as_slice());
        }
        if let Some(saps) = self.selective_acknowledgement_parameters.as_ref() {
            tpdu_header.push(PC_SELECTIVE_ACK_PARAMS);
            // TODO: Make sure we don't overflow here.
            if ext {
                tpdu_header.push((saps.len() << 3) as u8);
                for sap in saps {
                    tpdu_header.extend_from_slice(sap.lower_edge.to_be_bytes().as_slice());
                    tpdu_header.extend_from_slice(sap.upper_edge.to_be_bytes().as_slice());
                }
            } else {
                tpdu_header.push((saps.len() << 1) as u8);
                for sap in saps {
                    debug_assert!(sap.lower_edge < u8::MAX as u32);
                    debug_assert!(sap.upper_edge < u8::MAX as u32);
                    debug_assert!((sap.lower_edge & 0b1000_0000) == 0);
                    debug_assert!((sap.upper_edge & 0b1000_0000) == 0);
                    tpdu_header.push(sap.lower_edge as u8);
                    tpdu_header.push(sap.upper_edge as u8);
                }
            }
        }
        tpdu_header[0] = (tpdu_header.len() - 1) as u8 + checksum_len;
        let mut ret = NSDUParts{
            tpdu_header,
            tpdu_checksum: None,
            spdu_header: smallvec![],
            ppdu_tag_and_len: smallvec![],
            remaining_parts: smallvec![],
        };
        if add_checksum {
            ret.set_checksum();
        }
        ret
    }

}

impl<'a> IntoNSDU<'a>  for EA_TPDU {

    fn to_nsdu_parts (&self, _class: u8, ext: bool, add_checksum: bool) -> NSDUParts<'a> {
        let checksum_len: u8 = if add_checksum { 4 } else { 0 };
        let fixed_len: u8 = if ext { 7 } else { 4 };
        let li = fixed_len + checksum_len;
        let mut tpdu_header: SmallVec<[u8; 16]> = SmallVec::new();
        let dst_ref_bytes = self.dst_ref.to_be_bytes();
        tpdu_header.push(li);
        tpdu_header.push(TPDU_CODE_EA);
        tpdu_header.push(dst_ref_bytes[0]);
        tpdu_header.push(dst_ref_bytes[1]);
        let nr_bytes = self.nr.to_be_bytes();
        if ext {
            tpdu_header.push(nr_bytes[0] & 0b0111_1111);
            tpdu_header.push(nr_bytes[1]);
            tpdu_header.push(nr_bytes[2]);
            tpdu_header.push(nr_bytes[3]);
        } else {
            debug_assert!(self.nr <= u8::MAX.into());
            debug_assert!((self.nr & 0b1000_0000) == 0);
            tpdu_header.push(nr_bytes[3] & 0b0111_1111);
        }
        let mut ret = NSDUParts{
            tpdu_header,
            tpdu_checksum: None,
            spdu_header: smallvec![],
            ppdu_tag_and_len: smallvec![],
            remaining_parts: smallvec![],
        };
        if add_checksum {
            ret.set_checksum();
        }
        ret
    }

}

impl<'a> IntoNSDU<'a>  for ER_TPDU<'a> {

    fn to_nsdu_parts (&self, _class: u8, _ext: bool, add_checksum: bool) -> NSDUParts<'a> {
        let checksum_len: u8 = if add_checksum { 4 } else { 0 };
        let invalid_pdu_len: u8 = self.invalid_tpdu.as_ref().map(|p| p.len() as u8 + 2).unwrap_or(0);
        let fixed_len: u8 = 4;
        let li = fixed_len + checksum_len + invalid_pdu_len;
        let mut tpdu_header: SmallVec<[u8; 16]> = SmallVec::new();
        let dst_ref_bytes = self.dst_ref.to_be_bytes();
        tpdu_header.push(li);
        tpdu_header.push(TPDU_CODE_ER);
        tpdu_header.push(dst_ref_bytes[0]);
        tpdu_header.push(dst_ref_bytes[1]);
        tpdu_header.push(self.reject_cause);
        if let Some(invalid_tpdu) = self.invalid_tpdu.as_ref() {
            tpdu_header.push(PC_INVALID_TPDU);
            tpdu_header.push(invalid_pdu_len);
            tpdu_header.extend_from_slice(invalid_tpdu);
        }
        let mut ret = NSDUParts{
            tpdu_header,
            tpdu_checksum: None,
            spdu_header: smallvec![],
            ppdu_tag_and_len: smallvec![],
            remaining_parts: smallvec![],
        };
        if add_checksum {
            ret.set_checksum();
        }
        ret
    }

}

impl<'a> IntoNSDU<'a>  for ED_TPDU<'a> {

    fn to_nsdu_parts (&self, _class: u8, ext: bool, add_checksum: bool) -> NSDUParts<'a> {
        let checksum_len: u8 = if add_checksum { 4 } else { 0 };
        let fixed_len: u8 = if ext { 7 } else { 4 };
        let li = fixed_len + checksum_len;
        let mut tpdu_header: SmallVec<[u8; 16]> = SmallVec::new();
        let dst_ref_bytes = self.dst_ref.to_be_bytes();
        tpdu_header.push(li);
        tpdu_header.push(TPDU_CODE_ED);
        tpdu_header.push(dst_ref_bytes[0]);
        tpdu_header.push(dst_ref_bytes[1]);
        let nr_bytes = self.nr.to_be_bytes();
        let eot_mask: u8 = if self.eot { 0b1000_0000 } else { 0 };
        if ext {
            tpdu_header.push(nr_bytes[0] | eot_mask);
            tpdu_header.push(nr_bytes[1]);
            tpdu_header.push(nr_bytes[2]);
            tpdu_header.push(nr_bytes[3]);
        } else {
            debug_assert!(self.nr <= u8::MAX.into());
            debug_assert!((self.nr & 0b1000_0000) == 0);
            tpdu_header.push(nr_bytes[3] | eot_mask);
        }
        let mut ret = NSDUParts{
            tpdu_header,
            tpdu_checksum: None,
            spdu_header: smallvec![],
            ppdu_tag_and_len: smallvec![],
            remaining_parts: smallvec![],
        };
        if add_checksum {
            ret.set_checksum();
        }
        ret
    }

}

// TODO: Testing
