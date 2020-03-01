/*!
Support for the [JSON API v5 interface for Pandora](https://6xq.net/pandora-apidoc/json/).

Unless noted otherwise JSON-encoded requests sent by within the HTTP POST body
are encrypted using Blowfish ECB and converted to hexadecimal notation with
lowercase letters.
*/
// SPDX-License-Identifier: MIT AND WTFPL

pub mod accessory;
pub mod ad;
pub mod auth;
pub mod bookmark;
mod crypt;
pub mod device;
pub mod errors;
pub mod music;
pub mod station;
pub mod test;
pub mod track;
pub mod user;

use std::collections::HashMap;
use std::fmt::Debug;

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::errors::Error;
use crate::json::auth::{PartnerLogin, PartnerLoginResponse};
use crate::json::errors::{JsonError, JsonErrorKind};

/// A builder to construct the properties of an http request to Pandora.
#[derive(Debug, Clone)]
pub struct PandoraSession {
    client: reqwest::blocking::Client,
    endpoint_url: url::Url,
    tokens: SessionTokens,
    json: serde_json::value::Value,
    args: std::collections::BTreeMap<String, String>,
    encrypted: bool,
}

impl PandoraSession {
    /// Construct a new PandoraSession.
    pub fn new<T: ToEncryptionTokens, E: ToEndpoint>(
        client: Option<reqwest::blocking::Client>,
        to_encryption_tokens: &T,
        to_endpoint: &E,
    ) -> Self {
        Self {
            client: client.unwrap_or_else(reqwest::blocking::Client::new),
            endpoint_url: to_endpoint.to_endpoint_url(),
            tokens: SessionTokens::new(to_encryption_tokens),
            json: serde_json::value::Value::Object(serde_json::map::Map::new()),
            args: std::collections::BTreeMap::new(),
            encrypted: false,
        }
    }

    /// Create a new PandoraSession copying the endpoint and session values into the new
    /// object.
    pub fn copy_session(&self) -> Self {
        Self {
            client: self.client.clone(),
            endpoint_url: self.endpoint_url.clone(),
            tokens: self.tokens.clone(),
            json: serde_json::value::Value::Object(serde_json::map::Map::new()),
            args: std::collections::BTreeMap::new(),
            encrypted: false,
        }
    }

    /// Get a reference to the http client.
    pub fn http_client(&self) -> &reqwest::blocking::Client {
        &self.client
    }

    /// Set the Endpoint on this PandoraSession instance.
    pub fn endpoint<E: ToEndpoint>(&mut self, to_endpoint: E) -> &mut Self {
        self.endpoint_url = to_endpoint.to_endpoint_url();
        self
    }

    /// Get a mutable reference to the endpoint url::Url to update or make calls on it.
    pub fn endpoint_mut<E: ToEndpoint>(&mut self) -> &mut url::Url {
        &mut self.endpoint_url
    }

    /// Update the session partner tokens from type implementing ToPartnerTokens.
    pub fn update_partner_tokens<T: ToPartnerTokens>(&mut self, to_partner_tokens: &T) {
        self.tokens.update_partner_tokens(to_partner_tokens);
    }

    /// Update the session partner tokens from type implementing ToPartnerTokens.
    pub fn update_user_tokens<T: ToUserTokens>(&mut self, to_user_tokens: &T) {
        self.tokens.update_user_tokens(to_user_tokens);
    }

    /// Get a reference to the session tokens to check the state or make calls
    /// on it.
    pub fn session_tokens(&self) -> &SessionTokens {
        &self.tokens
    }

    /// Get a mutable reference to the session tokens to modify the state or
    /// make calls on it.
    pub fn session_tokens_mut(&mut self) -> &mut SessionTokens {
        &mut self.tokens
    }

    /// Set the json object on this PandoraSession instance.
    ///
    /// When build() is called, the json object will be updated with
    /// session keys from the session instance, if one was provided.
    pub fn json(&mut self, json: serde_json::value::Value) -> &mut Self {
        self.json = json;
        self
    }

