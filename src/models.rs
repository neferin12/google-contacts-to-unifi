use serde::Serialize;

#[derive(Debug, Serialize, Default, PartialEq)]
pub struct UnifiContact {
    pub first_name: String,
    pub last_name: String,
    pub company: String,
    pub job_title: String,
    pub email: String,
    pub mobile_number: String,
    pub home_number: String,
    pub work_number: String,
    pub fax_number: String,
    pub other_number: String,
}
