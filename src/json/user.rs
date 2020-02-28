/*!
User support methods.

User Settings
The following settings are currently read/writeable:

| Name |    Type |    Description |
| gender |  string |  Male or Female |
| birthYear |   int   | |
| zipCode | string    | |
| isProfilePrivate |    boolean   | |
| enableComments |  boolean   | |
| emailOptIn |  boolean   | |
| emailComments |   boolean   | |
| emailNewFollowers |   boolean   | |
| isExplicitContentFilterEnabled |  boolean   | |
| isExplicitContentFilterPINProtected | boolean   | |
| newUsername | string    | |
| newPassword | string    | |
| facebookAutoShareEnabled |    boolean   | |
| autoShareTrackPlay |  boolean   | |
| autoShareLikes |  boolean   | |
| autoShareFollows |    boolean   | |
| facebookSettingChecksum | boolean   | |
*/
// SPDX-License-Identifier: MIT AND WTFPL
use std::collections::HashMap;
use std::convert::TryFrom;

use pandora_api_derive::PandoraRequest;
use serde::{Deserialize, Serialize};

use crate::errors::Error;
use crate::json::{PandoraApiRequest, PandoraSession, Timestamp};

/// Valid values for the gender is user account settings. The documentation
/// suggests that the only valid values are "Male", "Female".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserGender {
    /// User gender is male.
    Male,
    /// User gender is female.
    Female,
}

impl ToString for UserGender {
    fn to_string(&self) -> String {
        match self {
            UserGender::Male => "Male".to_string(),
            UserGender::Female => "Female".to_string(),
        }
    }
}

impl TryFrom<&str> for UserGender {
    type Error = Error;
    fn try_from(fmt: &str) -> std::result::Result<Self, Self::Error> {
        match fmt {
            "Male" => Ok(UserGender::Male),
            "Female" => Ok(UserGender::Female),
            x => Err(Self::Error::InvalidUserGender(x.to_string())),
        }
    }
}

impl TryFrom<String> for UserGender {
    type Error = Error;
    fn try_from(fmt: String) -> std::result::Result<Self, Self::Error> {
        Self::try_from(fmt.as_str())
    }
}

/// **Unsupported!**
/// Undocumented method
/// [user.accountMessageDismissed()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct AccountMessageDismissedUnsupported {}

/// **Unsupported!**
/// Undocumented method
/// [user.acknowledgeSubscriptionExpiration()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct AcknowledgeSubscriptionExpirationUnsupported {}

/// **Unsupported!**
/// Undocumented method
/// [user.associateDevice()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct AssociateDeviceUnsupported {}

/// **Unsupported!**
/// Undocumented method
/// [user.authorizeFacebook()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct AuthorizeFacebookUnsupported {}

/// Returns whether a user is subscribed or if they can subscribe to Pandora One. Can be useful to determine which Partner password to use.
///
/// | Name | Type | Description |
/// | iapVendor | string | (optional) |
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[pandora_request(encrypted = true)]
#[serde(rename_all = "camelCase")]
pub struct CanSubscribe {
    /// Optional parameters on the call
    #[serde(flatten)]
    pub optional: HashMap<String, serde_json::value::Value>,
}

impl CanSubscribe {
    /// Create a new CanSubscribe with some values. All Optional fields are
    /// set to None.
    pub fn new() -> Self {
        Self::default()
    }

    /// Convenience function for setting string flags in the request. (Chaining call)
    pub fn and_string_option(mut self, option: &str, value: &str) -> Self {
        self.optional
            .insert(option.to_string(), serde_json::value::Value::from(value));
        self
    }

    /// Set the name of the in-app purchases vendor. (Chaining call)
    pub fn iap_vendor(self, value: &str) -> Self {
        self.and_string_option("iapVendor", value)
    }
}

impl Default for CanSubscribe {
    fn default() -> Self {
        Self {
            optional: HashMap::new(),
        }
    }
}

/// | Name | Type | Description |
/// | canSubscribe | boolean | false if user is a Pandora One subscriber |
/// | isSubscriber | boolean | true if user is a Pandora One Subscriber |
/// ``` json
/// {
///     "stat": "ok",
///      "result": {
///           "canSubscribe": false,
///           "isSubscriber": true
///      }
///  }
///  ```
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanSubscribeResponse {
    /// Whether it is valid for this account to subscribe to Pandora One.
    pub can_subscribe: bool,
    /// Whether this account has a valid subscription to Pandora One.
    pub is_subscriber: bool,
}

/// Convenience function to do a basic canSubscribe call.
pub fn can_subscribe(session: &PandoraSession) -> Result<CanSubscribeResponse, Error> {
    CanSubscribe::new().response(session)
}