    /// Get a mutable reference to the json to update or make calls on it.
    pub fn json_mut(&mut self) -> &mut serde_json::value::Value {
        &mut self.json
    }

    /// Add query arguments to the http request.
    pub fn arg(&mut self, key: &str, value: &str) -> &mut Self {
        self.args.insert(key.to_string(), value.to_string());
        self
    }

    /// Require that the request body be encrypted using the session instance, if any was set.  If
    /// no session instance is set, this will silently transmit the data unencrypted.
    pub fn encrypted(&mut self) -> &mut Self {
        self.encrypted = true;
        self
    }

    /// Merge necessary values from the session instance into the query arguments
    fn add_session_tokens_to_args(&mut self) {
        // auth_token arg should be set to user_token, if available, otherwise partner_token
        if let Some(auth_token) = self
            .tokens
            .user_token
            .clone()
            .or_else(|| self.tokens.partner_token.clone())
        {
            self.arg("auth_token", &auth_token);
        }
        if let Some(partner_id) = self.tokens.partner_id.clone() {
            self.arg("partner_id", &partner_id);
        }
        if let Some(user_id) = self.tokens.user_id.clone() {
            self.arg("user_id", &user_id);
        }
    }

    /// Merge necessary values from the session instance into the json body
    fn add_session_tokens_to_json(&mut self) {
        let json_obj = self
            .json
            .as_object_mut()
            .expect("Programming Error accessing API request json for modification.");
        if let Some(partner_auth_token) = self.tokens.partner_token.clone() {
            json_obj.insert(
                "partnerAuthToken".to_string(),
                serde_json::Value::String(partner_auth_token),
            );
        }
        if let Some(user_auth_token) = self.tokens.user_token.clone() {
            json_obj.insert(
                "userAuthToken".to_string(),
                serde_json::Value::String(user_auth_token),
            );
        }

        if let Some(sync_time) = self.tokens.sync_time {
            json_obj.insert("syncTime".to_string(), serde_json::Value::from(sync_time));
        }
    }

    /// Build a reqwest::blocking::Request, which can be inspected, modified, and executed with
    /// reqwest::blocking::Client::execute().
    pub fn build(&mut self) -> reqwest::blocking::RequestBuilder {
        self.add_session_tokens_to_args();
        let mut url: url::Url = self.endpoint_url.clone();
        url.query_pairs_mut().extend_pairs(&self.args);

        self.add_session_tokens_to_json();
        let mut body: String = self.json.to_string();
        //if cfg!(test) {
        //    println!("Request body: {:?}", body);
        //}
        if self.encrypted {
            body = self.tokens.encrypt(&body);
            //if cfg!(test) {
            //    println!("Encrypted body: {:?}", body);
            //}
        }

        self.client.post(url).body(body)
    }
}

/// A generic type to aid in converting the returned Json document from a
/// Pandora API call into a custom struct T that deserializes the content of
/// the API call result.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PandoraResponse<T> {
    /// The reported status of the call
    pub stat: PandoraStatus,
    /// The resulting content of the API call
    pub result: Option<T>,
    /// A message explaining the returned code
    pub message: Option<String>,
    /// A numeric error code
    pub code: Option<u32>,
}

impl<T: serde::de::DeserializeOwned> Into<std::result::Result<T, JsonError>>
    for PandoraResponse<T>
{
    fn into(self) -> std::result::Result<T, JsonError> {
        match self {
            PandoraResponse {
                stat: PandoraStatus::Ok,
                result: Some(result),
                ..
            } => Ok(result),
            PandoraResponse {
                stat: PandoraStatus::Ok,
                result: None,
                ..
            } => {
                let default_value = serde_json::json!({});
                let deser = serde_json::from_value(default_value);
                deser.map_err(|_| JsonError::new(None, Some(String::from("Invalid JSON content."))))
            }
            PandoraResponse { code, message, .. } => Err(JsonError::new(code, message)),
        }
    }
}

