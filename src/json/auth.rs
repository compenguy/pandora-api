/*!
Authentication/authorization support messages.
*/
// SPDX-License-Identifier: MIT AND WTFPL

use std::collections::HashMap;

use pandora_api_derive::PandoraRequest;
use serde::{Deserialize, Serialize};

use crate::errors::Error;
use crate::json::{PandoraApiRequest, PandoraSession, ToPartnerTokens, ToUserTokens};

/// **Unsupported!**
/// Undocumented method
/// [auth.getAdMetadata()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct GetAdMetadataUnsupported {}

/// **Unsupported!**
/// Undocumented method
/// [auth.partnerAdminLogin()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct PartnerAdminLoginUnsupported {}

/// This request additionally serves as API version validation, time synchronization and endpoint detection and must be sent over a TLS-encrypted link. The POST body however is not encrypted.
///
/// | Name | Type | Description |
/// | username | string | See Partner passwords |
/// | password | string | See Partner passwords |
/// | deviceModel | string | See Partner passwords |
/// | version | string | Current version number, “5”. |
/// | includeUrls | boolean |  |
/// | returnDeviceType | boolean |  |
/// | returnUpdatePromptVersions | boolean |  |
/// ``` json
/// {
///     "username": "pandora one",
///     "password": "TVCKIBGS9AO9TSYLNNFUML0743LH82D",
///     "deviceModel": "D01",
///     "version": "5"
/// }
/// ```
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[serde(rename_all = "camelCase")]
pub struct PartnerLogin {
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
    /// Optional parameters on the call
    #[serde(flatten)]
    pub optional: HashMap<String, serde_json::value::Value>,
}

impl PartnerLogin {
    /// Create a new PartnerLogin with some values. All Optional fields are
    /// set to None.
    pub fn new(
        username: &str,
        password: &str,
        device_model: &str,
        version: Option<String>,
    ) -> Self {
        PartnerLogin {
            username: username.to_string(),
            password: password.to_string(),
            device_model: device_model.to_string(),
            version: version.unwrap_or_else(|| String::from("5")),
            optional: HashMap::new(),
        }
    }

    /// Convenience function for setting boolean flags in the request. (Chaining call)
    pub fn and_boolean_option(mut self, option: &str, value: bool) -> Self {
        self.optional
            .insert(option.to_string(), serde_json::value::Value::from(value));
        self
    }

    /// Whether to request to include urls in the response. (Chaining call)
    pub fn include_urls(self, value: bool) -> Self {
        self.and_boolean_option("includeUrls", value)
    }

    /// Whether to request to include the device type in the response. (Chaining call)
    pub fn return_device_type(self, value: bool) -> Self {
        self.and_boolean_option("returnDeviceType", value)
    }

    /// Whether to request to return a prompt to update versions in the response. (Chaining call)
    pub fn return_update_prompt_versions(self, value: bool) -> Self {
        self.and_boolean_option("returnUpdatePromptVersions", value)
    }

    /// This is a wrapper around the `response` method from the
    /// PandoraApiRequest trait that automatically merges the partner tokens
    /// from the response back into the session.
    pub async fn merge_response(
        &self,
        session: &mut PandoraSession,
    ) -> Result<PartnerLoginResponse, Error> {
        let response = self.response(session).await?;
        session.update_partner_tokens(&response);
        Ok(response)
    }
}

/// syncTime is used to calculate the server time, see synctime. partnerId and authToken are required to proceed with user authentication.
///
/// | Name | Type | Description |
/// | syncTime | string | Hex-encoded, encrypted server time. Decrypt with password from Partner passwords and skip first four bytes of garbage. |
/// | partnerAuthToken | string |   |
/// | partnerId | string |   |
/// ``` json
/// {
///     "stat": "ok",
///     "result": {
///         "syncTime": "6923e263a8c3ac690646146b50065f43",
///         "deviceProperties": {
///             "videoAdRefreshInterval": 900,
///             "videoAdUniqueInterval": 0,
///             "adRefreshInterval": 5,
///             "videoAdStartInterval": 180
///         },
///         "partnerAuthToken": "VAzrFQTtsy3BQ3K+3iqFi0WF5HA63B1nFA",
///         "partnerId": "42",
///         "stationSkipUnit": "hour",
///         "urls": {
///             "autoComplete": "http://autocomplete.pandora.com/search"
///         },
///         "stationSkipLimit": 6
///     }
/// }
/// ```
/// | Code | Description |
/// | 1002 | INVALID_PARTNER_LOGIN. Invalid partner credentials. |
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartnerLoginResponse {
    /// The partner id that should be used for this session
    pub partner_id: String,
    /// The partner auth token that should be used for this session
    pub partner_auth_token: String,
    /// The server sync time that should be used for this session
    /// Note that this field is encrypted, and must be decrypted before use
    pub sync_time: String,
    /// Unknown field
    pub station_skip_unit: String,
    /// Unknown field
    pub station_skip_limit: u32,
    /// Unknown field
    pub urls: Option<HashMap<String, String>>,
    /// Optional response fields
    #[serde(flatten)]
    pub optional: HashMap<String, serde_json::value::Value>,
}

