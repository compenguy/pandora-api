/*!
Station support methods.

A station is a collection of one or more user-supplied seeds. Artists or tracks
can be used as seed. Based on the seeds Pandora decides which music to play.
*/
// SPDX-License-Identifier: MIT AND WTFPL
use std::convert::TryFrom;

use pandora_api_derive::PandoraRequest;
use serde::{Deserialize, Serialize};

use crate::errors::Error;
use crate::json::{PandoraApiRequest, PandoraSession, Timestamp, ToSessionTokens};

/// Songs can be “loved” or “banned”. Both influence the music played on the
/// station. Banned songs are never played again on this particular station.
///
/// | Name         | Type    | Description        |
/// | ------------ | ------- | ------------------ |
/// | stationToken | string  |                    |
/// | trackToken   | string  |                    |
/// | isPositive   | boolean | `false` bans track |
///
/// ``` json
/// {
///     "stationToken": "374145764047334893",
///     "trackToken": "fcc2298ec4b1c93e73ad4b2813ceca0dba565bbbe03d8a78bad65ee89a7aaf4d0b3b11954fe6ab08794283f8ef1d44bfc32ce9f8e0513bec",
///     "isPositive": false,
///     "userAuthToken": "XXX",
///     "syncTime": 1404911036
/// }
/// ```
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[serde(rename_all = "camelCase")]
pub struct AddFeedback {
    /// The unique id (token) for the station on which the track should be rated.
    /// Also sometimes referred to as a stationId.
    pub station_token: String,
    /// The unique id (token) for the track to be rated.
    pub track_token: String,
    /// Whether feedback is positive (true) or negative (false).
    pub is_positive: bool,
}

impl AddFeedback {
    /// Create a new AddFeedback with some values.
    pub fn new(station_token: &str, track_token: &str, is_positive: bool) -> Self {
        Self {
            station_token: station_token.to_string(),
            track_token: track_token.to_string(),
            is_positive,
        }
    }

    /// Create a new AddFeedback with some values, and positive feedback.
    pub fn new_positive(station_token: &str, track_token: &str) -> Self {
        Self::new(station_token, track_token, true)
    }

    /// Create a new AddFeedback with some values, and negative feedback.
    pub fn new_negative(station_token: &str, track_token: &str) -> Self {
        Self::new(station_token, track_token, false)
    }
}

///
/// | Name          | Type    | Description                  |
/// | ------------- | ------- | ---------------------------- |
/// | dateCreated   | object  |                              |
/// | musicToken    | string  |                              |
/// | songName      | string  |                              |
/// | totalThumbsUp | int     |                              |
/// | feedbackId    | string  | See `station-deleteFeedback` |
/// | isPositive    | boolean |                              |
///
/// ``` json
/// {
///     "stat": "ok",
///     "result": {
///         "totalThumbsDown": 4,
///         "stationPersonalizationPercent": 57,
///         "dateCreated": {
///             "date": 9,
///             "day": 3,
///             "hours": 6,
///             "minutes": 3,
///             "month": 6,
///             "seconds": 56,
///             "time": 1404911036840,
///             "timezoneOffset": 420,
///             "year": 114
///         },
///         "albumArtUrl": "http://cont-sv5-2.pandora.com/images/public/amz/2/2/9/5/094632175922_130W_130H.jpg",
///         "musicToken": "23234b0abdbeb37d",
///         "songName": "Nothing Compares 2 U",
///         "artistName": "Sinead O'Connor",
///         "totalThumbsUp": 20,
///         "feedbackId": "21955050420286614",
///         "isPositive": false
///     }
/// }
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddFeedbackResponse {
    /// Timestamp for when the bookmark was created.
    pub date_created: Timestamp,
    /// The unique id (token) for the artist. Artist tokens start with 'R',
    /// composers with 'C', songs with 'S', and genres with 'G'.
    pub music_token: String,
    /// Total positive feedback submissions (for this user across stations? across all users?).
    pub total_thumbs_up: u32,
    /// Total negative feedback submissions (for this user across stations? across all users?).
    pub total_thumbs_down: u32,
    /// The unique id (token) for the submitted feedback.
    pub feedback_id: String,
    /// Whether feedback is positive (true) or negative (false).
    pub is_positive: bool,
    /// The name of the song being rated.
    pub song_name: String,
    /// The name of the artist being rated.
    pub artist_name: String,
    /// A link to an image of the artist.
    pub album_art_url: String,
    /// Unknown
    pub station_personalization_percent: u8,
}

/// Convenience function to do a basic addFeedback call.
pub fn add_feedback<T: ToSessionTokens>(
    session: &PandoraSession<T>,
    station_token: &str,
    track_token: &str,
    is_positive: bool,
) -> Result<AddFeedbackResponse, Error> {
    AddFeedback::new(station_token, track_token, is_positive).response(session)
}

/// music-search results can be used to add new seeds to an existing station.
///
/// | Name | Type | Description |
/// | stationToken | string | Existing station, see user::get_station_list() |
/// | musicToken | string | See music::search() |
/// ``` json
///     {
///         "musicToken": "R1119",
///         "stationToken": "1181753543028256237",
///         "userAuthToken": "XXX",
///         "syncTime": 1404912202
///     }
/// ```
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[serde(rename_all = "camelCase")]
pub struct AddMusic {
    /// The unique id (token) for the station on which the track should be rated.
    pub station_token: String,
    /// The unique id (token) for the artist/composer/song/genre to be added to
    /// the station.  Artist tokens start with 'R', composers with 'C', songs
    /// with 'S', and genres with 'G'.
    pub music_token: String,
}

impl AddMusic {
    /// Create a new AddMusic with some values.
    pub fn new(station_token: &str, music_token: &str) -> Self {
        Self {
            station_token: station_token.to_string(),
            music_token: music_token.to_string(),
        }
    }
}