/// The status string returned by the Pandora JSON API call.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PandoraStatus {
    /// The API method call succeeded
    Ok,
    /// The API method call failed
    Fail,
}

/// A trait for accessing information and capabilities specific to each
/// Pandora JSON API call, including the method name, the json body content,
/// and whether the body content should be encrypted before transmission.
///
/// It also includes two convenience methods for submitting the request.
pub trait PandoraApiRequest: serde::ser::Serialize {
    /// The type that the json response will be deserialized to.
    type Response: Debug + serde::de::DeserializeOwned;
    /// The Error type to be returned by fallible calls on this trait.
    type Error: Debug + From<serde_json::error::Error> + From<reqwest::Error> + From<JsonError>;

    /// Returns the name of the Pandora JSON API call in the form that it must
    /// appear when making that call.
    fn get_method(&self) -> String;

    /// Returns the root json Value that should be serialized into the body of
    /// the API call.
    fn get_json(&self) -> std::result::Result<serde_json::value::Value, Self::Error> {
        serde_json::to_value(self).map_err(Self::Error::from)
    }

    /// Whether the json body of the API call is expected to be encrypted before
    /// transmission.
    fn encrypt_request(&self) -> bool {
        false
    }

    /// Generate an HTTP request that, when send() is called on it, will submit
    /// the built request.
    fn request(
        &self,
        session: &mut PandoraSession,
    ) -> std::result::Result<reqwest::blocking::RequestBuilder, Self::Error> {
        let mut tmp_session = session.clone();
        tmp_session
            .arg("method", &self.get_method())
            .json(self.get_json()?);
        if self.encrypt_request() {
            tmp_session.encrypted();
        }
        Ok(tmp_session.build())
    }

    /// Build the request, submit it, and extract the response content from the
    /// body json, and deserialize it into the Self::Response type.
    fn response(
        &self,
        session: &mut PandoraSession,
    ) -> std::result::Result<Self::Response, Self::Error> {
        let response = self.request(session)?.send().map_err(Self::Error::from)?;
        response.error_for_status_ref().map_err(Self::Error::from)?;

        let response_obj: PandoraResponse<Self::Response> = if cfg!(test) {
            // Debugging support - output full response text before attempting
            // deserialization
            let response_body = response.text()?;
            if cfg!(test) {
                //println!("Full response: {:?}", response_body);
            }
            serde_json::from_slice(response_body.as_bytes())?
        } else {
            // Regular builds just grab the json directly.
            response.json()?
        };

        if cfg!(test) {
            //println!("Json response: {:?}", response_obj);
        }

        let result: std::result::Result<Self::Response, JsonError> = response_obj.into();
        // Detect errors that indicate that our session tokens aren't valid, and clear them
        match result {
            Err(JsonError {
                kind: JsonErrorKind::InvalidAuthToken,
                message,
            }) => {
                session.session_tokens_mut().clear_partner_tokens();
                session.session_tokens_mut().clear_user_tokens();
                Err(JsonError {
                    kind: JsonErrorKind::InvalidAuthToken,
                    message,
                })
            }
            res => res,
        }
        .map_err(Self::Error::from)
    }
}

/// Trait for getting the JSON API endpoint specific to the partner account
/// being used for the service
pub trait ToEndpoint: serde::ser::Serialize {
    /// Generate a string describing the API endpoint to be used.
    fn to_endpoint(&self) -> String;
    /// Generate a url::Url for the API endpoint to be used.
    fn to_endpoint_url(&self) -> url::Url {
        url::Url::parse(&self.to_endpoint()).expect("Error parsing Pandora endpoint url.")
    }
}

impl ToEndpoint for String {
    /// Generate a string describing the API endpoint to be used.
    fn to_endpoint(&self) -> String {
        self.clone()
    }
}

