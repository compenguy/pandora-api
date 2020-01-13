/*!
Bookmark support messages.

Users can bookmark artists or songs.
*/
// SPDX-License-Identifier: MIT AND WTFPL

use pandora_api_derive::PandoraRequest;
use serde::{Deserialize, Serialize};

use crate::errors::Error;
use crate::json::{PandoraApiRequest, Timestamp};

/// | Name | Type | Description |
/// | trackToken | string | |
/// ``` json
/// {
///     "trackToken": "f17ff6c86c11743fc890808e1a1dd6ff5b1dca1a2e260f7d998ba6e7786dd9941c5dd4b345a1008e86862353da1e6cdc78172b4199240c76",
///     "userAuthToken": "XXX",
///     "syncTime": 1338210690
/// }
/// ```
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[serde(rename_all = "camelCase")]
pub struct AddArtistBookmark {
    /// The unique id (token) for this track.
    pub track_token: String,
}

///
/// ``` json
/// {
///     "stat": "ok",
///     "result": {
///         "artistName": "Wallis Bird",
///         "dateCreated": {
///             "date": 2,
///             "day": 3,
///             "hours": 7,
///             "minutes": 6,
///             "month": 6,
///             "seconds": 13,
///             "time": 1404309973468,
///             "timezoneOffset": 420,
///             "year": 114
///         },
///         "bookmarkToken": "49854851068341741",
///         "artUrl": "http://cont-dc6-2.pandora.com/images/public/amg/portrait/pic200/drP900/P998/P99805K1QKS.jpg",
///         "musicToken": "R278544"
///     }
/// }
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddArtistBookmarkResponse {
    /// The name of the artist being bookmarked.
    pub artist_name: String,
    /// Timestamp for when the bookmark was created.
    pub date_created: Timestamp,
    /// The unique id (token) for the newly-created bookmark.
    pub bookmark_token: String,
    /// A link to an image of the artist.
    pub art_url: String,
    /// The unique id (token) for the artist. Artist tokens start with 'R',
    /// composers with 'C', songs with 'S', and genres with 'G'.
    pub music_token: String,
}

/// | Name | Type | Description |
/// | trackToken | string | |
/// ``` json
/// {
///     "trackToken": "f17ff6c86c11743fc890808e1a1dd6ff5b1dca1a2e260f7d998ba6e7786dd9941c5dd4b345a1008e86862353da1e6cdc78172b4199240c76",
///     "userAuthToken": "XXX",
///     "syncTime": 1338210690
/// }
/// ```
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[serde(rename_all = "camelCase")]
pub struct AddSongBookmark {
    /// The unique id (token) for this track.
    pub track_token: String,
}

///
/// ``` json
/// {
///     "stat": "ok",
///     "result": {
///         "sampleGain": "1.96",
///         "musicToken": "S1143982",
///         "bookmarkToken": "200207779061968365",
///         "sampleUrl": "http://www.pandora.com/favorites/getSample.jsp?token=a74b4f7551e3e174425ba2910f7abf8b&allowExplicit=true",
///         "albumName": "The 5th Exotic",
///         "songName": "The 5th Exotic",
///         "artUrl": "http://cont-sjl-1.pandora.com/images/public/amz/9/4/5/2/800002549_500W_500H.jpg",
///         "dateCreated": {
///             "date": 28,
///             "day": 1,
///             "hours": 6,
///             "minutes": 11,
///             "month": 4,
///             "seconds": 31,
///             "time": 1338210691404,
///             "timezoneOffset": 420,
///             "year": 112
///         },
///         "artistName": "Quantic"
///     }
/// }
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddSongBookmarkResponse {
    /// The audio gain for the bookmarked track. (?)
    pub sample_gain: String,
    /// The unique id (token) for the song. Artist tokens start with 'R',
    /// composers with 'C', songs with 'S', and genres with 'G'.
    pub music_token: String,
    /// The unique id (token) for the newly-created bookmark.
    pub bookmark_token: String,
    /// Url for a sample of the bookmarked song.
    pub sample_url: String,
    /// The name of the album for the song being bookmarked.
    pub album_name: String,
    /// The name of the song being bookmarked.
    pub song_name: String,
    /// A link to an image of the artist.
    pub art_url: String,
    /// Timestamp for when the bookmark was created.
    pub date_created: Timestamp,
    /// The name of the artist being bookmarked.
    pub artist_name: String,
}

/// **Unsupported!**
/// Undocumented method
/// [bookmark.deleteArtistBookmark()](https://6xq.net/pandora-apidoc/json/methods/)
pub fn delete_artist_bookmark() {
    unimplemented!();
}

/// **Unsupported!**
/// Undocumented method
/// [bookmark.deleteSongBookmark()](https://6xq.net/pandora-apidoc/json/methods/)
pub fn delete_song_bookmark() {
    unimplemented!();
}
