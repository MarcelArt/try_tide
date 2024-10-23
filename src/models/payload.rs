use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Payload {
    pub name: String,
    pub value: u8,
}