/// | Name   |  Type    Description |
/// | currentUsername | string   | |
/// | currentPassword | string   | |
/// | userInitiatedChange | boolean | optional |
/// | includeFacebook | boolean | optional |
/// Additionally keys listed in Settings are permitted in the request body.
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[pandora_request(encrypted = true)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSettings {
    /// Current credentials must be provided with the request.
    pub current_username: String,
    /// Current credentials must be provided with the request.
    pub current_password: String,
    /// Optional parameters on the call
    #[serde(flatten)]
    pub optional: HashMap<String, serde_json::value::Value>,
}

impl ChangeSettings {
    /// Create a new ChangeSettings with some values. All Optional fields are
    /// set to None.
    pub fn new(current_username: &str, current_password: &str) -> Self {
        Self {
            current_username: current_username.to_string(),
            current_password: current_password.to_string(),
            optional: HashMap::new(),
        }
    }

    /// Convenience function for setting boolean flags in the request. (Chaining call)
    pub fn and_boolean_option(mut self, option: &str, value: bool) -> Self {
        self.optional
            .insert(option.to_string(), serde_json::value::Value::from(value));
        self
    }

    /// Convenience function for setting boolean flags in the request. (Chaining call)
    pub fn and_string_option(mut self, option: &str, value: &str) -> Self {
        self.optional
            .insert(option.to_string(), serde_json::value::Value::from(value));
        self
    }

    /// Convenience function for setting boolean flags in the request. (Chaining call)
    pub fn and_number_option(mut self, option: &str, value: u32) -> Self {
        self.optional
            .insert(option.to_string(), serde_json::value::Value::from(value));
        self
    }

    /// Whether request was initiated at the user request. (Chaining call)
    pub fn user_initiated_change(self, value: bool) -> Self {
        self.and_boolean_option("userInitiatedChange", value)
    }

    /// Unknown. (Chaining call)
    pub fn include_facebook(self, value: bool) -> Self {
        self.and_boolean_option("includeFacebook", value)
    }

    /// Set account-holder gender, Male or Female. (Chaining call)
    pub fn gender(self, value: UserGender) -> Self {
        self.and_string_option("gender", &value.to_string())
    }

    /// Set account-holder birth year. (Chaining call)
    pub fn birth_year(self, value: u32) -> Self {
        self.and_number_option("birthYear", value)
    }

    /// Set account-holder zip code. (Chaining call)
    pub fn zip_code(self, value: &str) -> Self {
        self.and_string_option("zipCode", value)
    }

    /// Whether the user profile is private or publicly visible. (Chaining call)
    pub fn is_profile_private(self, value: bool) -> Self {
        self.and_boolean_option("isProfilePrivate", value)
    }

    /// Whether account comments are enabled. (Chaining call)
    pub fn enable_comments(self, value: bool) -> Self {
        self.and_boolean_option("enableComments", value)
    }

    /// Whether email communications from Pandora are permitted. (Chaining call)
    pub fn email_opt_in(self, value: bool) -> Self {
        self.and_boolean_option("emailOptIn", value)
    }

    /// Whether to receive email notifications for comments. (Chaining call)
    pub fn email_comments(self, value: bool) -> Self {
        self.and_boolean_option("emailComments", value)
    }

    /// Whether to receive email notifications of new followers. (Chaining call)
    pub fn email_new_followers(self, value: bool) -> Self {
        self.and_boolean_option("emailNewFollowers", value)
    }

    /// Whether the explicit content filter should be enabled. (Chaining call)
    pub fn is_explicit_content_filter_enabled(self, value: bool) -> Self {
        self.and_boolean_option("isExplicitContentFilterEnabled", value)
    }

    /// Whether the explicit content filter is protected by a PIN code. (Chaining call)
    pub fn is_explicit_content_filter_pin_protected(self, value: bool) -> Self {
        self.and_boolean_option("isExplicitContentFilterPINProtected", value)
    }

    /// New account username. (Chaining call)
    pub fn new_username(self, value: &str) -> Self {
        self.and_string_option("newUsername", value)
    }

    /// New account password. (Chaining call)
    pub fn new_password(self, value: &str) -> Self {
        self.and_string_option("newPassword", value)
    }

    /// Whether to auto-share on facebook. (Chaining call)
    pub fn facebook_auto_share_enabled(self, value: bool) -> Self {
        self.and_boolean_option("facebookAutoShareEnabled", value)
    }

    /// Whether to auto-share tracks played. (Chaining call)
    pub fn auto_share_track_play(self, value: bool) -> Self {
        self.and_boolean_option("autoShareTrackPlay", value)
    }

    /// Whether to auto-share liked tracks. (Chaining call)
    pub fn auto_share_track_likes(self, value: bool) -> Self {
        self.and_boolean_option("autoShareTrackLikes", value)
    }

    /// Whether to auto-share user follows. (Chaining call)
    pub fn auto_share_follows(self, value: bool) -> Self {
        self.and_boolean_option("autoShareFollows", value)
    }

    /// Unknown. (Chaining call)
    pub fn facebook_setting_checksum(self, value: bool) -> Self {
        self.and_boolean_option("facebookSettingChecksum", value)
    }
}

/// There's no known response to data to this request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSettingsResponse {
    /// The fields of the changeSettings response are unknown.
    #[serde(flatten)]
    pub optional: HashMap<String, serde_json::value::Value>,
}