///
/// | Name | Type | Description |
/// | seedId | string | Can be used to remove seed with station::delete_music() |
/// ``` json
///     {
///         "stat": "ok",
///         "result": {
///             "artistName": "Foo Fighters",
///             "musicToken": "3bcf3f314419f974",
///             "seedId": "2123197691273031149",
///             "artUrl": "http://cont-dc6-1.pandora.com/images/public/amg/portrait/pic200/drP900/P972/P97242B3S6P.jpg"
///         }
///     }
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMusicResponse {
    /// The name of the artist being rated.
    pub artist_name: String,
    /// The unique id (token) for the music object added. Artist tokens start with 'R',
    /// composers with 'C', songs with 'S', and genres with 'G'.
    pub music_token: String,
    /// Unknown
    pub seed_id: String,
    /// A link to an image of the added object.
    pub art_url: String,
}

/// Convenience function to do a basic addMusic call.
pub fn add_music<T: ToSessionTokens>(
    session: &PandoraSession<T>,
    station_token: &str,
    music_token: &str,
) -> Result<AddMusicResponse, Error> {
    AddMusic::new(station_token, music_token).response(session)
}

/// Stations can either be created with a musicToken obtained by Search or
/// trackToken from playlists (Retrieve playlist). The latter needs a musicType
/// to specify whether the track itself or its artist should be used as seed.
///
/// | Name | Type | Description |
/// | trackToken | string | See Retrieve playlist |
/// | musicType  | string | “song” or “artist” (“song” for genre stations) |
/// | musicToken | string | See Search |
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[serde(rename_all = "camelCase")]
pub struct CreateStation {
    /// The unique id (token) for the track around which the station should
    /// be created.
    pub track_token: String,
    /// The unique id (token) for the artist/composer/song/genre to be added to
    /// the station.  Artist tokens start with 'R', composers with 'C', songs
    /// with 'S', and genres with 'G'.
    pub music_token: String,
    /// Whether the artist or the song referred to by the musicToken should be
    /// used to create the station.
    pub music_type: MusicType,
}

impl CreateStation {
    /// Create a new CreateStation with some values.
    pub fn new(track_token: &str, music_token: &str, music_type: MusicType) -> Self {
        Self {
            track_token: track_token.to_string(),
            music_token: music_token.to_string(),
            music_type,
        }
    }

    /// Create a new CreateStation for a song with some values.
    pub fn new_song(track_token: &str, music_token: &str) -> Self {
        Self::new(track_token, music_token, MusicType::Song)
    }

    /// Create a new CreateStation for an artist with some values.
    pub fn new_artist(track_token: &str, music_token: &str) -> Self {
        Self::new(track_token, music_token, MusicType::Artist)
    }
}

/// Used for selecting whether a musicToken should be interpreted
/// as referring to the associated artist or the associated song.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MusicType {
    /// Use the song referred by the musicToken
    Song,
    /// Use the artist for the song referred by the musicToken
    Artist,
}

/// station.createStation has no known response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStationResponse {
    /// The fields of the createStation response are unknown.
    pub unknown: Option<serde_json::value::Value>,
}

/// Convenience function to do a basic createStation call.
pub fn create_station_from_song<T: ToSessionTokens>(
    session: &PandoraSession<T>,
    track_token: &str,
    music_token: &str,
) -> Result<CreateStationResponse, Error> {
    CreateStation::new_song(track_token, music_token).response(session)
}

/// Convenience function to do a basic createStation call.
pub fn create_station_from_artist<T: ToSessionTokens>(
    session: &PandoraSession<T>,
    track_token: &str,
    music_token: &str,
) -> Result<CreateStationResponse, Error> {
    CreateStation::new_artist(track_token, music_token).response(session)
}

/// Feedback added by Rate track can be removed from the station.
///
/// | Name |   Type |   Description |
/// | feedbackId | string | See Retrieve extended station information |
/// ``` json
/// {
///     "feedbackId": "3738252050522320365",
///     "userAuthToken": "XXX",
///     "syncTime": 1404910760
/// }
/// ```
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFeedback {
    /// The unique id (token) for the feedback submission that should be deleted.
    pub feedback_id: String,
}

impl<TS: ToString> From<&TS> for DeleteFeedback {
    fn from(feedback_id: &TS) -> Self {
        Self {
            feedback_id: feedback_id.to_string(),
        }
    }
}

/// This method does not return data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFeedbackResponse {
    /// The fields of the deleteFeedback response are unknown.
    pub unknown: Option<serde_json::value::Value>,
}

/// Convenience function to do a basic deleteFeedback call.
pub fn delete_feedback<T: ToSessionTokens>(
    session: &PandoraSession<T>,
    feedback_id: &str,
) -> Result<DeleteFeedbackResponse, Error> {
    DeleteFeedback::from(&feedback_id).response(session)
}

/// Seeds can be removed from a station, except for the last one.
///
/// | Name   | Type   | Description |
/// | seedId | string | See Retrieve extended station information and Add seed |
/// ``` json
/// {
///     "seedId": "1230715903914683885",
///     "userAuthToken": "XXX",
///     "syncTime": 1404912023
/// }
/// ```
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[serde(rename_all = "camelCase")]
pub struct DeleteMusic {
    /// The unique id (token) for the music seed that should be deleted
    /// from this station.
    pub seed_id: String,
}

impl<TS: ToString> From<&TS> for DeleteMusic {
    fn from(seed_id: &TS) -> Self {
        Self {
            seed_id: seed_id.to_string(),
        }
    }
}

/// This method does not return data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteMusicResponse {
    /// The fields of the deleteMusic response are unknown.
    pub unknown: Option<serde_json::value::Value>,
}

/// Convenience function to do a basic deleteMusic call.
pub fn delete_music<T: ToSessionTokens>(
    session: &PandoraSession<T>,
    seed_id: &str,
) -> Result<DeleteMusicResponse, Error> {
    DeleteMusic::from(&seed_id).response(session)
}

