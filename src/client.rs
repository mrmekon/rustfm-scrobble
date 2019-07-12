// Last.fm scrobble API 2.0 client

use std::collections::HashMap;
use http::{http, HttpMethod, HttpResponse, QueryString};
use serde_json;

use auth::AuthCredentials;
use models::responses::{AuthResponse, SessionResponse, NowPlayingResponse,
                        NowPlayingResponseWrapper, ScrobbleResponse, ScrobbleResponseWrapper,
                        BatchScrobbleResponse, BatchScrobbleResponseWrapper};

pub enum ApiOperation {
    AuthWebSession,
    AuthMobileSession,
    NowPlaying,
    Scrobble,
}

impl ApiOperation {
    fn to_string(&self) -> String {
        match *self {
            ApiOperation::AuthWebSession => "auth.getSession",
            ApiOperation::AuthMobileSession => "auth.getMobileSession",
            ApiOperation::NowPlaying => "track.updateNowPlaying",
            ApiOperation::Scrobble => "track.scrobble",
        }
        .to_string()
    }
}

pub struct LastFmClient {
    auth: AuthCredentials,
}

impl LastFmClient {
    pub fn new(api_key: String, api_secret: String) -> LastFmClient {
        let partial_auth = AuthCredentials::new_partial(api_key, api_secret);

        LastFmClient {
            auth: partial_auth,
        }
    }

    pub fn set_user_credentials(&mut self, username: String, password: String) {
        self.auth.set_user_credentials(username, password);
    }

    pub fn set_user_token(&mut self, token: String) {
        self.auth.set_user_token(token);
    }

    pub fn authenticate_with_password(&mut self) -> Result<SessionResponse, String> {
        let params = self.auth.get_auth_request_params()?;

        match self.api_request(ApiOperation::AuthMobileSession, params) {
            Ok(body) => {
                let decoded: AuthResponse = serde_json::from_str(body.as_str()).unwrap();
                self.auth.set_session_key(decoded.session.clone().key);

                Ok(decoded.session)
            }
            Err(msg) => Err(format!("Authentication failed: {}", msg)),
        }
    }

    pub fn authenticate_with_token(&mut self) -> Result<SessionResponse, String> {
        let params = self.auth.get_auth_request_params()?;

        match self.api_request(ApiOperation::AuthWebSession, params) {
            Ok(body) => {
                let decoded: AuthResponse = serde_json::from_str(body.as_str()).unwrap();
                self.auth.set_session_key(decoded.session.clone().key);

                Ok(decoded.session)
            }
            Err(msg) => Err(format!("Authentication failed: {}", msg)),
        }
    }

    pub fn authenticate_with_session_key(&mut self, session_key: String) {
        // TODO: How to verify session key at this point?
        self.auth.set_session_key(session_key)
    }

    pub fn session_key(&self) -> Option<String> {
        self.auth.session_key()
    }

    pub fn send_now_playing(&self,
                            params: &HashMap<String, String>)
                            -> Result<NowPlayingResponse, String> {
        match self.send_authenticated_request(ApiOperation::NowPlaying, params) {
            Ok(body) => {
                let decoded: NowPlayingResponseWrapper = serde_json::from_str(body.as_str())
                    .unwrap();
                Ok(decoded.nowplaying)
            }
            Err(msg) => Err(format!("Now playing request failed: {}", msg)),
        }
    }

    pub fn send_scrobble(&self,
                         params: &HashMap<String, String>)
                         -> Result<ScrobbleResponse, String> {
        match self.send_authenticated_request(ApiOperation::Scrobble, params) {
            Ok(body) => {
                let decoded: ScrobbleResponseWrapper = serde_json::from_str(body.as_str()).unwrap();
                Ok(decoded.scrobbles.scrobble)
            }
            Err(msg) => Err(format!("Scrobble request failed: {}", msg)),
        }
    }

    pub fn send_batch_scrobbles(&self,
                         params: &HashMap<String, String>)
                         -> Result<BatchScrobbleResponse, String> {
        match self.send_authenticated_request(ApiOperation::Scrobble, params) {
            Ok(body) => {
                let wrapper: BatchScrobbleResponseWrapper = serde_json::from_str(body.as_str()).unwrap();
                Ok(BatchScrobbleResponse {
                    scrobbles: wrapper.scrobbles.scrobbles
                })
            }
            Err(msg) => Err(format!("Batch scrobble request failed: {}", msg)),
        }
    }

    pub fn send_authenticated_request(&self,
                                      operation: ApiOperation,
                                      params: &HashMap<String, String>)
                                      -> Result<String, String> {
        if !self.auth.is_authenticated() {
            return Err("Not authenticated".to_string());
        }

        let mut req_params = self.auth.get_request_params();
        for (k, v) in params {
            req_params.insert(k.clone(), v.clone());
        }

        self.api_request(operation, req_params)
    }

    fn api_request(&self, operation: ApiOperation, params: HashMap<String, String>) -> Result<String, String> {
        match self.send_request(operation, params) {
            Ok(resp) => {
                let status = resp.code.unwrap_or(500);
                if status != 200 {
                    return Err(format!("Non Success status ({})", status));
                }

                match resp.data {
                    Ok(resp) => Ok(resp),
                    Err(_) => Err("Failed to read response body".to_string())
                }
            },
            Err(msg) => Err(format!("{}", msg))
        }
    }

    fn send_request(&self, operation: ApiOperation, params: HashMap<String, String>) -> Result<HttpResponse, std::io::Error> {
        let url = "https://ws.audioscrobbler.com/2.0/?format=json";
        let signature = self.auth.get_signature(operation.to_string(), &params);

        let mut req_params = params.clone();
        req_params.insert("method".to_string(), operation.to_string());
        req_params.insert("api_sig".to_string(), signature);

        let mut query = QueryString::new();
        for (key, value) in &req_params {
            query.add(key, value);
        }
        let query = query.build();
        Ok(http(url, Some(&query), None, HttpMethod::POST))
    }

}
