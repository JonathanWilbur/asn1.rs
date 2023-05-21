use asn1::{BIT_STRING, read_i64};
use dirstudio_api_types::{
    BindResult,
    DirectoryClientError,
    TlsProtectionLevel,
    CredentialType,
    PwdResponse,
    PwdResponseWarning,
    Base64String,
    ApiGeneralName,
    IntoApiGeneralName,
};
use crate::state::{SessionId, ServerSideState, SessionState};
use rose::BindOutcome;
use ::idm::IdmStream;
use rose::BindParameters;
use std::net::ToSocketAddrs;
use std::time::Duration;
use tokio::net::TcpSocket;
use x500::DirectoryAbstractService::{
    DirectoryBindArgument,
    DirectoryBindResult,
    Credentials,
    PwdResponseValue_warning,
};
use x500::DirectoryIDMProtocols::id_idm_dap;
use rose_stream::RoseClient;
use x500_client::DAPClient;
use std::sync::Arc;
use tauri::async_runtime::Mutex;

async fn bind_idm (state: &ServerSideState, address: &str) -> Result<BindResult, DirectoryClientError> {
    let mut addrs = address.to_socket_addrs().map_err(|_| DirectoryClientError::MalformedUrl)?;
    let socket = TcpSocket::new_v4().unwrap();
    let stream = socket.connect(addrs.next().unwrap()).await.unwrap();
    let (rose, _) = RoseClient::from_idm(
        IdmStream::new(stream),
        Duration::from_millis(5000),
        5,
    );
    let mut dap = DAPClient::new(rose);
    // TODO: Use credentials
    let dba = DirectoryBindArgument::new(None, Some(BIT_STRING::from_bin("11")), vec![]);
    let bind_outcome = dap.bind(BindParameters {
            protocol_id: id_idm_dap(),
            timeout: 5,
            parameter: dba,
            calling_ae_title: None,
            calling_ap_invocation_identifier: None,
            calling_ae_invocation_identifier: None,
            called_ae_title: None,
            called_ap_invocation_identifier: None,
            called_ae_invocation_identifier: None,
            implementation_information: Some(String::from("Wildboar Directory Studio")),
        })
        .await
        .map_err(|_| DirectoryClientError::ApplicationAssociationRejected)?; // TODO: Return a proper bind error.
    match bind_outcome {
        BindOutcome::Result(r) => {
            let versions = r.parameter.versions.unwrap_or(DirectoryBindResult::_default_value_for_versions());
            let v1: bool = versions.bytes.len() > 0 && ((versions.bytes[0] & 0b1000_0000) > 0);
            let v2: bool = versions.bytes.len() > 0 && ((versions.bytes[0] & 0b0100_0000) > 0);
            if !v1 && !v2 {
                // TODO: Define a different error type. Theoretically, this should not happen.
                return Err(DirectoryClientError::ApplicationAssociationRejected);
            }
            let dsa_credential_type = match &r.parameter.credentials {
                None => CredentialType::Anonymous,
                Some(creds) => {
                    match &creds {
                        Credentials::simple(c) => {
                            if c.password.is_some() { // TODO: Verify this somehow.
                                CredentialType::SimpleWithPassword
                            } else {
                                CredentialType::SimpleNoPassword
                            }
                        },
                        Credentials::strong(_) => CredentialType::Strong,
                        Credentials::sasl(c) => CredentialType::Sasl(c.mechanism.to_string()),
                        Credentials::spkm(_) => CredentialType::Spkm,
                        Credentials::externalProcedure(_) => CredentialType::External,
                        _ => CredentialType::Unrecognized,
                    }
                },
            };
            let pwd_response: Option<PwdResponse> = match &r.parameter.pwdResponseValue {
                None => None,
                Some(p) => {
                    let warning: Option<PwdResponseWarning> = match &p.warning {
                        None => None,
                        Some(w) => match w {
                            PwdResponseValue_warning::graceRemaining(g) => {
                                let fallible_i = read_i64(&g);
                                if let Ok(i) = fallible_i {
                                    if i < 0 || i > (u32::MAX as i64) {
                                        None
                                    } else {
                                        Some(PwdResponseWarning::GraceRemaining(i as u32))
                                    }
                                } else {
                                    None
                                }
                            },
                            PwdResponseValue_warning::timeLeft(g) => {
                                let fallible_i = read_i64(&g);
                                if let Ok(i) = fallible_i {
                                    if i < 0 || i > (u32::MAX as i64) {
                                        None
                                    } else {
                                        Some(PwdResponseWarning::TimeLeft(i as u32))
                                    }
                                } else {
                                    None
                                }
                            },
                            _ => None
                        },
                    };

                    if let Some(e) = &p.error {
                        Some(PwdResponse{
                            warning,
                            error: Some(*e as u8),
                        })
                    } else {
                        Some(PwdResponse{
                            warning,
                            error: None,
                        })
                    }
                },
            };
            let signing_cert_path: Option<Vec<Base64String>> = None;
            let attr_cert_path: Option<Vec<Base64String>> = None;
            let mut responding_ae_title: Option<ApiGeneralName> = None;
            let mut responding_ap_invocation_identifier: Option<u32> = None;
            let mut responding_ae_invocation_identifier: Option<u32> = None;
            // TODO: Keep the raw bytes of the toBeSigned.
            // if let Some(creds) = &r.parameter.credentials {
            //     if let Credentials::strong(screds) = creds {
            //         if let Some(cert_path) = screds.certification_path {
            //             cert_path.userCertificate.
            //         }
            //     }
            // }
            // let signing_cert_path = match &r.parameter.credentials
            if let Some(ae_title) = &r.responding_ae_title {
                responding_ae_title = state.serialize_x500_gen_name(&ae_title).ok();
            }
            if let Some(responding_apii) = r.responding_ap_invocation_identifier {
                if let Ok(i) = read_i64(&responding_apii) {
                    if i >= 0 && i < (u32::MAX as i64) {
                        responding_ap_invocation_identifier = Some(i as u32);
                    }
                }
            }
            if let Some(responding_aeii) = r.responding_ae_invocation_identifier {
                if let Ok(i) = read_i64(&responding_aeii) {
                    if i >= 0 && i < (u32::MAX as i64) {
                        responding_ae_invocation_identifier = Some(i as u32);
                    }
                }
            }
            let b = BindResult{
                version: if v2 { 2 } else { 1 },
                session_id: String::from("REPLACE_ME"),
                tls_protection: TlsProtectionLevel::None,
                mtls: false,
                dsa_authenticated: false, // TODO: Support DSA authentication?
                dsa_credential_type,
                pwd_response,
                tls_cert_path: None,
                signing_cert_path,
                attr_cert_path,
                responding_ae_title,
                responding_ap_invocation_identifier,
                responding_ae_invocation_identifier,
            };
            let mut sessions = state.sessions.lock().await;
            sessions.insert(String::from("REPLACE_ME"), SessionState{
                client: Arc::new(Mutex::new(dap)),
            });
            Ok(b)
        },
        _ => return Err(DirectoryClientError::ApplicationAssociationRejected) // TODO: Change this.
    }
    // TODO: Attempt StartTLS.
}

async fn bind_via_url (state: &ServerSideState, url: &str) -> Result<BindResult, DirectoryClientError> {
    if url.to_lowercase().starts_with("idm://") {
        return bind_idm(state, &url[6..]).await;
    }
    Err(DirectoryClientError::MalformedUrl) // TODO: Define new error type: Unrecognized Scheme.
}

#[tauri::command]
pub async fn bind(
    state: tauri::State<'_, ServerSideState>,
    url: &str,
) -> Result<BindResult, DirectoryClientError> {
    bind_via_url(&state, &url).await
}