/// This trait is used to provide access to the tokens needed to initiate
/// a partnerLogin.
pub trait ToEncryptionTokens {
    /// Returns the encryption key to be used for this session.
    fn to_encrypt_key(&self) -> String;
    /// Encrypt the provided data using the session encryption key.
    fn encrypt(&self, data: &str) -> String {
        crypt::encrypt(&self.to_encrypt_key(), data)
    }
    /// Returns the decryption key to be used for this session.
    fn to_decrypt_key(&self) -> String;
    /// Decrypt the provided data using the session decryption key.
    fn decrypt(&self, hex_data: &str) -> Vec<u8> {
        crypt::decrypt(&self.to_decrypt_key(), hex_data)
    }
}

/// This trait is used to provide access to all the tokens needed to track
/// the partner (application) session.
pub trait ToPartnerTokens {
    /// Return the partner id for the session, if one has been already been set.
    ///
    /// Returns None otherwise.
    fn to_partner_id(&self) -> Option<String>;

    /// Return the partner token for the session, if one has been already been set.
    ///
    /// Returns None otherwise.
    fn to_partner_token(&self) -> Option<String>;

    /// Return the session sync time as a u64, if one has been already been set.
    ///
    /// Returns None otherwise.
    fn to_sync_time(&self) -> Option<String>;
}

/// This trait is used to provide access to all the tokens needed to track
/// the user session.
pub trait ToUserTokens {
    /// Return the user id for the session, if one has been already been set.
    ///
    /// Returns None otherwise.
    fn to_user_id(&self) -> Option<String>;

    /// Return the user token for the session, if one has been already been set.
    ///
    /// Returns None otherwise.
    fn to_user_token(&self) -> Option<String>;
}

/// Trait for providing access to a station token.
pub trait ToStationToken: serde::ser::Serialize {
    /// Return the station token as a String.
    fn to_station_token(&self) -> String;
}

impl ToStationToken for String {
    /// Allow for using strings with functions accepting ToStationToken.
    fn to_station_token(&self) -> String {
        self.clone()
    }
}

impl ToStationToken for &str {
    /// Allow for using string slices with functions accepting ToStationToken.
    fn to_station_token(&self) -> String {
        // Clippy says it's faster to dereference self first before calling
        // to_string() when self is a &&str
        (*self).to_string()
    }
}

/// Trait for providing access to one or more ad tracking tokens.
pub trait ToTrackingToken: serde::ser::Serialize {
    /// Return the ad tracking tokens as a String.
    fn to_ad_tracking_tokens(&self) -> String;
}

impl ToTrackingToken for String {
    /// Allow for using strings with functions accepting ToTrackingToken.
    fn to_ad_tracking_tokens(&self) -> String {
        self.clone()
    }
}

impl ToTrackingToken for &str {
    /// Allow for using string slices with functions accepting ToTrackingToken.
    fn to_ad_tracking_tokens(&self) -> String {
        // Clippy says it's faster to dereference self first before calling
        // to_string() when self is a &&str
        (*self).to_string()
    }
}

/// A convenience type that can produce valid PartnerLogin instances for a
/// number of different endpoints and device types.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Partner {
    /// The partner login name (not the account-holder's username)
    /// used to authenticate the application with the Pandora service.
    pub username: String,
    /// The partner login password (not the account-holder's username)
    /// used to authenticate the application with the Pandora service.
    pub password: String,
    /// The partner device model name.
    pub device_model: String,
    /// The Pandora JSON API version
    pub version: String,
    /// The encryption key associated with this partner login
    #[serde(skip)]
    pub encrypt_password: String,
    /// The decryption key associated with this partner login
    #[serde(skip)]
    pub decrypt_password: String,
    /// The hostname for the endpoint used to communicate with the Pandora API.
    /// This is a bare hostname, without scheme/protocol.  This value will later
    /// be combined with a scheme and path to create a complete, valid URL.
    #[serde(skip)]
    pub endpoint_host: String,
}

