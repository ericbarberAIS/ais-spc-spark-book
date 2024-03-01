use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CompanyData {
    pub company_name: String,
    pub contact_email: String,
    pub contact_name: String,
}