/// | Name   | Type  |  Description |
/// | stationToken  |  string | Existing station, see Retrieve station list |
/// ``` json
/// {
///     "stationToken": "374145764047334893",
///     "userAuthToken": "XXX",
///     "syncTime": 1404911699
/// }
/// ```
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[serde(rename_all = "camelCase")]
pub struct DeleteStation {
    /// The unique id (token) for the station that should be deleted.
    pub station_token: String,
}

impl<TS: ToString> From<&TS> for DeleteStation {
    fn from(station_token: &TS) -> Self {
        Self {
            station_token: station_token.to_string(),
        }
    }
}

/// No data is returned in response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteStationResponse {
    /// The fields of the deleteStation response are unknown.
    pub unknown: Option<serde_json::value::Value>,
}

/// Convenience function to do a basic deleteStation call.
pub fn delete_station<T: ToSessionTokens>(
    session: &PandoraSession<T>,
    station_token: &str,
) -> Result<DeleteStationResponse, Error> {
    DeleteStation::from(&station_token).response(session)
}

/// Check to see if the list of genre stations has changed.
///
/// | Name   | Type   | Description |
/// | includeGenreCategoryAdUrl  | bool  |  (optional) |
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[serde(rename_all = "camelCase")]
pub struct GetGenreStationsChecksum {
    /// Unknown
    pub include_genre_category_ad_url: bool,
}

impl GetGenreStationsChecksum {
    /// Create a new GetGenreStationsChecksum with some default values.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for GetGenreStationsChecksum {
    fn default() -> Self {
        Self {
            include_genre_category_ad_url: false,
        }
    }
}

/// | Name   | Type  |  Description |
/// | checksum  |  string | |
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGenreStationsChecksumResponse {
    /// The checksum for the list of genre stations. This is useful to detect
    /// when the list of genre stations has changed so that it can be requested
    /// and refreshed for the user.  This also allows for app caching of the
    /// list across session.
    pub checksum: String,
}

/// Convenience function to do a basic getGenreStationsChecksum call.
pub fn get_genre_stations_checksum<T: ToSessionTokens>(
    session: &PandoraSession<T>,
) -> Result<GetGenreStationsChecksumResponse, Error> {
    GetGenreStationsChecksum::default().response(session)
}

/// Pandora provides a list of predefined stations ("genre stations").
/// The request has no parameters.
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[serde(rename_all = "camelCase")]
pub struct GetGenreStations {}

impl GetGenreStations {
    /// Create a new GetGenreStations.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for GetGenreStations {
    fn default() -> Self {
        Self {}
    }
}

/// Each station belongs to one category, usually a genre name. stationToken
/// can be used as musicToken to create a new station with Create.
///
/// | Name   | Type  |  Description |
/// | categories | array  | List of categories |
/// | categories.stations | array |  List of stations in category |
/// | categories.stations.stationToken |   string | Actually a musicToken, see Create |
/// | categories.categoryName | string | Category name |
/// ``` json
/// {
///     "stat": "ok",
///     "result": {
///         "categories": [{
///             "stations": [{
///                 "stationToken": "G165",
///                 "stationName": "90s Alternative ",
///                 "stationId": "G165"
///             }],
///             "categoryName": "Alternative"
///         }]
///     }
/// }
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGenreStationsResponse {
    /// The checksum for the list of genre stations. This is useful to detect
    /// when the list of genre stations has changed so that it can be requested
    /// and refreshed for the user.  This also allows for app caching of the
    /// list across session.
    pub categories: Vec<GenreCategory>,
}

/// A collection of stations that fall in a broad genre category
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenreCategory {
    /// Genre/music category name
    pub category_name: String,
    /// List of stations in the category
    pub stations: Vec<GenreStation>,
}

/// A specific genre station
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenreStation {
    /// Actually a musicToken, which can be used with station.createStation.
    pub station_token: String,
    /// User-friendly name for the station.
    pub station_name: String,
    /// Unknown
    pub station_id: String,
}

/// Convenience function to do a basic getGenreStations call.
pub fn get_genre_stations<T: ToSessionTokens>(
    session: &PandoraSession<T>,
) -> Result<GetGenreStationsResponse, Error> {
    GetGenreStations::default().response(session)
}

/// This method must be sent over a TLS-encrypted connection.
///
/// | Name | Type | Description |
/// | stationToken | string | station token from Retrieve station list |
/// | additionalAudioUrl | string | Comma separated list of additional audio formats to return. (optional) |
/// | stationIsStarting | boolean | (optional) |
/// | includeTrackLength | boolean | (optional) |
/// | includeAudioToken | boolean | (optional) |
/// | xplatformAdCapable | boolean | (optional) |
/// | includeAudioReceiptUrl | boolean | (optional) |
/// | includeBackstageAdUrl | boolean | (optional) |
/// | includeSharingAdUrl | boolean | (optional) |
/// | includeSocialAdUrl | boolean | (optional) |
/// | includeCompetitiveSepIndicator | boolean | (optional) |
/// | includeCompletePlaylist | boolean | (optional) |
/// | includeTrackOptions | boolean | (optional) |
/// | audioAdPodCapable | boolean | (optional) |
///
/// Valid values for additionalAudioUrl are:
///
/// * HTTP_40_AAC_MONO
/// * HTTP_64_AAC
/// * HTTP_32_AACPLUS
/// * HTTP_64_AACPLUS
/// * HTTP_24_AACPLUS_ADTS
/// * HTTP_32_AACPLUS_ADTS
/// * HTTP_64_AACPLUS_ADTS
/// * HTTP_128_MP3
/// * HTTP_32_WMA
///
/// Usually a playlist contains four tracks.
/// ``` json
/// {
///      "userAuthToken": "XXX",
///      "additionalAudioUrl":  "HTTP_32_AACPLUS_ADTS,HTTP_64_AACPLUS_ADTS",
///      "syncTime": 1335841463,
///      "stationToken": "121193154444133035"
/// }
/// ```
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[pandora_request(encrypted = true)]
#[serde(rename_all = "camelCase")]
pub struct GetPlaylist {
    /// The unique id (token) for the station to request a playlist from
    pub station_token: String,
    /// Request a non-default audio format as an additional audio url response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_audio_url: Option<String>,
    /// Unknown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub station_is_starting: Option<bool>,
    /// Unknown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_track_length: Option<bool>,
    /// Unknown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_audio_token: Option<bool>,
    /// Unknown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xplatform_ad_capable: Option<bool>,
    /// Unknown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_audio_receipt_url: Option<bool>,
    /// Unknown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_backstage_ad_url: Option<bool>,
    /// Unknown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_sharing_ad_url: Option<bool>,
    /// Unknown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_social_ad_url: Option<bool>,
    /// Unknown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_competitive_sep_indicator: Option<bool>,
    /// Unknown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_complete_playlist: Option<bool>,
    /// Unknown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_track_options: Option<bool>,
    /// Unknown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_ad_pod_capable: Option<bool>,
}

