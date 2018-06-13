use actix_web::{Result, Json};

pub type MessageResult<T> = Result<Json<Message<T>>>; 

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message<T> {
    Error {
        success: bool,
        message: String,
    },
    Success {
        success: bool,
        data: T
    }
}

impl<T> Message<T> {

    pub fn success(data: T) -> Result<Json<Message<T>>> {

        Ok(Json(Message::Success {
            success: true,
            data: data
        }))
    }

    pub fn error(msg: &str) -> Result<Json<Message<T>>> {

        Ok(Json(Message::Error {
            success: false,
            message: msg.to_owned()
        }))
    }
}