/// Convenience function to do a basic canSubscribe call. This function
/// is basically useless for actually changing settings, but is useful
/// to return the current values for user account settings.
pub fn change_settings(
    session: &PandoraSession,
    username: &str,
    password: &str,
) -> Result<ChangeSettingsResponse, Error> {
    ChangeSettings::new(username, password).response(session)
}

/// | Name    | Type  |  Description   |
/// | username |    string       | |
/// | password  |   string       | |
/// | gender |  string       | |
/// | birthYear |   int      | |
/// | zipCode | int      | |
/// | emailOptIn |  boolean      | |
/// | countryCode | string       | |
/// | accountType|  string  registered   | |
/// | registeredType |  string  user     | |
/// | includePandoraOneInfo |   boolean      | |
/// | includeAccountMessage |   boolean      | |
/// | returnCollectTrackLifetimeStats | boolean      | |
/// | returnIsSubscriber |  boolean      | |
/// | xplatformAdCapable |  boolean      | |
/// | includeFacebook | boolean      | |
/// | includeGoogleplay |   boolean      | |
/// | includeShowUserRecommendations |  boolean      | |
/// | includeAdvertiserAttributes | boolean      | |
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[pandora_request(encrypted = true)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
    /// Username to associate with the account.
    pub username: String,
    /// Password to set for the account.
    pub password: String,
    /// Account-holder gender, Male or Female.
    pub gender: UserGender,
    /// Account-holder birth year.
    pub birth_year: u32,
    /// Account-holder zip code.
    pub zip_code: String,
    /// Whether the user opts in to e-mail communciations.
    pub email_opt_in: bool,
    /// Account-holder country code.
    pub country_code: String,
    /// Unknown: "registered"?
    pub account_type: String,
    /// Unknown: "user"?
    pub registered_type: String,
    /// Unknown.
    pub include_pandora_one_info: bool,
    /// Unknown.
    pub include_account_message: bool,
    /// Unknown.
    pub return_collect_track_lifetime_stats: bool,
    /// Unknown.
    pub return_is_subscriber: bool,
    /// Unknown.
    pub xplatform_ad_capable: bool,
    /// Unknown.
    pub include_facebook: bool,
    /// Unknown.
    pub include_googleplay: bool,
    /// Unknown.
    pub include_show_user_recommendations: bool,
    /// Unknown.
    pub include_advertiser_attributes: bool,
}

impl CreateUser {
    /// Create a new CreateUser with some values. All Optional fields are
    /// set to None.
    pub fn new(
        username: &str,
        password: &str,
        gender: UserGender,
        birth_year: u32,
        zip_code: &str,
        country_code: &str,
    ) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            gender,
            birth_year,
            zip_code: zip_code.to_string(),
            country_code: country_code.to_string(),
            email_opt_in: false,
            account_type: "registered".to_string(),
            registered_type: "user".to_string(),
            include_pandora_one_info: false,
            include_account_message: false,
            return_collect_track_lifetime_stats: false,
            return_is_subscriber: false,
            xplatform_ad_capable: false,
            include_facebook: false,
            include_googleplay: false,
            include_show_user_recommendations: false,
            include_advertiser_attributes: false,
        }
    }
}

/// There's no known response to data to this request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserResponse {
    /// The fields of the createUser response are unknown.
    #[serde(flatten)]
    pub optional: HashMap<String, serde_json::value::Value>,
}

/// Convenience function to do a basic emailPassword call.
pub fn create_user(
    session: &PandoraSession,
    username: &str,
    password: &str,
    gender: UserGender,
    birth_year: u32,
    zip_code: &str,
    country_code: &str,
) -> Result<CreateUserResponse, Error> {
    CreateUser::new(
        username,
        password,
        gender,
        birth_year,
        zip_code,
        country_code,
    )
    .response(session)
}

/// **Unsupported!**
/// Undocumented method
/// [user.disconnectFacebook()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct DisconnectFacebookUnsupported {}

/// | Name  |   Type  |   Description |
/// | username  |   string  | |
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[serde(rename_all = "camelCase")]
pub struct EmailPassword {
    /// The e-mail password recovery information to the e-mail associated with
    /// the this username.
    pub username: String,
}

impl<TS: ToString> From<&TS> for EmailPassword {
    fn from(username: &TS) -> Self {
        Self {
            username: username.to_string(),
        }
    }
}

/// There's no known response to data to this request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailPasswordResponse {
    /// The fields of the emailPassword response are unknown.
    #[serde(flatten)]
    pub optional: HashMap<String, serde_json::value::Value>,
}

/// Convenience function to do a basic emailPassword call.
pub fn email_password(
    session: &PandoraSession,
    username: &str,
) -> Result<EmailPasswordResponse, Error> {
    EmailPassword::from(&username).response(session)
}

