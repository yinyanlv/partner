#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLogin {
    username: String,
    password: String
}

impl UserLogin {

    pub fn validate(&self) -> Result<String, String> {

    }
}