impl<TS: ToString> From<&TS> for GetPlaylist {
    fn from(station_token: &TS) -> Self {
        Self {
            station_token: station_token.to_string(),
            additional_audio_url: Some(AudioFormat::Mp3128.to_string()),
            station_is_starting: None,
            include_track_length: None,
            include_audio_token: None,
            xplatform_ad_capable: None,
            include_audio_receipt_url: None,
            include_backstage_ad_url: None,
            include_sharing_ad_url: None,
            include_social_ad_url: None,
            include_competitive_sep_indicator: None,
            include_complete_playlist: None,
            include_track_options: None,
            audio_ad_pod_capable: None,
        }
    }
}

/// Valid values for additionalAudioUrl are:
///
/// * HTTP_40_AAC_MONO
/// * HTTP_64_AAC
/// * HTTP_32_AACPLUS
/// * HTTP_64_AACPLUS
/// * HTTP_24_AACPLUS_ADTS
/// * HTTP_32_AACPLUS_ADTS
/// * HTTP_64_AACPLUS_ADTS
/// * HTTP_128_MP3
/// * HTTP_32_WMA
#[derive(Debug, Clone)]
pub enum AudioFormat {
    /// AAC format, monaural audio, 40kbps
    AacMono40,
    /// AAC format, 64kbps
    Aac64,
    /// AACPlus format, 32kbps
    AacPlus32,
    /// AACPlus format, 64kbps
    AacPlus64,
    /// AACPlus format in an ADTS container, 24kbps
    AacPlusAdts24,
    /// AACPlus format in an ADTS container, 32kbps
    AacPlusAdts32,
    /// AACPlus format in an ADTS container, 64kbps
    AacPlusAdts64,
    /// MP3 format, 128kbps
    Mp3128,
    /// WMA format, 32kbps
    Wma32,
}

impl ToString for AudioFormat {
    fn to_string(&self) -> String {
        match self {
            AudioFormat::AacMono40 => String::from("HTTP_40_AAC_MONO"),
            AudioFormat::Aac64 => String::from("HTTP_64_AAC"),
            AudioFormat::AacPlus32 => String::from("HTTP_32_AACPLUS"),
            AudioFormat::AacPlus64 => String::from("HTTP_64_AACPLUS"),
            AudioFormat::AacPlusAdts24 => String::from("HTTP_24_AACPLUS_ADTS"),
            AudioFormat::AacPlusAdts32 => String::from("HTTP_32_AACPLUS_ADTS"),
            AudioFormat::AacPlusAdts64 => String::from("HTTP_64_AACPLUS_ADTS"),
            AudioFormat::Mp3128 => String::from("HTTP_128_MP3"),
            AudioFormat::Wma32 => String::from("HTTP_32_WMA"),
        }
    }
}

impl TryFrom<&str> for AudioFormat {
    type Error = Error;
    fn try_from(fmt: &str) -> std::result::Result<Self, Self::Error> {
        match fmt {
            "HTTP_40_AAC_MONO" => Ok(AudioFormat::AacMono40),
            "HTTP_64_AAC" => Ok(AudioFormat::Aac64),
            "HTTP_32_AACPLUS" => Ok(AudioFormat::AacPlus32),
            "HTTP_64_AACPLUS" => Ok(AudioFormat::AacPlus64),
            "HTTP_24_AACPLUS_ADTS" => Ok(AudioFormat::AacPlusAdts24),
            "HTTP_32_AACPLUS_ADTS" => Ok(AudioFormat::AacPlusAdts32),
            "HTTP_64_AACPLUS_ADTS" => Ok(AudioFormat::AacPlusAdts64),
            "HTTP_128_MP3" => Ok(AudioFormat::Mp3128),
            "HTTP_32_WMA" => Ok(AudioFormat::Wma32),
            x => Err(Self::Error::InvalidAudioFormat(x.to_string())),
        }
    }
}

impl TryFrom<String> for AudioFormat {
    type Error = Error;
    fn try_from(fmt: String) -> std::result::Result<Self, Self::Error> {
        Self::try_from(fmt.as_str())
    }
}