/// **Unsupported!**
/// Undocumented method
/// [user.facebookAuthFailed()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct FacebookAuthFailedUnsupported {}

/// The request has no parameters.
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[pandora_request(encrypted = true)]
#[serde(rename_all = "camelCase")]
pub struct GetBookmarks {}

impl GetBookmarks {
    /// Create a new GetBookmarks.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for GetBookmarks {
    fn default() -> Self {
        Self {}
    }
}

/// ``` json
/// {
///     "stat":"ok",
///     "result": {
///         "artists": [
///             {
///                 "musicToken": "R130360",
///                 "artistName": "Cannabich, Christian",
///                 "artUrl": "http://cont-sv5-2.pandora.com/images/public/amz/5/2/9/7/095115137925_500W_488H.jpg",
///                 "bookmarkToken": "80982345262345234",
///                 "dateCreated": {
///                     "nanos": 300000000,
///                     "seconds": 22,
///                     "year": 112,
///                     "month": 4,
///                     "hours": 11,
///                     "time": 1350566223422,
///                     "date": 23,
///                     "minutes": 01,
///                     "day": 2,
///                     "timezoneOffset": 720
///                 }
///             }
///         ],
///         "songs": [
///             {
///                 "sampleUrl": "http://www.pandora.com/favorites/getSample.jsp?token=32458973245b90287345d0234fc34f8b&allowExplicit=true",
///                 "sampleGain": "-7.87",
///                 "albumName": "Symphony In G Major",
///                 "artistName": "Cannabich, Christian",
///                 "musicToken": "S2894329",
///                 "dateCreated": {
///                     "nanos": 300000000,
///                     "seconds": 22,
///                     "year": 112,
///                     "month": 4,
///                     "hours": 11,
///                     "time": 1350566223422,
///                     "date": 23,
///                     "minutes": 01,
///                     "day": 2,
///                     "timezoneOffset": 720
///                 },
///                 "artUrl": "http://cont-sv5-2.pandora.com/images/public/amz/5/2/9/7/095115137925_500W_488H.jpg",
///                 "bookmarkToken": "290832123432459854",
///                 "songName": "London Mozart Players, Christian Cannabich: Symphonies"
///             }
///         ]
///     }
///
///  }
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBookmarksResponse {
    /// A list of bookmarked artists.
    pub artists: Vec<ArtistBookmark>,
    /// A list of bookmarked songs.
    pub songs: Vec<SongBookmark>,
}

///         "artists": [
///             {
///                 "musicToken": "R130360",
///                 "artistName": "Cannabich, Christian",
///                 "artUrl": "http://cont-sv5-2.pandora.com/images/public/amz/5/2/9/7/095115137925_500W_488H.jpg",
///                 "bookmarkToken": "80982345262345234",
///                 "dateCreated": {
///                     "nanos": 300000000,
///                     "seconds": 22,
///                     "year": 112,
///                     "month": 4,
///                     "hours": 11,
///                     "time": 1350566223422,
///                     "date": 23,
///                     "minutes": 01,
///                     "day": 2,
///                     "timezoneOffset": 720
///                 }
///             }
///         ]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistBookmark {
    /// Unique identifier (token) associated with this bookmark.
    pub bookmark_token: String,
    /// Unique identifier (token) for the music item that was bookmarked.
    pub music_token: String,
    /// The name of the artist bookmarked.
    pub artist_name: String,
    /// Art url for the bookmark.
    pub art_url: String,
    /// Timestamp for when the bookmark was created.
    pub date_created: Timestamp,
}

///         "songs": [
///             {
///                 "sampleUrl": "http://www.pandora.com/favorites/getSample.jsp?token=32458973245b90287345d0234fc34f8b&allowExplicit=true",
///                 "sampleGain": "-7.87",
///                 "albumName": "Symphony In G Major",
///                 "artistName": "Cannabich, Christian",
///                 "musicToken": "S2894329",
///                 "dateCreated": {
///                     "nanos": 300000000,
///                     "seconds": 22,
///                     "year": 112,
///                     "month": 4,
///                     "hours": 11,
///                     "time": 1350566223422,
///                     "date": 23,
///                     "minutes": 01,
///                     "day": 2,
///                     "timezoneOffset": 720
///                 },
///                 "artUrl": "http://cont-sv5-2.pandora.com/images/public/amz/5/2/9/7/095115137925_500W_488H.jpg",
///                 "bookmarkToken": "290832123432459854",
///                 "songName": "London Mozart Players, Christian Cannabich: Symphonies"
///             }
///         ]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SongBookmark {
    /// Unique identifier (token) associated with this bookmark.
    pub bookmark_token: String,
    /// Unique identifier (token) for the music item that was bookmarked.
    pub music_token: String,
    /// The name of the song bookmarked.
    pub song_name: String,
    /// The name of the artist for the bookmarked song.
    pub artist_name: String,
    /// The name of the album for the bookmarked song.
    pub album_name: String,
    /// Art url for the bookmark.
    pub art_url: String,
    /// Url for a sample of the bookmarked song.
    pub sample_url: String,
    /// Playback gain for the song sample.
    pub sample_gain: String,
    /// Timestamp for when the bookmark was created.
    pub date_created: Timestamp,
}

