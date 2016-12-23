// Authentication utilities for Last.fm Scrobble API 2.0

pub struct AuthCredentials {
    // Application specific key & secret
    api_key: String,
    api_secret: String,

    // Individual user's username & pass
    username: String,
    password: String,

    // Dynamic parameter not included until we're authenticated
    session_key: Option<String>
}

impl AuthCredentials {

    pub fn new_partial(api_key: String, api_secret: String) -> AuthCredentials {
        AuthCredentials{
            api_key: api_key,
            api_secret: api_secret,

            username: String::new(),
            password: String::new(),

            session_key: None
        }
    }

    pub fn set_user_credentials(&mut self, username: String, password: String) {
        self.username = username;
        self.password = password;

        // Invalidate session because we have new credentials
        self.session_key = None
    }

    // Returns true if there's enough valid data to attempt authentication (ignores session key)
    pub fn is_valid(&self) -> bool {
        self.api_key.len() > 0 && self.api_secret.len() > 0 && self.username.len() > 0
            && self.password.len() > 0
    }

    // Returns true if we have valid authentication parameters AND a session token
    pub fn is_authenticated(&self) -> bool {
        self.is_valid() && self.session_key.is_some()
    }

}