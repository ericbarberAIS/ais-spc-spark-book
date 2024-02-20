use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct CompanyData {
    pub company_name: String,
    pub contact_email: String,
    pub contact_name: String,
}