/// Convenience function to do a basic getBookmarks call.
pub fn get_bookmarks(session: &PandoraSession) -> Result<GetBookmarksResponse, Error> {
    GetBookmarks::new().response(session)
}

/// **Unsupported!**
/// Undocumented method
/// [user.getFacebookInfo()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct GetFacebookInfoUnsupported {}

/// | Name   |  Type   |  Description |
/// | includeFacebook | boolean   | |
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[pandora_request(encrypted = true)]
#[serde(rename_all = "camelCase")]
pub struct GetSettings {
    /// Whether to include Facebook settings in the response.
    pub include_facebook: bool,
}

impl GetSettings {
    /// Create a new GetSettings, omitting facebook from the response.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new GetSettings, and include facebook in the response.
    pub fn new_with_facebook() -> Self {
        Self {
            include_facebook: true,
        }
    }
}

impl Default for GetSettings {
    fn default() -> Self {
        Self {
            include_facebook: false,
        }
    }
}

/// See Settings for return values.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSettingsResponse {
    /// The fields of the setQuickMix response are unknown.
    #[serde(flatten)]
    pub optional: HashMap<String, serde_json::value::Value>,
}

/// Convenience function to do a basic getSettings call.
pub fn get_settings(session: &PandoraSession) -> Result<GetSettingsResponse, Error> {
    GetSettings::new().response(session)
}

/// To check if the station list was modified by another client the checksum
/// can be fetched. No parameters are required for this request.
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[serde(rename_all = "camelCase")]
pub struct GetStationListChecksum {}

impl GetStationListChecksum {
    /// Create a new GetStationListChecksum.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for GetStationListChecksum {
    fn default() -> Self {
        Self {}
    }
}

/// The response contains the new checksum.
///
/// | Name    | Type    | Description |
/// | checksum    | string   | |
/// ``` json
/// {
///     "stat":"ok",
///     "result":{
///         "checksum":"99776ddd31ad798895578593e78e3691"
///     }
/// }
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStationListChecksumResponse {
    /// Checksum for station list
    pub checksum: String,
}

/// | Name | Type | Description |
/// | includeStationArtUrl | boolean | Includes “artUrl” field in result (optional) |
/// | stationArtSize | string | “W130H130” (optional) |
/// | includeAdAttributes | boolean | (optional) |
/// | includeStationSeeds | boolean | (optional) |
/// | includeShuffleInsteadOfQuickMix | boolean | (optional) |
/// | includeRecommendations | boolean | (optional) |
/// | includeExplanations | boolean | (optional) |
/// ``` json
/// {
///    "userAuthToken": "XXX",
///    "syncTime": XXX
/// }
/// ```
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[pandora_request(encrypted = true)]
#[serde(rename_all = "camelCase")]
pub struct GetStationList {
    /// Optional parameters on the call
    #[serde(flatten)]
    pub optional: HashMap<String, serde_json::value::Value>,
}

impl GetStationList {
    /// Create a new GetStationList with some values. All Optional fields are
    /// set to None.
    pub fn new() -> Self {
        Self::default()
    }

    /// Convenience function for setting boolean flags in the request. (Chaining call)
    pub fn and_boolean_option(mut self, option: &str, value: bool) -> Self {
        self.optional
            .insert(option.to_string(), serde_json::value::Value::from(value));
        self
    }

    /// Convenience function for setting boolean flags in the request. (Chaining call)
    pub fn and_string_option(mut self, option: &str, value: &str) -> Self {
        self.optional
            .insert(option.to_string(), serde_json::value::Value::from(value));
        self
    }

    /// Whether to include station art url in the response. (Chaining call)
    pub fn include_station_art_url(self, value: bool) -> Self {
        self.and_boolean_option("includeStationArtUrl", value)
    }

    /// The size of the station art image to include in the response. (Chaining call)
    pub fn station_art_size(self, value: &str) -> Self {
        self.and_string_option("stationArtSize", value)
    }

    /// Whether to include ad attributes in the response. (Chaining call)
    pub fn include_ad_attributes(self, value: bool) -> Self {
        self.and_boolean_option("includeAdAttributes", value)
    }

    /// Whether to include station seeds in the response. (Chaining call)
    pub fn include_station_seeds(self, value: bool) -> Self {
        self.and_boolean_option("includeStationSeeds", value)
    }

    /// Whether to include shuffle stations instead of quickmix in the response. (Chaining call)
    pub fn include_shuffle_instead_of_quick_mix(self, value: bool) -> Self {
        self.and_boolean_option("includeShuffleInsteadOfQuickMix", value)
    }

    /// Whether to include recommendations in the response. (Chaining call)
    pub fn include_recommendations(self, value: bool) -> Self {
        self.and_boolean_option("includeRecommendations", value)
    }

