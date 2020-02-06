/*!
Authentication/authorization support messages.
*/
// SPDX-License-Identifier: MIT AND WTFPL

use std::collections::HashMap;

use pandora_api_derive::PandoraRequest;
use serde::{Deserialize, Serialize};

use crate::errors::Error;
use crate::json::{PandoraApiRequest, PandoraSession, ToSessionTokens};

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
    /// Unknown field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_urls: Option<bool>,
    /// Unknown field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_device_type: Option<bool>,
    /// Unknown field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_update_prompt_versions: Option<bool>,
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
            include_urls: None,
            return_device_type: None,
            return_update_prompt_versions: None,
        }
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
    pub device_properties: HashMap<String, serde_json::value::Value>,
    /// Unknown field
    pub urls: Option<HashMap<String, String>>,
}

/// Convenience function to do a basic partnerLogin call.
pub fn partner_login<T: ToSessionTokens>(
    session: &PandoraSession<T>,
    username: &str,
    password: &str,
    device_model: &str,
) -> Result<PartnerLoginResponse, Error> {
    PartnerLogin::new(username, password, device_model, None).response(session)
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
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_genre_stations: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_capped: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_pandora_one_info: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_demographics: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_ad_attributes: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_station_list: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_station_art_url: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_station_seeds: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_shuffle_instead_of_quick_mix: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub station_art_size: Option<String>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_collect_track_lifetime_stats: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_is_subscriber: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xplatform_ad_capable: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complimentary_sponsor_supported: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subscription_expiration: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_has_used_trial: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_userstate: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_account_message: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_user_webname: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_listening_hours: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_facebook: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_twitter: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_daily_skip_limit: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_skip_delay: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_googleplay: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_show_user_recommendations: Option<bool>,
    /// Unknown field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_advertiser_attributes: Option<bool>,
}

impl UserLogin {
    /// Initialize a basic UserLogin request. All optional fields are set to None.
    pub fn new(username: &str, password: &str) -> Self {
        UserLogin {
            // This field should always have the value `user`.
            login_type: "user".to_string(),
            username: username.to_string(),
            password: password.to_string(),
            return_genre_stations: None,
            return_capped: None,
            include_pandora_one_info: None,
            include_demographics: None,
            include_ad_attributes: None,
            return_station_list: None,
            include_station_art_url: None,
            include_station_seeds: None,
            include_shuffle_instead_of_quick_mix: None,
            station_art_size: None,
            return_collect_track_lifetime_stats: None,
            return_is_subscriber: None,
            xplatform_ad_capable: None,
            complimentary_sponsor_supported: None,
            include_subscription_expiration: None,
            return_has_used_trial: None,
            return_userstate: None,
            include_account_message: None,
            include_user_webname: None,
            include_listening_hours: None,
            include_facebook: None,
            include_twitter: None,
            include_daily_skip_limit: None,
            include_skip_delay: None,
            include_googleplay: None,
            include_show_user_recommendations: None,
            include_advertiser_attributes: None,
        }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub now_playing_ad_url: Option<String>,
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
}

/// Convenience function to perform a basic user login.
pub fn user_login<T: ToSessionTokens>(
    session: &PandoraSession<T>,
    username: &str,
    password: &str,
) -> Result<UserLoginResponse, Error> {
    UserLogin::new(username, password).response(session)
}

#[cfg(test)]
mod tests {
    use crate::json::{tests::session_login, Partner};

    // Tests both PartnerLogin and UserLogin
    #[test]
    fn auth_test() {
        let partner = Partner::default();
        let session = session_login(&partner).expect("Failed initializing login session");
        println!("Session tokens: {:?}", session);
    }
}