impl ToPartnerTokens for PartnerLoginResponse {
    fn to_partner_id(&self) -> Option<String> {
        Some(self.partner_id.clone())
    }

    fn to_partner_token(&self) -> Option<String> {
        Some(self.partner_auth_token.clone())
    }

    fn to_sync_time(&self) -> Option<String> {
        Some(self.sync_time.clone())
    }
}

/// Convenience function to do a basic partnerLogin call.
pub async fn partner_login(
    session: &mut PandoraSession,
    username: &str,
    password: &str,
    device_model: &str,
) -> Result<PartnerLoginResponse, Error> {
    PartnerLogin::new(username, password, device_model, None)
        .include_urls(false)
        .return_device_type(false)
        .return_update_prompt_versions(false)
        .merge_response(session)
        .await
}

/// This request *must* be sent over a TLS-encrypted link. It authenticates the Pandora user by sending his username, usually his email address, and password as well as the partnerAuthToken obtained by Partner login.
///
/// Additional response data can be requested by setting flags listed below.
///
/// | Name | Type | Description |
/// | loginType | string | “user” |
/// | username | string | Username |
/// | password | string | User’s password |
/// | partnerAuthToken | string | Partner token obtained by Partner login |
/// | returnGenreStations | boolean | (optional) |
/// | returnCapped | boolean | return isCapped parameter (optional) |
/// | includePandoraOneInfo | boolean | (optional) |
/// | includeDemographics | boolean | (optional) |
/// | includeAdAttributes | boolean | (optional) |
/// | returnStationList | boolean | Return station list, see Retrieve station list (optional) |
/// | includeStationArtUrl | boolean | (optional) |
/// | includeStationSeeds | boolean | (optional) |
/// | includeShuffleInsteadOfQuickMix | boolean | (optional) |
/// | stationArtSize | string | W130H130(optional) |
/// | returnCollectTrackLifetimeStats | boolean | (optional) |
/// | returnIsSubscriber | boolean | (optional) |
/// | xplatformAdCapable | boolean | (optional) |
/// | complimentarySponsorSupported | boolean | (optional) |
/// | includeSubscriptionExpiration | boolean | (optional) |
/// | returnHasUsedTrial | boolean | (optional) |
/// | returnUserstate | boolean | (optional) |
/// | includeAccountMessage | boolean | (optional) |
/// | includeUserWebname | boolean | (optional) |
/// | includeListeningHours | boolean | (optional) |
/// | includeFacebook | boolean | (optional) |
/// | includeTwitter | boolean | (optional) |
/// | includeDailySkipLimit | boolean | (optional) |
/// | includeSkipDelay | boolean | (optional) |
/// | includeGoogleplay | boolean | (optional) |
/// | includeShowUserRecommendations | boolean | (optional) |
/// | includeAdvertiserAttributes | boolean | (optional) |
/// ``` json
/// {
///    "loginType": "user",
///    "username": "user@example.com",
///    "password": "example",
///    "partnerAuthToken": "VAzrFQTtsy3BQ3K+3iqFi0WF5HA63B1nFA",
///    "includePandoraOneInfo":true,
///    "includeAdAttributes":true,
///    "includeSubscriptionExpiration":true,
///    "includeStationArtUrl":true,
///    "returnStationList":true,
///    "returnGenreStations":true,
///    "syncTime": 1335777573
/// }
/// ```
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[pandora_request(encrypted = true)]
#[serde(rename_all = "camelCase")]
pub struct UserLogin {
    /// This field should always have the value `user`.
    pub login_type: String,
    /// The account username to login with.
    pub username: String,
    /// The account password to login with.
    pub password: String,
    /// Optional parameters on the call
    #[serde(flatten)]
    pub optional: HashMap<String, serde_json::value::Value>,
}