    /// Whether to include explanations in the response. (Chaining call)
    pub fn include_explanations(self, value: bool) -> Self {
        self.and_boolean_option("includeExplanations", value)
    }
}

impl Default for GetStationList {
    fn default() -> Self {
        Self {
            optional: HashMap::new(),
        }
    }
}

/// Currently stationId and stationToken are the same.
///
/// QuickMix stations additionally include a list of station ids
/// (quickMixStationIds) that are currently selected for the mix.
///
/// | Name | Type | Description |
/// | stations.stationId | string |   |
/// | stations.stationName | string |   |
/// | stations.dateCreated.time | int | Number of milliseconds since Unix epoch |
/// | checksum | string |   |
/// ``` json
/// {
///         "stat":"ok",
///         "result":{
///            "stations":[
///               {
///                  "suppressVideoAds":true,
///                  "isQuickMix":true,
///                  "stationId":"3914377363925265",
///                  "stationDetailUrl":"https://www.pandora.com/login?target=%2Fstations%2Fa61985110ea3d6c6c8d8a9c038588b26425ba2910f7abf8b",
///                  "isShared":false,
///                  "dateCreated":{
///                     "date":8,
///                     "day":4,
///                     "hours":22,
///                     "minutes":44,
///                     "month":10,
///                     "nanos":241000000,
///                     "seconds":46,
///                     "time":1194590686241,
///                     "timezoneOffset":480,
///                     "year":107
///                  },
///                  "stationToken":"3914377363925265",
///                  "stationName":"QuickMix",
///                  "stationSharingUrl":"https://www.pandora.com/login?target=%2Fshare%2Fstation%2Fa61985110ea3d6c6c8d8a9c038588b26425ba2910f7abf8b",
///                  "requiresCleanAds":true,
///                  "allowRename":false,
///                  "allowAddMusic":false,
///                  "quickMixStationIds":[
///                     "339646069607180561",
///                     "339644480469281041"
///                  ],
///                  "allowDelete":false,
///                  "allowEditDescription":false
///               }
///            ],
///            "checksum":"99776ddd31ad798895578593e78e3691"
///         }
///      }
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStationListResponse {
    /// List of user-defined stations.
    pub stations: Vec<Station>,
    /// Checksum over the list of stations.  This can be used later to determine
    /// whether the station list has changed and cached station list information
    /// can be re-used.
    pub checksum: String,
}

///            "stations":[
///               {
///                  "suppressVideoAds":true,
///                  "isQuickMix":true,
///                  "stationId":"3914377363925265",
///                  "stationDetailUrl":"https://www.pandora.com/login?target=%2Fstations%2Fa61985110ea3d6c6c8d8a9c038588b26425ba2910f7abf8b",
///                  "isShared":false,
///                  "dateCreated":{
///                     "date":8,
///                     "day":4,
///                     "hours":22,
///                     "minutes":44,
///                     "month":10,
///                     "nanos":241000000,
///                     "seconds":46,
///                     "time":1194590686241,
///                     "timezoneOffset":480,
///                     "year":107
///                  },
///                  "stationToken":"3914377363925265",
///                  "stationName":"QuickMix",
///                  "stationSharingUrl":"https://www.pandora.com/login?target=%2Fshare%2Fstation%2Fa61985110ea3d6c6c8d8a9c038588b26425ba2910f7abf8b",
///                  "requiresCleanAds":true,
///                  "allowRename":false,
///                  "allowAddMusic":false,
///                  "quickMixStationIds":[
///                     "339646069607180561",
///                     "339644480469281041"
///                  ],
///                  "allowDelete":false,
///                  "allowEditDescription":false
///               }
///            ],
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    /// Unique identifier (token) for this station. Currently stationId and
    /// stationToken are the same.
    pub station_id: String,
    /// Unique identifier (token) for this station. Currently stationId and
    /// stationToken are the same.
    pub station_token: String,
    /// User-defined name for this station.
    pub station_name: String,
    /// Url for additional information about station.
    pub station_detail_url: String,
    /// Ids for stations included in this quickmix.
    #[serde(default)]
    pub quick_mix_station_ids: Vec<String>,
    /// Is this station a quickmix.
    pub is_quick_mix: bool,
    /// Unknown.
    pub suppress_video_ads: bool,
    /// Wether this station is shared.
    pub is_shared: bool,
    /// Unknown.
    pub requires_clean_ads: bool,
    /// Whether station may be renamed.
    pub allow_rename: bool,
    /// Whether station allows adding music.
    pub allow_add_music: bool,
    /// Whether station can be deleted.
    pub allow_delete: bool,
    /// Whether station description can be edited.
    pub allow_edit_description: bool,
    /// Timestamp for when the station was created.
    pub date_created: Timestamp,
    /// Additional, optional fields of the response.
    #[serde(flatten)]
    pub optional: HashMap<String, serde_json::value::Value>,
}