impl Partner {
    /// Generate a Partner instance using the android app credentials
    pub fn new_android() -> Self {
        Self {
            username: "android".to_string(),
            password: "AC7IBG09A3DTSYM4R41UJWL07VLN8JI7".to_string(),
            device_model: "android-generic".to_string(),
            version: "5".to_string(),
            decrypt_password: "R=U!LH$O2B#".to_string(),
            encrypt_password: "6#26FRL$ZWD".to_string(),
            endpoint_host: "tuner.pandora.com".to_string(),
        }
    }

    /// Generate a Partner instance using the iOS app credentials
    pub fn new_ios() -> Self {
        Self {
            username: "iphone".to_string(),
            password: "P2E4FC0EAD3*878N92B2CDp34I0B1@388137C".to_string(),
            device_model: "IP01".to_string(),
            version: "5".to_string(),
            decrypt_password: "20zE1E47BE57$51".to_string(),
            encrypt_password: "721^26xE22776".to_string(),
            endpoint_host: "tuner.pandora.com".to_string(),
        }
    }

    /// Generate a Partner instance using the Palm Pre credentials
    pub fn new_palm() -> Self {
        Self {
            username: "palm".to_string(),
            password: "IUC7IBG09A3JTSYM4N11UJWL07VLH8JP0".to_string(),
            device_model: "pre".to_string(),
            version: "5".to_string(),
            decrypt_password: "E#U$MY$O2B=".to_string(),
            encrypt_password: "%526CBL$ZU3".to_string(),
            endpoint_host: "tuner.pandora.com".to_string(),
        }
    }

    /// Generate a Partner instance using the Windows Mobile credentials
    pub fn new_windows_mobile() -> Self {
        Self {
            username: "winmo".to_string(),
            password: "ED227E10a628EB0E8Pm825Dw7114AC39".to_string(),
            device_model: "VERIZON_MOTOQ9C".to_string(),
            version: "5".to_string(),
            decrypt_password: "7D671jt0C5E5d251".to_string(),
            encrypt_password: "v93C8C2s12E0EBD".to_string(),
            endpoint_host: "tuner.pandora.com".to_string(),
        }
    }

    /// Generate a Partner instance using the Desktop AIR credentials
    pub fn new_desktop_air() -> Self {
        Self {
            username: "pandora one".to_string(),
            password: "TVCKIBGS9AO9TSYLNNFUML0743LH82D".to_string(),
            device_model: "D01".to_string(),
            version: "5".to_string(),
            decrypt_password: "U#IO$RZPAB%VX2".to_string(),
            encrypt_password: "2%3WCL*JU$MP]4".to_string(),
            endpoint_host: "internal-tuner.pandora.com".to_string(),
        }
    }

    /// Generate a Partner instance using the Vista widget credentials
    pub fn new_vista_widget() -> Self {
        Self {
            username: "windowsgadget".to_string(),
            password: "EVCCIBGS9AOJTSYMNNFUML07VLH8JYP0".to_string(),
            device_model: "WG01".to_string(),
            version: "5".to_string(),
            decrypt_password: "E#IO$MYZOAB%FVR2".to_string(),
            encrypt_password: "%22CML*ZU$8YXP[1".to_string(),
            endpoint_host: "internal-tuner.pandora.com".to_string(),
        }
    }

    /// Initialize a PandoraSession using the corresponding Partner
    /// tokens and endpoint.
    pub fn init_session(&self) -> PandoraSession {
        PandoraSession::new(None, self, self)
    }

    /// Generate a PartnerLogin instance from this object that can be
    /// used for initiating authentication with the service.
    pub fn to_partner_login(&self) -> PartnerLogin {
        PartnerLogin {
            username: self.username.clone(),
            password: self.password.clone(),
            device_model: self.device_model.clone(),
            version: self.version.clone(),
            optional: HashMap::new(),
        }
    }

