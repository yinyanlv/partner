use actix_web::{Result, Json};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    NotFound,
    Unauthorized,
    DatabaseError
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterMessage {
    pub success: bool,
    pub message: String
}

impl RegisterMessage {
    
    pub fn respond(&self) ->  Result<Json<Self>> {

        Ok(Json(self.clone()))
    }
}