/// Convenience function to do a basic getStationList call.
pub fn get_station_list(session: &PandoraSession) -> Result<GetStationListResponse, Error> {
    GetStationList::new().response(session)
}

/// The request has no parameters.
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[pandora_request(encrypted = true)]
#[serde(rename_all = "camelCase")]
pub struct GetUsageInfo {}

impl GetUsageInfo {
    /// Create a new GetUsageInfo.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for GetUsageInfo {
    fn default() -> Self {
        Self {}
    }
}

///
/// | Name | Type | Description |
/// | accountMonthlyListening | int |   |
/// | deviceMonthlyListening | int |   |
/// | monthlyCapHours | int |   |
/// | monthlyCapWarningPercent | int |   |
/// | monthlyCapWarningRepeatPercent | int |   |
/// | isMonthlyPayer | bool |   |
/// | isCapped | bool |   |
/// | listeningTimestamp | int |   |
/// ``` json
/// {
///     "stat": "ok",
///     "result": {
///         "monthlyCapWarningRepeatPercent": 10,
///         "monthlyCapHours": 320,
///         "deviceMonthlyListening": 0,
///         "isMonthlyPayer": false,
///         "isCapped": false,
///         "monthlyCapWarningPercent": 85,
///         "accountMonthlyListening": 0
///     }
/// }
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUsageInfoResponse {
    /// Number of hours spent listening this month on this account.
    pub account_monthly_listening: u32,
    /// Number of hours spent listening from this device this month.
    pub device_monthly_listening: u32,
    /// Maximum number of allowed hours per month.
    pub monthly_cap_hours: u32,
    /// At what percentage of monthly allowed hours the user should be warned.
    pub monthly_cap_warning_percent: u32,
    /// At what percentage of monthly allowed hours the user should get a second
    /// warning.
    pub monthly_cap_warning_repeat_percent: u32,
    /// Whether the account is billed monthly.
    pub is_monthly_payer: bool,
    /// Whether the account has a usage cap.
    pub is_capped: bool,
    /// Unknown.
    pub listening_timestamp: Option<u32>,
}

/// Convenience function to get account usage info.
pub fn get_usage_info(session: &PandoraSession) -> Result<GetUsageInfoResponse, Error> {
    GetUsageInfo {}.response(session)
}

/// **Unsupported!**
/// Undocumented method
/// [user.purchaseAmazonPayToPlay()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct PurchaseAmazonPayToPlayUnsupported {}

/// **Unsupported!**
/// Undocumented method
/// [user.purchaseAmazonSubscription()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct PurchaseAmazonSubscriptionUnsupported {}

/// **Unsupported!**
/// Undocumented method
/// [user.purchaseGooglePayToPlay()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct PurchaseGooglePayToPlayUnsupported {}

/// **Unsupported!**
/// Undocumented method
/// [user.purchaseGoogleSubscription()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct PurchaseGoogleSubscriptionUnsupported {}

/// **Unsupported!**
/// Undocumented method
/// [user.purchaseItunesSubscription()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct PurchaseItunesSubscriptionUnsupported {}

/// **Unsupported!**
/// Undocumented method
/// [user.setAwareOfProfile()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct SetAwareOfProfileUnsupported {}

/// **Unsupported!**
/// Undocumented method
/// [user.setExplicitContentFilter()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct SetExplicitContentFilterUnsupported {}

/// | Name    | Type    | Description |
/// | quickMixStationIds  | array   | List of station id’s (strings) (see Retrieve station list) |
/// ``` json
/// {
///     "quickMixStationIds": ["404958383414849005", "403387202773593581"],
///     "userAuthToken": "XXX",
///     "syncTime": 1338211186
/// }
/// ```
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[serde(rename_all = "camelCase")]
pub struct SetQuickMix {
    /// The identifiers for stations that should be included in the quickmix.
    pub quick_mix_station_ids: Vec<String>,
}

impl SetQuickMix {
    /// Create a new SetQuickMix.  Call add_station() to add a station to the
    /// mix.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a station to this quickmix.
    pub fn add_station(&mut self, station_id: &str) {
        self.quick_mix_station_ids.push(station_id.to_string());
    }
}

impl Default for SetQuickMix {
    fn default() -> Self {
        Self {
            quick_mix_station_ids: Vec::new(),
        }
    }
}

/// The response contains no data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetQuickMixResponse {
    /// The fields of the setQuickMix response are unknown.
    #[serde(flatten)]
    pub optional: HashMap<String, serde_json::value::Value>,
}

/// A song can be banned from all stations temporarily (one month).
///
/// | Name    | Type    | Description |
/// | trackToken  | string  | See Retrieve playlist |
/// ``` json
/// {
///     "trackToken":
///     "d6aa37c60833f12150c4e2ba172c46f24590ebc49df948b6fb7117314c41c8e7d4faee3568884468d9509db2ab998dafdbc4093baf8c38ef",
///     "userAuthToken": "XXX",
///     "syncTime": 1336386838
/// }
/// ```
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[serde(rename_all = "camelCase")]
pub struct SleepSong {
    /// Temporarily ban the specified track from all stations for one month.
    pub track_token: String,
}