/// | Name | Type | Description |
/// | items.additionalAudioUrl | array/string | List of additional audio urls in the requested order or single string if only one format was requested |
/// | items.songRating | int | 1 if song was given a thumbs up, 0 if song was not rated yet |
/// | items.audioUrlMap | object | Song audio format and bitrates returned differ based on what partner credentials are used. |
/// ``` json
/// {
///      "stat": "ok",
///      "result": {
///          "items": [{
///              "trackToken": "40b892bc5376e695c2e5c2b347227b85af2761b6aa417f736d9a79319b8f4cb97c9695a5f9a9a32aa2abaed43571235c",
///              "artistName": "Cannabich, Christian",
///              "albumName": "London Mozart Players, Christian Cannabich: Symphonies",
///              "amazonAlbumUrl": "http://www.amazon.com/dp/B000GW8ATU/?tag=wwwpandoracom-20",
///              "songExplorerUrl": "http://www.pandora.com/xml/music/song/london-mozart-players/christian-cannabich-symphonies/2-andantino?explicit=false",
///              "albumArtUrl": "http://cont-sv5-2.pandora.com/images/public/amz/5/2/9/7/095115137925_500W_488H.jpg",
///              "artistDetailUrl": "http://www.pandora.com/christian-cannabich?...",
///              "audioUrlMap": {
///                  "highQuality": {
///                      "bitrate": "64",
///                      "encoding": "aacplus",
///                      "audioUrl": "http://audio-sjl-t1-2.pandora.com/access/166132182435087962.mp4?...",
///                      "protocol": "http"
///                  },
///                  "mediumQuality": {
///                      "bitrate": "64",
///                      "encoding": "aacplus",
///                      "audioUrl": "http://t1-2.cdn.pandora.com/access/4127124196771074419.mp4?...",
///                      "protocol": "http"
///                  },
///                  "lowQuality": {
///                      "bitrate": "32",
///                      "encoding": "aacplus",
///                      "audioUrl": "http://audio-sv5-t1-1.pandora.com/access/3464788359714661029.mp4?...",
///                      "protocol": "http"
///                  }
///              },
///              "itunesSongUrl": "http://click.linksynergy.com/fs-bin/stat?...",
///              "additionalAudioUrl": [
///                  "http://t1-2.cdn.pandora.com/access/6705986462049243054.mp4?...",
///                  "http://audio-sjl-t1-1.pandora.com/access/2473529637452270302.mp4?..."
///              ],
///              "amazonAlbumAsin": "B000GW8ATU",
///              "amazonAlbumDigitalAsin": "B003H37NN4",
///              "artistExplorerUrl": "http://www.pandora.com/xml/music/composer/christian-cannabich?explicit=false",
///              "songName": "Symphony In G Major",
///              "albumDetailUrl": "http://www.pandora.com/london-mozart-players/christian-cannabich-symphonies?...",
///              "songDetailUrl": "http://www.pandora.com/london-mozart-players/christian-cannabich-symphonies/2-andantino?...",
///              "stationId": "121193154444133035",
///              "songRating": 0,
///              "trackGain": "10.09",
///              "albumExplorerUrl": "http://www.pandora.com/xml/music/album/london-mozart-players/christian-cannabich-symphonies?explicit=false",
///              "allowFeedback": true,
///              "amazonSongDigitalAsin": "B003H39AGW",
///              "nowPlayingStationAdUrl": "http://ad.doubleclick.net/pfadx/pand.android/prod.nowplaying..."
///          }, {
///              "adToken": "121193154444133035-none"
///          },
///          ]
///      }
/// }
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPlaylistResponse {
    /// Contains a list of playlist entries, each being either a song/track or
    /// an ad.
    pub items: Vec<PlaylistEntry>,
}

/// Responses can be either a track or an ad.
/// The responses don't have a standard tag identifying which type it is,
/// but ads have only one value: adToken: String.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum PlaylistEntry {
    /// Playlist entry representing an ad.
    PlaylistAd(PlaylistAd),
    /// Playlist entry representing a song/track.
    PlaylistTrack(PlaylistTrack),
}

impl PlaylistEntry {
    /// Returns whether the playlist entry is an ad
    pub fn is_ad(&self) -> bool {
        match self {
            PlaylistEntry::PlaylistAd(_) => true,
            _ => false,
        }
    }

    /// Returns whether the playlist entry is a track
    pub fn is_track(&self) -> bool {
        match self {
            PlaylistEntry::PlaylistTrack(_) => true,
            _ => false,
        }
    }

    /// Returns the PlaylistAd object for this entry, if any
    pub fn get_ad(&self) -> Option<PlaylistAd> {
        match self {
            PlaylistEntry::PlaylistAd(a) => Some(a.clone()),
            _ => None,
        }
    }

    /// Returns the PlaylistTrack object for this entry, if any
    pub fn get_track(&self) -> Option<PlaylistTrack> {
        match self {
            PlaylistEntry::PlaylistTrack(t) => Some(t.clone()),
            _ => None,
        }
    }
}

/// Represents an ad entry in a playlist.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistAd {
    /// The unique id (token) for the ad which should be played.
    pub ad_token: String,
}

/// Represents a track (song) entry in a playlist.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistTrack {
    /// The unique id (token) for the track to be played.
    pub track_token: String,
    /// The unique id (token) for the station from which this track was
    /// requested.
    pub station_id: String,
    /// The default audio streams available for this track.
    pub audio_url_map: AudioQuality,
    /// Additional audio stream formats requested for this track.
    pub additional_audio_url: Vec<String>,
    /// A floating point value, encoded as a string, representing the track gain
    /// that should be applied for playback.
    pub track_gain: String,
    /// The name of the artist for this track.
    pub artist_name: String,
    /// The name of the album for this track.
    pub album_name: String,
    /// The name of the song for this track.
    pub song_name: String,
    /// The rating of the song for this track.
    pub song_rating: u32,
    /// Whether this track allows submitting feedback.
    pub allow_feedback: Option<bool>,
    /// Unknown
    pub song_explorer_url: Option<String>,
    /// Unknown
    pub artist_explorer_url: Option<String>,
    /// Unknown
    pub album_explorer_url: Option<String>,
    /// Unknown
    pub album_art_url: Option<String>,
    /// Unknown
    pub song_detail_url: Option<String>,
    /// Unknown
    pub artist_detail_url: Option<String>,
    /// Unknown
    pub album_detail_url: Option<String>,
    /// URL to lookup/buy song on iTunes.
    pub itunes_song_url: Option<String>,
    /// URL to lookup/buy song on Amazon.
    pub amazon_song_digital_asin: Option<String>,
    /// URL to lookup/buy album on Amazon.
    pub amazon_album_url: Option<String>,
    /// Amazon product code for the album product.
    pub amazon_album_asin: Option<String>,
    /// Amazon product code for the digital album product.
    pub amazon_album_digital_asin: Option<String>,
    /// Unknown
    pub now_playing_station_ad_url: Option<String>,
}

