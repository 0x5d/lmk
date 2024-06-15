use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client as HttpClient,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct PostMessageReq {
    pub channel: String,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostMessageRes {
    pub ok: bool,
    pub error: Option<String>,
    pub warning: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JoinConversationReq {
    pub channel: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConversationsJoinRes {
    pub ok: bool,
    pub error: Option<String>,
    pub warning: Option<String>,
}

pub struct Client {
    base_url: String,
    bot_token: String,
    http_client: HttpClient,
}

impl Client {
    /// Creates a new instances of Client.
    pub fn new(bot_token: String) -> Self {
        Client {
            bot_token,
            base_url: "https://slack.com/api/".into(),
            http_client: HttpClient::new(),
        }
    }

    /// https://api.slack.com/methods/conversations.join
    pub async fn join_conversation(
        &self,
        req: JoinConversationReq,
    ) -> Result<ConversationsJoinRes, Box<dyn Error>> {
        self.post("conversations.join", &req).await
    }

    /// https://api.slack.com/methods/chat.postMessage
    pub async fn post_message(
        &self,
        req: PostMessageReq,
    ) -> Result<PostMessageRes, Box<dyn Error>> {
        self.post("chat.postMessage", &req).await
    }

    async fn post<T: Serialize + ?Sized, U: DeserializeOwned>(
        &self,
        method: &str,
        req: &T,
    ) -> Result<U, Box<dyn Error>> {
        let res = self
            .http_client
            .post(format!("{}/{method}", self.base_url))
            .headers(self.common_headers())
            .json(req)
            .send()
            .await?
            .json::<U>()
            .await?;
        Ok(res)
    }

    fn common_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let auth = HeaderValue::from_str(format!("Bearer {}", &self.bot_token).as_str())
            .expect("Bot token contains invalid characters.");
        // Constant; safe to unwrap.
        let content_type = HeaderValue::from_str("application/json; charset=utf-8").unwrap();
        let _ = headers.insert(AUTHORIZATION, auth);
        let _ = headers.insert(CONTENT_TYPE, content_type);
        headers
    }
}