impl<TS: ToString> From<&TS> for SleepSong {
    fn from(track_token: &TS) -> Self {
        Self {
            track_token: track_token.to_string(),
        }
    }
}

/// The response contains no data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SleepSongResponse {
    /// The fields of the sleepSong response are unknown.
    #[serde(flatten)]
    pub optional: HashMap<String, serde_json::value::Value>,
}

/// Starts a complimentary pandora one trial. It is unknown what constitutes a
/// valid sponsor at this time, and as such this method will always fail.
///
/// | Name   | Type   | Description |
/// | complimentarySponsor   | string | The ID of the sponsor providing the complimentary trial. |
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[serde(rename_all = "camelCase")]
pub struct StartComplimentaryTrial {
    /// The ID of the sponsor providing the complimentary trial.  There are no
    /// known valid values for this field.
    pub complimentary_sponsor: String,
}

impl<TS: ToString> From<&TS> for StartComplimentaryTrial {
    fn from(complimentary_sponsor: &TS) -> Self {
        Self {
            complimentary_sponsor: complimentary_sponsor.to_string(),
        }
    }
}

/// The response contains no data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartComplimentaryTrialResponse {
    /// The fields of the startComplimentaryTrial response are unknown.
    #[serde(flatten)]
    pub optional: HashMap<String, serde_json::value::Value>,
}

/// This method can be used before authenticating a user with User login, but
/// requires a valid Partner login.
///
/// | Name  |   Type |    Description |
/// | username |   string   | |
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[pandora_request(encrypted = true)]
#[serde(rename_all = "camelCase")]
pub struct ValidateUsername {
    /// The username to validate.
    pub username: String,
}

impl<TS: ToString> From<&TS> for ValidateUsername {
    fn from(username: &TS) -> Self {
        Self {
            username: username.to_string(),
        }
    }
}

/// | Name  |  Type  |  Description |
/// | isValid | boolean  | |
/// | isUnique |    boolean  | |
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateUsernameResponse {
    /// Whether the username is valid (registered).
    pub is_valid: bool,
    /// Whether the username is unique (already used).
    pub is_unique: Option<bool>,
}

/// Convenience function to verify that a username is either valid or unique.
pub fn validate_username(
    session: &PandoraSession,
    username: &str,
) -> Result<ValidateUsernameResponse, Error> {
    ValidateUsername {
        username: username.to_string(),
    }
    .response(session)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors;
    use crate::json::{errors::JsonErrorKind, tests::session_login, Partner};

    #[test]
    fn user_test() {
        let partner = Partner::default();
        let session = session_login(&partner).expect("Failed initializing login session");

        let _can_subscribe =
            can_subscribe(&session).expect("Failed submitting subscription information request");

        let _get_settings =
            get_settings(&session).expect("Failed submitting settings info request");

        let test_username_raw = include_str!("../../test_username.txt");
        let test_username = test_username_raw.trim();
        let test_password_raw = include_str!("../../test_password.txt");
        let test_password = test_password_raw.trim();

        let _change_settings = change_settings(&session, &test_username, &test_password)
            .expect("Failed submitting settings change request");
    }

    /* This test might trigger e-mail-based account recovery, which we probably
     * don't want to do automatically as a test.
    #[test]
    fn email_password_test() {
        let partner = Partner::default();
        let session = session_login(&partner).expect("Failed initializing login session");

        let email_password = email_password(&session).expect("Failed submitting settings change request");
    }
    */

    #[test]
    #[should_panic(expected = "Invalid country code.")]
    fn create_user_test() {
        let partner = Partner::default();
        let mut session = partner.init_session();
        let partner_login = partner
            .login(&mut session)
            .expect("Failed completing partner login");
        session.update_partner_tokens(&partner_login);

        let test_username_raw = include_str!("../../test_username.txt");
        let test_username = test_username_raw.trim();
        let test_password_raw = include_str!("../../test_password.txt");
        let test_password = test_password_raw.trim();

        let test_gender = UserGender::Male;
        let test_birth = 1970u32;
        let test_zip = "90210";
        // I can't figure out a valid country code to use, so we'll write this
        // test in the negative and check for the correct error for invalid
        // country code.
        let test_cc = "US";

        // Theory is that the above credentials are for an existing account,
        // so this should fail as a duplicate account.
        match create_user(
            &session,
            &test_username,
            &test_password,
            test_gender,
            test_birth,
            test_zip,
            test_cc,
        ) {
            Ok(cu) => println!("User successfully created? {:?}", cu),
            Err(errors::Error::PandoraJsonRequestError(e))
                if e.kind() == JsonErrorKind::InvalidCountryCode =>
            {
                panic!("Invalid country code.")
            }
            Err(e) => panic!("Unexpected request error: {:?}", e),
        }
    }
}