impl UserLogin {
    /// Initialize a basic UserLogin request. All optional fields are set to None.
    pub fn new(username: &str, password: &str) -> Self {
        UserLogin {
            // This field should always have the value `user`.
            login_type: "user".to_string(),
            username: username.to_string(),
            password: password.to_string(),
            optional: HashMap::new(),
        }
    }

    /// Convenience function for setting boolean flags in the request. (Chaining call)
    pub fn and_boolean_option(mut self, option: &str, value: bool) -> Self {
        self.optional
            .insert(option.to_string(), serde_json::value::Value::from(value));
        self
    }

    /// Convenience function for setting string flags in the request. (Chaining call)
    pub fn and_string_option(mut self, option: &str, value: &str) -> Self {
        self.optional
            .insert(option.to_string(), serde_json::value::Value::from(value));
        self
    }

    /// Whether request should return genre stations in the response. (Chaining call)
    pub fn return_genre_stations(self, value: bool) -> Self {
        self.and_boolean_option("returnGenreStations", value)
    }

    /// Whether request should return capped in the response. (Chaining call)
    pub fn return_capped(self, value: bool) -> Self {
        self.and_boolean_option("returnCapped", value)
    }

    /// Whether request should include PandoraOne info in the response. (Chaining call)
    pub fn include_pandora_one_info(self, value: bool) -> Self {
        self.and_boolean_option("includePandoraOneInfo", value)
    }

    /// Whether request should include demographics in the response. (Chaining call)
    pub fn include_demographics(self, value: bool) -> Self {
        self.and_boolean_option("includeDemographics", value)
    }

    /// Whether request should include ad attributes in the response. (Chaining call)
    pub fn include_ad_attributes(self, value: bool) -> Self {
        self.and_boolean_option("includeAdAttributes", value)
    }

    /// Whether request should return station list in the response. (Chaining call)
    pub fn return_station_list(self, value: bool) -> Self {
        self.and_boolean_option("returnStationList", value)
    }

    /// Whether request should include the station art url in the response. (Chaining call)
    pub fn include_station_art_url(self, value: bool) -> Self {
        self.and_boolean_option("includeStationArtUrl", value)
    }

    /// Whether request should include the station seeds in the response. (Chaining call)
    pub fn include_station_seeds(self, value: bool) -> Self {
        self.and_boolean_option("includeStationSeeds", value)
    }

    /// Whether request should include shuffle stations instead of quickmix in the response. (Chaining call)
    pub fn include_shuffle_instead_of_quick_mix(self, value: bool) -> Self {
        self.and_boolean_option("includeShuffleInsteadOfQuickMix", value)
    }

    /// The size of station art to include in the response (if includeStationArlUrl was set). (Chaining call)
    pub fn station_art_size(self, value: &str) -> Self {
        self.and_string_option("includeShuffleInsteadOfQuickMix", value)
    }

    /// Whether request should return collect track lifetime stats in the response. (Chaining call)
    pub fn return_collect_track_lifetime_stats(self, value: bool) -> Self {
        self.and_boolean_option("returnCollectTrackLifetimeStats", value)
    }

    /// Whether request should return whether the user is a subscriber in the response. (Chaining call)
    pub fn return_is_subscriber(self, value: bool) -> Self {
        self.and_boolean_option("returnIsSubscriber", value)
    }

    /// Whether the requesting client is cross-platform ad capable. (Chaining call)
    pub fn xplatform_ad_capable(self, value: bool) -> Self {
        self.and_boolean_option("xplatformAdCapable", value)
    }

    /// Whether the complimentary sponsors are supported. (Chaining call)
    pub fn complimentary_sponsor_supported(self, value: bool) -> Self {
        self.and_boolean_option("complimentarySponsorSupported", value)
    }

    /// Whether request should include subscription expiration in the response. (Chaining call)
    pub fn include_subscription_expiration(self, value: bool) -> Self {
        self.and_boolean_option("includeSubscriptionExpiration", value)
    }

    /// Whether request should return whether the user has used their trial
    /// subscription in the response. (Chaining call)
    pub fn return_has_used_trial(self, value: bool) -> Self {
        self.and_boolean_option("returnHasUsedTrial", value)
    }

    /// Whether request should return user state in the response. (Chaining call)
    pub fn return_userstate(self, value: bool) -> Self {
        self.and_boolean_option("returnUserstate", value)
    }

