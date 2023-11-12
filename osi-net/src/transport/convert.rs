use crate::transport::CR_TPDU;
use crate::transport::T_CONNECT_Request_Parameters;

impl <'a> From<CR_TPDU> for T_CONNECT_Request_Parameters {

    fn from(value: CR_TPDU) -> Self {
        T_CONNECT_Request_Parameters {
            expedited_data: false, // FIXME:
            qos: None, // FIXME:
            user_data: value.user_data.as_ref().to_owned(),
        }
    }

}
