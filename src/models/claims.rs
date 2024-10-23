use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user: User,
    pub exp: usize,
}