    /// Whether request should return account message in the response. (Chaining call)
    pub fn include_account_message(self, value: bool) -> Self {
        self.and_boolean_option("includeAccountMessage", value)
    }

    /// Whether request should include user webname in the response. (Chaining call)
    pub fn include_user_webname(self, value: bool) -> Self {
        self.and_boolean_option("includeUserWebname", value)
    }

    /// Whether request should include listening hours in the response. (Chaining call)
    pub fn include_listening_hours(self, value: bool) -> Self {
        self.and_boolean_option("includeListeningHours", value)
    }

    /// Whether request should include facebook connections in the response. (Chaining call)
    pub fn include_facebook(self, value: bool) -> Self {
        self.and_boolean_option("includeFacebook", value)
    }

    /// Whether request should include twitter connections in the response. (Chaining call)
    pub fn include_twitter(self, value: bool) -> Self {
        self.and_boolean_option("includeTwitter", value)
    }

    /// Whether request should include daily skip limit in the response. (Chaining call)
    pub fn include_daily_skip_limit(self, value: bool) -> Self {
        self.and_boolean_option("includeDailySkipLimit", value)
    }

    /// Whether request should include the track skip delay in the response. (Chaining call)
    pub fn include_skip_delay(self, value: bool) -> Self {
        self.and_boolean_option("includeSkipDelay", value)
    }

    /// Whether request should include Google Play metadata in the response. (Chaining call)
    pub fn include_googleplay(self, value: bool) -> Self {
        self.and_boolean_option("includeGoogleplay", value)
    }

    /// Whether request should include the user recommendations in the response. (Chaining call)
    pub fn include_show_user_recommendations(self, value: bool) -> Self {
        self.and_boolean_option("includeShowUserRecommendations", value)
    }

    /// Whether request should include advertiser attributes in the response. (Chaining call)
    pub fn include_advertiser_attributes(self, value: bool) -> Self {
        self.and_boolean_option("includeAdvertiserAttributes", value)
    }

    /// This is a wrapper around the `response` method from the
    /// PandoraApiRequest trait that automatically merges the user tokens from
    /// the response back into the session.
    pub async fn merge_response(
        &self,
        session: &mut PandoraSession,
    ) -> Result<UserLoginResponse, Error> {
        let response = self.response(session).await?;
        session.update_user_tokens(&response);
        Ok(response)
    }
}