///                  "lowQuality": {
///                      "bitrate": "32",
///                      "encoding": "aacplus",
///                      "audioUrl": "http://audio-sv5-t1-1.pandora.com/access/3464788359714661029.mp4?...",
///                      "protocol": "http"
///                  }
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioQuality {
    /// Attributes for the high quality audio stream.
    pub high_quality: AudioStream,
    /// Attributes for the medium quality audio stream.
    pub medium_quality: AudioStream,
    /// Attributes for the low quality audio stream.
    pub low_quality: AudioStream,
}

/// Playback/decoding attributes of an available audio stream.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioStream {
    /// The audio bitrate/quality for this stream.
    pub bitrate: String,
    /// The audio encoding format for this stream.
    pub encoding: String,
    /// The url to stream audio from.
    pub audio_url: String,
    /// The protocol to use with the audio URL.
    pub protocol: String,
}

/// Convenience function to do a basic getPlaylist call.
pub fn get_playlist<T: ToSessionTokens>(
    session: &PandoraSession<T>,
    station_token: &str,
) -> Result<GetPlaylistResponse, Error> {
    GetPlaylist::from(&station_token).response(session)
}

/// Extended station information includes seeds and feedback.
///
/// | Name | Type | Description |
/// | stationToken | string |  |
/// | includeExtendedAttributes | bool |  |
/// ``` json
/// {
///     "stationToken": "374145764047334893",
///     "includeExtendedAttributes": true,
///     "userAuthToken": "XXX",
///     "syncTime": 1404910732
/// }
/// ```
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[serde(rename_all = "camelCase")]
pub struct GetStation {
    /// The unique id (token) for the station to request information on.
    pub station_token: String,
    /// Include additional station attributes in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_extended_attributes: Option<bool>,
}

impl<TS: ToString> From<&TS> for GetStation {
    fn from(station_token: &TS) -> Self {
        GetStation {
            station_token: station_token.to_string(),
            include_extended_attributes: None,
        }
    }
}

/// | Name | Type | Description |
/// | music | object | Station seeds, see Add seed |
/// | music.songs | list | Song seeds |
/// | music.artists | list | Artist seeds |
/// | feedback | object | Feedback added by Rate track |
/// | feedback.thumbsUp | list |   |
/// | feedback.thumbsDown | list |   |
/// ``` json
/// {
///     "stat": "ok",
///     "result": {
///         "suppressVideoAds": false,
///         "stationId": "374145764047334893",
///         "allowAddMusic": true,
///         "dateCreated": {
///             "date": 15,
///             "day": 6,
///             "hours": 7,
///             "minutes": 34,
///             "month": 0,
///             "nanos": 874000000,
///             "seconds": 21,
///             "time": 1295105661874,
///             "timezoneOffset": 480,
///             "year": 111
///         },
///         "stationDetailUrl": "https://www.pandora.com/login?target=%2Fstations%2Fc644756145fc3f5df1916901125ee697495159685ae39575",
///         "artUrl": "http://cont-1.p-cdn.com/images/public/amz/5/2/8/5/075678235825_500W_498H.jpg",
///         "requiresCleanAds": false,
///         "stationToken": "374145764047334893",
///         "stationName": "Winter Radio",
///         "music": {
///             "songs": [{
///                 "seedId": "428301990230109677",
///                 "artistName": "Tori Amos",
///                 "artUrl": "http://cont-sjl-1.pandora.com/images/public/amz/5/2/8/5/075678235825_130W_130H.jpg",
///                 "songName": "Winter",
///                 "musicToken": "87ef9db1c3f04330"
///             }],
///             "artists": [{
///                 "artistName": "Jason Derulo",
///                 "musicToken": "563f577e00d837a5",
///                 "seedId": "31525199612287328",
///                 "artUrl": "http://mediaserver-cont-sv5-1-v4v6.pandora.com/images/public/amg/portrait/pic200/drQ300/Q366/Q36675SDAPJ.jpg"
///             }],
///             "genres": [{
///                 "musicToken": "cc021b31a48b8acf",
///                 "genreName": "Today's Hits",
///                 "seedId": "31525199599467854"
///             }]
///         },
///         "isShared": false,
///         "allowDelete": true,
///         "genre": ["Rock"],
///         "isQuickMix": false,
///         "allowRename": true,
///         "stationSharingUrl": "https://www.pandora.com/login?target=%2Fshare%2Fstation%2Fc644756145fc3f5df1916901125ee697495159685ae39575",
///         "allowEditDescription": true,
///         "feedback": {
///             "thumbsUp": [{
///                 "dateCreated": {
///                     "date": 28,
///                     "day": 5,
///                     "hours": 13,
///                     "minutes": 57,
///                     "month": 2,
///                     "nanos": 760000000,
///                     "seconds": 49,
///                     "time": 1396040269760,
///                     "timezoneOffset": 420,
///                     "year": 114
///                 },
///                 "albumArtUrl": "http://cont-1.p-cdn.com/images/public/amz/9/7/1/4/900004179_130W_130H.jpg",
///                 "musicToken": "d33dd0c199ebaf28425ba2910f7abf8b",
///                 "songName": "Hey Lover",
///                 "artistName": "Keri Noble",
///                 "feedbackId": "-7239441039566426643",
///                 "isPositive": true
///             }],
///             "totalThumbsUp": 20,
///             "totalThumbsDown": 5,
///             "thumbsDown": [{
///                 "dateCreated": {
///                     "date": 28,
///                     "day": 5,
///                     "hours": 10,
///                     "minutes": 43,
///                     "month": 2,
///                     "nanos": 637000000,
///                     "seconds": 30,
///                     "time": 1396028610637,
///                     "timezoneOffset": 420,
///                     "year": 114
///                 },
///                 "albumArtUrl": "http://cont-ch1-1.pandora.com/images/public/amz/9/0/5/1/724383771509_130W_130H.jpg",
///                 "musicToken": "5a0018da7876f6e7",
///                 "songName": "Talk Show Host",
///                 "artistName": "Radiohead",
///                 "feedbackId": "-7241622182873125395",
///                 "isPositive": false
///             }]
///         }
///     }
/// }
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStationResponse {
    /// The unique id (token) for the station for which information was
    /// requested. The stationId (station_id) and stationToken (station_token)
    /// attributes appear to be duplicates.
    pub station_id: String,
    /// The unique id (token) for the station for which information was
    /// requested. The stationId (station_id) and stationToken (station_token)
    /// attributes appear to be duplicates.
    pub station_token: String,
    /// The user-created name of the station.
    pub station_name: String,
    /// Whether the station allows adding music to it.
    pub allow_add_music: Option<bool>,
    /// Unknown
    pub suppress_video_ads: Option<bool>,
    /// When the station was created.
    pub date_created: Timestamp,
    /// Unknown
    pub station_detail_url: Option<String>,
    /// Unknown
    pub art_url: Option<String>,
    /// Unknown
    pub requires_clean_ads: Option<bool>,
    /// Station music seeds.
    pub music: StationSeeds,
    /// Whether the station is visible for sharing.
    pub is_shared: Option<bool>,
    /// Whether the station can be deleted.
    pub allow_delete: Option<bool>,
    /// The genre(s) the station belongs to.
    pub genre: Vec<String>,
    /// Whether this is a QuickMix station.
    pub is_quick_mix: Option<bool>,
    /// Whether the station may be renamed.
    pub allow_rename: Option<bool>,
    /// The URL to use for sharing this station.
    pub station_sharing_url: Option<String>,
    /// Whether the description for this station may be edited.
    pub allow_edit_description: Option<bool>,
    /// Feedback submitted for tracks on this station.
    pub feedback: StationFeedback,
}