    /// Convenience method for submitting the partner login request for this
    /// partner.
    pub fn login(&self, session: &mut PandoraSession) -> Result<PartnerLoginResponse, Error> {
        let response = self.to_partner_login().response(session)?;
        session.update_partner_tokens(&response);
        Ok(response)
    }
}

impl Default for Partner {
    /// Create a default Partner instance using the android device type.
    fn default() -> Self {
        Self::new_android()
    }
}

impl ToEncryptionTokens for Partner {
    fn to_encrypt_key(&self) -> String {
        self.encrypt_password.clone()
    }

    fn to_decrypt_key(&self) -> String {
        self.decrypt_password.clone()
    }
}

impl ToEndpoint for Partner {
    /// Returns the service endpoint to be used for this session as a String.
    fn to_endpoint(&self) -> String {
        format!("https://{}/services/json", self.endpoint_host)
    }
}

/// A convenience type that holds all the values necessary to maintain an active
/// session with the Pandora service.
#[derive(Debug, Clone)]
pub struct SessionTokens {
    /// The key used to encrypt the body of certain API requests.
    pub encrypt_key: String,
    /// The key used to decrypt certain values from the body of certain API
    /// responses.
    pub decrypt_key: String,
    /// The partner id token returned by the partner login request
    pub partner_id: Option<String>,
    /// The partner auth token returned by the partner login request
    pub partner_token: Option<String>,
    /// The sync time token returned by the partner login request.  This is
    /// private so that it will be updated/read by accessor methods that
    /// correctly adjust for the time offset that needs to be added on.
    ///
    /// Also note that this is the decrypted form, not the encrypted bytes
    /// passed around on types that implement ToPartnerTokens.
    sync_time: Option<u64>,
    /// The instant when the sync_time was set, so that when we return sync_time,
    /// we return the value offset by however much time has passed since we were
    /// issued the token.
    local_time_base: Option<std::time::Instant>,
    /// The user id token returned by the user login request
    pub user_id: Option<String>,
    /// The user auth token returned by the user login request
    pub user_token: Option<String>,
}

impl SessionTokens {
    /// Initialize a SessionTokens object with only the encryption keys,
    /// as those are needed even before authentication begins
    pub fn new<T: ToEncryptionTokens>(to_encryption_tokens: &T) -> Self {
        Self {
            encrypt_key: to_encryption_tokens.to_encrypt_key(),
            decrypt_key: to_encryption_tokens.to_decrypt_key(),
            partner_id: None,
            partner_token: None,
            sync_time: None,
            local_time_base: None,
            user_id: None,
            user_token: None,
        }
    }

    /// Update the current SessionTokens instance using values from the
    /// response to the PartnerLogin request.
    pub fn update_partner_tokens<T: ToPartnerTokens>(&mut self, to_partner_tokens: &T) {
        self.partner_id = to_partner_tokens.to_partner_id();
        self.partner_token = to_partner_tokens.to_partner_token();
        // The first four bytes are, reportedly, garbage, but I suspect it's
        // actually supposed to function as a salt that was intended to make it
        // difficult to recover the decryption keys.
        if let Some(sync_time) = to_partner_tokens.to_sync_time() {
            let sync_time_bytes: Vec<u8> =
                self.decrypt(&sync_time).iter().skip(4).cloned().collect();
            let sync_time_str = std::str::from_utf8(&sync_time_bytes).unwrap_or("0");
            self.set_sync_time(sync_time_str.parse::<u64>().unwrap_or(0));
        }
    }

    /// Update the current SessionTokens instance using values from the
    /// response to the UserLogin request.
    pub fn update_user_tokens<T: ToUserTokens>(&mut self, to_user_tokens: &T) {
        self.user_id = to_user_tokens.to_user_id();
        self.user_token = to_user_tokens.to_user_token();
    }

