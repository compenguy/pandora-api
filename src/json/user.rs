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

/// **Unsupported!**
/// Undocumented method
/// [user.accountMessageDismissed()](https://6xq.net/pandora-apidoc/json/methods/)
pub fn account_message_dismissed() {
    unimplemented!();
}

/// **Unsupported!**
/// Undocumented method
/// [user.acknowledgeSubscriptionExpiration()](https://6xq.net/pandora-apidoc/json/methods/)
pub fn acknowledge_subscription_expiration() {
    unimplemented!();
}

/// **Unsupported!**
/// Undocumented method
/// [user.associateDevice()](https://6xq.net/pandora-apidoc/json/methods/)
pub fn associate_device() {
    unimplemented!();
}

/// **Unsupported!**
/// Undocumented method
/// [user.authorizeFacebook()](https://6xq.net/pandora-apidoc/json/methods/)
pub fn authorize_facebook() {
    unimplemented!();
}

/// Returns whether a user is subscribed or if they can subscribe to Pandora One. Can be useful to determine which Partner password to use.
///
/// | Name | Type | Description |
/// | iapVendor | string | (optional) |
///
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
pub fn can_subscribe() {}

/// | Name   |  Type    Description |
/// | currentUsername | string   | |
/// | currentPassword | string   | |
/// | userInitiatedChange | boolean | optional |
/// | includeFacebook | boolean | optional |
/// Additionally keys listed in Settings are permitted in the request body.
pub fn change_settings() {}

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
pub fn create_user() {}

/// **Unsupported!**
/// Undocumented method
/// [user.disconnectFacebook()](https://6xq.net/pandora-apidoc/json/methods/)
pub fn disconnect_facebook() {
    unimplemented!();
}

/// | Name  |   Type  |   Description |
/// | username  |   string  | |
pub fn email_password() {}

/// **Unsupported!**
/// Undocumented method
/// [user.facebookAuthFailed()](https://6xq.net/pandora-apidoc/json/methods/)
pub fn facebook_auth_failed() {
    unimplemented!();
}

/// The request has no parameters.
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
pub fn get_bookmarks() {}

/// **Unsupported!**
/// Undocumented method
/// [user.getFacebookInfo()](https://6xq.net/pandora-apidoc/json/methods/)
pub fn get_facebook_info() {
    unimplemented!();
}

/// | Name   |  Type   |  Description |
/// | includeFacebook | boolean   | |
/// See Settings for return values.
pub fn get_settings() {}

/// To check if the station list was modified by another client the checksum
/// can be fetched. No parameters are required for this request.
///
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
pub fn get_station_list_checksum() {}

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
pub fn get_station_list() {}

/// The request has no parameters.
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
pub fn get_usage_info() {}

/// **Unsupported!**
/// Undocumented method
/// [user.purchaseAmazonPayToPlay()](https://6xq.net/pandora-apidoc/json/methods/)
pub fn purchase_amazon_pay_to_play() {
    unimplemented!();
}

/// **Unsupported!**
/// Undocumented method
/// [user.purchaseAmazonSubscription()](https://6xq.net/pandora-apidoc/json/methods/)
pub fn purchase_amazon_subscription() {
    unimplemented!();
}

/// **Unsupported!**
/// Undocumented method
/// [user.purchaseGooglePayToPlay()](https://6xq.net/pandora-apidoc/json/methods/)
pub fn purchase_google_pay_to_play() {
    unimplemented!();
}

/// **Unsupported!**
/// Undocumented method
/// [user.purchaseGoogleSubscription()](https://6xq.net/pandora-apidoc/json/methods/)
pub fn purchase_google_subscription() {
    unimplemented!();
}

/// **Unsupported!**
/// Undocumented method
/// [user.purchaseItunesSubscription()](https://6xq.net/pandora-apidoc/json/methods/)
pub fn purchase_itunes_subscription() {
    unimplemented!();
}

/// **Unsupported!**
/// Undocumented method
/// [user.setAwareOfProfile()](https://6xq.net/pandora-apidoc/json/methods/)
pub fn set_aware_of_profile() {}

/// **Unsupported!**
/// Undocumented method
/// [user.setExplicitContentFilter()](https://6xq.net/pandora-apidoc/json/methods/)
pub fn set_explicit_content_filter() {}

/// | Name    | Type    | Description |
/// | quickMixStationIds  | array   | List of station id’s (strings) (see Retrieve station list) |
/// ``` json
/// {
///     "quickMixStationIds": ["404958383414849005", "403387202773593581"],
///     "userAuthToken": "XXX",
///     "syncTime": 1338211186
/// }
/// ```
/// The response contains no data.
pub fn set_quick_mix() {}

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
/// Nothing is returned in the response.
pub fn sleep_song() {}

/// Starts a complimentary pandora one trial. It is unknown what constitutes a
/// valid sponsor at this time, and as such this method will always fail.
///
/// | Name   | Type   | Description |
/// | complimentarySponsor   | string | The ID of the sponsor providing the complimentary trial. |
pub fn start_complimentary_trial() {}

/// This method can be used before authenticating a user with User login, but
/// requires a valid Partner login.
///
/// | Name  |   Type |    Description |
/// | username |   string   | |
///
/// | Name  |  Type  |  Description |
/// | isValid | boolean  | |
/// | isUnique |    boolean  | |
pub fn validate_username() {}