/// ``` json
///         "music": {
///             "songs": [],
///             "artists": [],
///             "genres": []
///         },
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationSeeds {
    /// Songs used as seeds for this station.
    pub songs: Vec<Seed>,
    /// Atrists used as seeds for this station.
    pub artist: Vec<Seed>,
    /// Genres used as seeds for this station.
    pub genres: Vec<Seed>,
}

/// ``` json
///             "songs": [{
///                 "seedId": "428301990230109677",
///                 "artistName": "Tori Amos",
///                 "artUrl": "http://cont-sjl-1.pandora.com/images/public/amz/5/2/8/5/075678235825_130W_130H.jpg",
///                 "songName": "Winter",
///                 "musicToken": "87ef9db1c3f04330"
///             }],
///             "artists": [{
///                 "artistName": "Jason Derulo",
///                 "musicToken": "563f577e00d837a5",
///                 "seedId": "31525199612287328",
///                 "artUrl": "http://mediaserver-cont-sv5-1-v4v6.pandora.com/images/public/amg/portrait/pic200/drQ300/Q366/Q36675SDAPJ.jpg"
///             }],
///             "genres": [{
///                 "musicToken": "cc021b31a48b8acf",
///                 "genreName": "Today's Hits",
///                 "seedId": "31525199599467854"
///             }]
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Seed {
    /// Attributes of a song seed for a station.
    Song {
        /// Unique identifier/handle for this seed.
        seed_id: String,
        /// Identifier for the song used for this seed.
        music_token: String,
        /// Name of the song used for this seed.
        song_name: String,
        /// Name of the artist for the song used for this seed.
        artist_name: String,
        /// Unknown
        art_url: String,
    },
    /// Attributes of an artist seed for a station.
    Artist {
        /// Unique identifier/handle for this seed.
        seed_id: String,
        /// Identifier for the artist used for this seed.
        music_token: String,
        /// Name of the artist used for this seed.
        artist_name: String,
        /// Unknown
        art_url: String,
    },
    /// Attributes of a genre seed for a station.
    Genre {
        /// Unique identifier/handle for this seed.
        seed_id: String,
        /// Identifier for the genre used for this seed.
        music_token: String,
        /// Name of the genre used for this seed.
        genre_name: String,
    },
}

impl Seed {
    /// Return the name of the object used as a seed.
    pub fn get_name(&self) -> String {
        match self {
            Seed::Song { song_name, .. } => song_name.to_string(),
            Seed::Artist { artist_name, .. } => artist_name.to_string(),
            Seed::Genre { genre_name, .. } => genre_name.to_string(),
        }
    }

    /// Return the unique identifier/handle used to refer to this seed.
    pub fn get_seed_id(&self) -> String {
        match self {
            Seed::Song { seed_id, .. } => seed_id.to_string(),
            Seed::Artist { seed_id, .. } => seed_id.to_string(),
            Seed::Genre { seed_id, .. } => seed_id.to_string(),
        }
    }

    /// Return the identifier for the object used as a seed.
    pub fn get_music_token(&self) -> String {
        match self {
            Seed::Song { music_token, .. } => music_token.to_string(),
            Seed::Artist { music_token, .. } => music_token.to_string(),
            Seed::Genre { music_token, .. } => music_token.to_string(),
        }
    }
}

/// ``` json
///         "feedback": {
///             "thumbsUp": [],
///             "totalThumbsUp": 20,
///             "totalThumbsDown": 5,
///             "thumbsDown": []
///         }
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationFeedback {
    /// A list of positive feedback submitted to a station.
    pub thumbs_up: Vec<TrackFeedback>,
    /// The total number of positive submissions to a station.
    pub total_thumbs_up: u32,
    /// A list of negative feedback submitted to a station.
    pub thumbs_down: Vec<TrackFeedback>,
    /// The total number of negative submissions to a station.
    pub total_thumbs_down: u32,
}