    /// The current server time as of the last request.  Submitted requests must
    /// include a value of syncTime that corresponds to the new server time,
    /// based on the amount of time elapsed since authenticating.
    pub fn set_sync_time(&mut self, sync_time: u64) {
        self.local_time_base = Some(std::time::Instant::now());
        self.sync_time = Some(sync_time);
    }

    /// Clear the session syncTime base.
    pub fn clear_sync_time(&mut self) {
        self.local_time_base = None;
        self.sync_time = None;
    }

    /// Returns the current syncTime relative to the
    pub fn get_sync_time(&self) -> Option<u64> {
        self.sync_time
            .and_then(|st| self.local_time_base.map(|ltb| ltb.elapsed().as_secs() + st))
    }

    /// Clears all active partner session tokens.
    pub fn clear_partner_tokens(&mut self) {
        self.partner_id = None;
        self.partner_token = None;
        self.clear_sync_time();
    }

    /// Clears all active user session tokens.
    pub fn clear_user_tokens(&mut self) {
        self.user_id = None;
        self.user_token = None;
    }
}

impl ToEncryptionTokens for SessionTokens {
    /// Retrieve the encryption key for this session
    fn to_encrypt_key(&self) -> String {
        self.encrypt_key.clone()
    }
    /// Retrieve the decryption key for this session
    fn to_decrypt_key(&self) -> String {
        self.decrypt_key.clone()
    }
}

impl<T: ToEncryptionTokens> From<&T> for SessionTokens {
    fn from(tokens: &T) -> Self {
        Self::new(tokens)
    }
}

/// A representation of a moment in time.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timestamp {
    /// The offset from UTC in minutes
    timezone_offset: u32,
    /// Unix epoch time for the timezone offset
    time: i64,
    /// Year, adjusted for timezone offset
    year: u32,
    /// Month, adjusted for timezone offset
    month: u8,
    /// Day of month, adjusted for timezone offset
    day: u8,
    /// Hour, adjusted for timezone offset
    hours: u8,
    /// Minute, adjusted for timezone offset
    minutes: u8,
    /// Seconds, adjusted for timezone offset
    seconds: u8,
    /// Unknown
    date: u8,
}

impl Into<chrono::DateTime<chrono::Utc>> for Timestamp {
    fn into(self) -> chrono::DateTime<chrono::Utc> {
        // TODO: Figure out proper handling of timezoneOffset
        // e.g. is it signed? is the provided time Utc (and offset is applied
        // to get local) or is it local (and tells the offset used to determine
        // local)? is it the local time of the user, or the local time for the
        // system that generated the timestamp?
        let naive_dt = chrono::NaiveDateTime::from_timestamp(self.time, 0);
        chrono::DateTime::<chrono::Utc>::from_utc(naive_dt, chrono::Utc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::errors::Error;
    use crate::json::auth::user_login;

    // TODO: lazy_static create a single session and return a RcRefCell to
    // it instead.  I suspect that some of the transient
    // InsufficientConnectivity errors are resulting from simultaneously
    // creating a large number of sessions, creating race conditions or
    // invalidating tokens.
    pub fn session_login(partner: &Partner) -> Result<PandoraSession, Error> {
        let mut session = partner.init_session();
        let _partner_login = partner.login(&mut session)?;

        let test_username_raw = include_str!("../../test_username.txt");
        let test_username = test_username_raw.trim();
        let test_password_raw = include_str!("../../test_password.txt");
        let test_password = test_password_raw.trim();

        let user_login = user_login(&mut session, &test_username, &test_password)?;
        session.update_user_tokens(&user_login);
        Ok(session)
    }

    #[test]
    fn partner_test() {
        let partner = Partner::default();
        let mut session = partner.init_session();
        let partner_login = partner
            .login(&mut session)
            .expect("Failed while performing partner login");
        session.update_partner_tokens(&partner_login);
    }
}