/// The returned userAuthToken is used to authenticate access to other API methods.
///
/// | Name | Type | Description |
/// | isCapped | boolean |  |
/// | userAuthToken | string |  |
/// ``` json
/// {
///    "stat": "ok",
///    "result": {
///        "stationCreationAdUrl": "http://ad.doubleclick.net/adx/pand.android/prod.createstation;ag=112;gnd=1;zip=23950;genre=0;model=;app=;OS=;dma=560;clean=0;logon=__LOGON__;tile=1;msa=115;st=VA;co=51117;et=0;mc=0;aa=0;hisp=0;hhi=0;u=l*2jedvn446s7ce!ag*112!gnd*1!zip*23950!dma*560!clean*0!logon*__LOGON__!msa*115!st*VA!co*51117!et*0!mc*0!aa*0!hisp*0!hhi*0!genre*0;sz=320x50;ord=__CACHEBUST__",
///        "hasAudioAds": true,
///        "splashScreenAdUrl": "http://ad.doubleclick.net/pfadx/pand.android/prod.welcome;ag=112;gnd=1;zip=23950;model=;app=;OS=;dma=560;clean=0;hours=1;msa=115;st=VA;co=51117;et=0;mc=0;aa=0;hisp=0;hhi=0;u=l*op4jfgdxmddjk!ag*112!gnd*1!zip*23950!dma*560!clean*0!msa*115!st*VA!co*51117!et*0!mc*0!aa*0!hisp*0!hhi*0!hours*1;sz=320x50;ord=__CACHEBUST__",
///        "videoAdUrl": "http://ad.doubleclick.net/pfadx/pand.android/prod.nowplaying;ag=112;gnd=1;zip=23950;dma=560;clean=0;hours=1;app=;index=__INDEX__;msa=115;st=VA;co=51117;et=0;mc=0;aa=0;hisp=0;hhi=0;u=l*2jedvn446s7ce!ag*112!gnd*1!zip*23950!dma*560!clean*0!index*__INDEX__!msa*115!st*VA!co*51117!et*0!mc*0!aa*0!hisp*0!hhi*0!hours*1;sz=442x188;ord=__CACHEBUST__",
///        "username": "user@example.com",
///        "canListen": true,
///        "nowPlayingAdUrl": "http://ad.doubleclick.net/pfadx/pand.android/prod.nowplaying;ag=112;gnd=1;zip=23950;genre=0;station={4};model=;app=;OS=;dma=560;clean=0;hours=1;artist=;interaction=__INTERACTION__;index=__INDEX__;newUser=__AFTERREG__;logon=__LOGON__;msa=115;st=VA;co=51117;et=0;mc=0;aa=0;hisp=0;hhi=0;u=l*op4jfgdxmddjk!ag*112!gnd*1!zip*23950!station*{4}!dma*560!clean*0!index*__INDEX__!newUser*__AFTERREG__!logon*__LOGON__!msa*115!st*VA!co*51117!et*0!mc*0!aa*0!hisp*0!hhi*0!genre*0!interaction*__INTERACTION__!hours*1;sz=320x50;ord=__CACHEBUST__",
///        "userId": "272772589",
///        "listeningTimeoutMinutes": "180",
///        "maxStationsAllowed": 100,
///        "listeningTimeoutAlertMsgUri": "/mobile/still_listening.vm",
///        "userProfileUrl": "https://www.pandora.com/login?auth_token=XXX&target=%2Fpeople%2FXXX",
///        "minimumAdRefreshInterval": 5,
///        "userAuthToken": "XXX"
///    }
/// }
/// ```
/// | Code | Description |
/// | 1002 | Wrong user credentials. |
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLoginResponse {
    /// The user id that should be used for this session
    pub user_id: String,
    /// The user auth token that should be used for this session
    pub user_auth_token: String,
    /// Unknown field.
    pub station_creation_ad_url: String,
    /// Unknown field.
    pub has_audio_ads: bool,
    /// Unknown field.
    pub splash_screen_ad_url: String,
    /// Unknown field.
    pub video_ad_url: String,
    /// Unknown field.
    pub username: String,
    /// Unknown field.
    pub can_listen: bool,
    /// Unknown field.
    pub listening_timeout_minutes: String,
    /// Unknown field.
    pub max_stations_allowed: u32,
    /// Unknown field.
    pub listening_timeout_alert_msg_uri: String,
    /// Unknown field.
    pub user_profile_url: String,
    /// Unknown field.
    pub minimum_ad_refresh_interval: u32,
    /// Additional optional fields that may appear in the response.
    #[serde(flatten)]
    pub optional: HashMap<String, serde_json::value::Value>,
}

impl ToUserTokens for UserLoginResponse {
    fn to_user_id(&self) -> Option<String> {
        Some(self.user_id.clone())
    }

    fn to_user_token(&self) -> Option<String> {
        Some(self.user_auth_token.clone())
    }
}

/// Convenience function to perform a basic user login.
pub async fn user_login(
    session: &mut PandoraSession,
    username: &str,
    password: &str,
) -> Result<UserLoginResponse, Error> {
    UserLogin::new(username, password)
        .return_genre_stations(false)
        .return_capped(false)
        .include_pandora_one_info(false)
        .include_demographics(false)
        .include_ad_attributes(false)
        .return_station_list(false)
        .include_station_art_url(false)
        .include_station_seeds(false)
        .include_shuffle_instead_of_quick_mix(false)
        .return_collect_track_lifetime_stats(false)
        .return_is_subscriber(false)
        .xplatform_ad_capable(false)
        .complimentary_sponsor_supported(false)
        .include_subscription_expiration(false)
        .return_has_used_trial(false)
        .return_userstate(false)
        .include_account_message(false)
        .include_user_webname(false)
        .include_listening_hours(false)
        .include_facebook(false)
        .include_twitter(false)
        .include_daily_skip_limit(false)
        .include_skip_delay(false)
        .include_googleplay(false)
        .include_show_user_recommendations(false)
        .include_advertiser_attributes(false)
        .merge_response(session)
        .await
}

#[cfg(test)]
mod tests {
    use crate::json::{tests::session_login, Partner};

    // Tests both PartnerLogin and UserLogin
    #[tokio::test]
    async fn auth_test() {
        let partner = Partner::default();
        let session = session_login(&partner)
            .await
            .expect("Failed initializing login session");
        println!("Session tokens: {:?}", session);
    }
}