/// ``` json
///             "thumbsDown": [{
///                 "dateCreated": {
///                     "date": 28,
///                     "day": 5,
///                     "hours": 10,
///                     "minutes": 43,
///                     "month": 2,
///                     "nanos": 637000000,
///                     "seconds": 30,
///                     "time": 1396028610637,
///                     "timezoneOffset": 420,
///                     "year": 114
///                 },
///                 "albumArtUrl": "http://cont-ch1-1.pandora.com/images/public/amz/9/0/5/1/724383771509_130W_130H.jpg",
///                 "musicToken": "5a0018da7876f6e7",
///                 "songName": "Talk Show Host",
///                 "artistName": "Radiohead",
///                 "feedbackId": "-7241622182873125395",
///                 "isPositive": false
///             }]
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackFeedback {
    /// Unique identifier/handle referring to this feedback submission.
    pub feedback_id: String,
    /// Name of the song that was rated.
    pub song_name: String,
    /// Name of the artist for the song that was rated.
    pub artist_name: String,
    /// Whether the rating is positive (true) or negative (false).
    pub is_positive: bool,
    /// A token referring to the song that was rated.
    pub music_token: String,
    /// Date the feedback was created.
    pub date_created: Timestamp,
    /// Unknown
    pub album_art_url: String,
}

/// Convenience function to do a basic getStation call.
pub fn get_station<T: ToSessionTokens>(
    session: &PandoraSession<T>,
    station_token: &str,
) -> Result<GetStationResponse, Error> {
    GetStation::from(&station_token).response(session)
}

/// **Unsupported!**
/// Undocumented method
/// [station.publishStationShare()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct PublishStationShareUnsupported {}

/// | Name   | Type |   Description |
/// | stationToken  |  string | Existing station, see Retrieve station list |
/// | stationName | string | New station name |
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[serde(rename_all = "camelCase")]
pub struct RenameStation {
    /// The unique id (token) for the station that should be renamed.
    /// Also sometimes referred to as a stationId.
    pub station_token: String,
    /// The new name that should be used for this station.
    pub station_name: String,
}

impl RenameStation {
    /// Create a new RenameStation with some initial values.
    pub fn new(station_token: &str, station_name: &str) -> Self {
        Self {
            station_token: station_token.to_string(),
            station_name: station_name.to_string(),
        }
    }
}

/// There's no known response data to this request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameStationResponse {
    /// The fields of the renameStation response, if any, are unknown.
    pub unknown: Option<serde_json::value::Value>,
}

/// Convenience function to do a basic renameStation call.
pub fn rename_station<T: ToSessionTokens>(
    session: &PandoraSession<T>,
    station_token: &str,
    station_name: &str,
) -> Result<RenameStationResponse, Error> {
    RenameStation::new(station_token, station_name).response(session)
}

/// Shares a station with the specified email addresses. that emails is a string array
///
/// | Name  |  Type |   Description |
/// | stationId |  string | See Retrieve station list |
/// | stationToken |   string | See Retrieve station list |
/// | emails | string[] |   A list of emails to share the station with |
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[serde(rename_all = "camelCase")]
pub struct ShareStation {
    /// The unique id (token) for the station that should be shared.
    /// Also sometimes referred to as a stationId.
    pub station_id: String,
    /// The unique id (token) for the station that should be shared.
    /// Also sometimes referred to as a stationId.
    pub station_token: String,
    /// A list of emails to share the station with.
    pub emails: Vec<String>,
}

impl ShareStation {
    /// Create a new RenameStation with some initial values.  Call
    /// add_recipient() to add recipient emails to the request.
    pub fn new(station_id: &str, station_token: &str) -> Self {
        Self {
            station_id: station_id.to_string(),
            station_token: station_token.to_string(),
            emails: Vec::new(),
        }
    }

    /// Add a recipient email to the request.
    pub fn add_recipient(&mut self, recipient: &str) {
        self.emails.push(recipient.to_string());
    }
}

/// There's no known response data to this request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShareStationResponse {
    /// The fields of the shareStation response, if any, are unknown.
    pub unknown: Option<serde_json::value::Value>,
}

/// Convenience function to do a basic shareStation call.
pub fn share_station<T: ToSessionTokens>(
    session: &PandoraSession<T>,
    station_id: &str,
    station_token: &str,
    emails: Vec<String>,
) -> Result<ShareStationResponse, Error> {
    let mut request = ShareStation::new(station_id, station_token);
    request.emails = emails;
    request.response(session)
}

/// Stations created by other users are added as reference to the user’s
/// station list. These stations cannot be modified (i.e. rate tracks) unless
/// transformed.
///
/// | Name   |  Type  |   Description |
/// | stationToken  |   string |  See Retrieve station list |
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[serde(rename_all = "camelCase")]
pub struct TransformSharedStation {
    /// The unique id (token) for the shared station that should be converted to
    /// a personal station.
    /// Also sometimes referred to as a stationId.
    pub station_token: String,
}

impl<TS: ToString> From<&TS> for TransformSharedStation {
    fn from(station_token: &TS) -> Self {
        Self {
            station_token: station_token.to_string(),
        }
    }
}

/// There's no known response data to this request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformSharedStationResponse {
    /// The fields of the transformSharedStation response, if any, are unknown.
    pub unknown: Option<serde_json::value::Value>,
}

/// Convenience function to do a basic transformSharedStation call.
pub fn transform_shared_station<T: ToSessionTokens>(
    session: &PandoraSession<T>,
    station_token: &str,
) -> Result<TransformSharedStationResponse, Error> {
    TransformSharedStation::from(&station_token).response(session)
}
