use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Apiv2Schema)]
pub struct SigningRequest {
    pub cert: String,
    #[serde(rename = "clientName")]
    pub client_name: String,
}
