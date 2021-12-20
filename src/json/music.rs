/*!
Music support methods.
*/
// SPDX-License-Identifier: MIT AND WTFPL
use std::collections::HashMap;

use pandora_api_derive::PandoraRequest;
use serde::{Deserialize, Serialize};

use crate::errors::Error;
use crate::json::{PandoraApiRequest, PandoraSession};

/// **Unsupported!**
/// Undocumented method
/// [music.getSearchRecommendations()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct GetSearchRecommendationsUnsupported {}

/// This method returns a description of the track associated with the provided
/// musicId included with each track in a playlist.
/// | musicId | String | as returned from a playlist that has not yet expired |
///
/// [music.getTrack()](https://github.com/pithos/pithos/issues/351)
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[pandora_request(encrypted = true)]
#[serde(rename_all = "camelCase")]
pub struct GetTrack {
    /// The token for the track as returned by the playlist
    pub music_id: String,
}

impl<TS: ToString> From<&TS> for GetTrack {
    fn from(music_id: &TS) -> Self {
        Self {
            music_id: music_id.to_string(),
        }
    }
}

/// Get extended information for a track as returned by a playlist.
///
/// See https://github.com/pithos/pithos/issues/351 for additional
/// information
/// [music.getTrack()](
///
/// | Name | Type | Description |
/// | artistName | String | |
/// | albumName | String | |
/// | songName | String | |
/// | trackToken | String | |
/// | musicId | String | |
/// | musicToken | String | |
/// ``` json
/// {
///     "stat": "ok",
///     "result": {
///         'albumName': 'Lukas Graham',
///         'trackToken': 'S5264080',
///         'artistName': 'Lukas Graham',
///         'albumArtUrl':
///             'http://mediaserver-cont-dc6-2-v4v6.pandora.com/images/public/gracenote/albumart/9/6/6/9/800079669_500W_500H.jpg',
///         'score': '',
///         'songName': '7 Years',
///         'musicId': 'S5264080',
///         'songDetailUrl':
///             'http://www.pandora.com/lukas-graham/lukas-graham/7-years',
///         'musicToken': '2b0dc86c994aa1e9425ba2910f7abf8b'
///     }
/// }
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTrackResponse {
    /// The name of the song for the provided token.
    pub song_name: String,
    /// The name of the artist for the provided token.
    pub artist_name: String,
    /// The name of the album for the provided token.
    pub album_name: String,
    /// The track token that is unique to the playlist is was provided with.
    pub track_token: String,
    /// The unique id (token) for the song. Artist tokens start with 'R',
    /// composers with 'C', songs with 'S', and genres with 'G'.
    pub music_id: String,
    /// A unique token for a song/track.
    pub music_token: String,
    /// Additional optional or undocumented fields of a GetTrack response.
    #[serde(flatten)]
    pub optional: HashMap<String, serde_json::value::Value>,
}

/// Convenience function to do a basic getTrack call.
pub async fn get_track(
    session: &mut PandoraSession,
    track_token: &str,
) -> Result<GetTrackResponse, Error> {
    GetTrack::from(&track_token).response(session).await
}

/// **Unsupported!**
/// Undocumented method
/// [music.publishSongShare()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct PublishSongShareUnsupported {}

/// This is a free text search that matches artist and track names.
///
/// | Name | Type | Description |
/// |searchText | string | Artist name or track title |
/// |includeNearMatches | bool | (optional) |
/// |includeGenreStations | bool | (optional) |
/// ``` json
/// {
///     "searchText": "encore",
///     "userAuthToken": "XXX",
///     "syncTime": 1335869287
/// }
/// ```
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[pandora_request(encrypted = true)]
#[serde(rename_all = "camelCase")]
pub struct Search {
    /// The text to search for in artist names or track titles.
    pub search_text: String,
    /// Optional parameters on the call
    #[serde(flatten)]
    pub optional: HashMap<String, serde_json::value::Value>,
}

impl Search {
    /// Convenience function for setting boolean flags in the request. (Chaining call)
    pub fn and_boolean_option(mut self, option: &str, value: bool) -> Self {
        self.optional
            .insert(option.to_string(), serde_json::value::Value::from(value));
        self
    }

    /// Whether request should include partial matches in the response. (Chaining call)
    pub fn include_near_matches(self, value: bool) -> Self {
        self.and_boolean_option("includeNearMatches", value)
    }

    /// Whether request should include genre stations in the response. (Chaining call)
    pub fn include_genre_stations(self, value: bool) -> Self {
        self.and_boolean_option("includeGenreStations", value)
    }
}

impl<TS: ToString> From<&TS> for Search {
    fn from(search_text: &TS) -> Self {
        Self {
            search_text: search_text.to_string(),
            optional: HashMap::new(),
        }
    }
}

/// Convenience function to do a basic addSongBookmark call.
pub async fn search(
    session: &mut PandoraSession,
    search_text: &str,
) -> Result<SearchResponse, Error> {
    Search::from(&search_text)
        .include_near_matches(false)
        .include_genre_stations(false)
        .response(session)
        .await
}

/// Matching songs, artists, and genre stations are returned in three separate lists.
///
/// | Name | Type | Description |
/// | songs.musicToken | string | Token starts with ‘S’ followed by one or more digits (e.g. ‘S1234567’). |
/// | artists.musicToken | string | Results can be either for artists (token starts with ‘R’) or composers (token starts with ‘C’). |
/// | genreStations.musicToken | string | Token starts with ‘G’ followed by one or more digits (e.g. ‘G123’). |
/// ``` json
/// {
///     "stat": "ok",
///     "result": {
///          "nearMatchesAvailable": true,
///          "explanation": "",
///          "songs": [{
///              "artistName": "Jason DeRulo",
///              "musicToken": "S1508963",
///              "songName": "Encore",
///              "score": 100
///          }],
///          "artists": [{
///              "artistName": "Encore",
///             "musicToken": "R175304",
///             "likelyMatch": false,
///             "score": 100
///         }],
///         "genreStations": [{
///             "musicToken": "G123",
///             "score": 100,
///             "stationName": "Today's Encore"
///         }]
///     }
/// }
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    /// Songs matching the search.
    #[serde(default)]
    pub songs: Vec<SongMatch>,
    /// Artists matching the search.
    #[serde(default)]
    pub artists: Vec<ArtistMatch>,
    /// Genre stations matching the search.
    #[serde(default)]
    pub genre_stations: Vec<GenreMatch>,
    /// Additional optional fields that may appear in the response.
    #[serde(flatten)]
    pub optional: HashMap<String, serde_json::value::Value>,
}

/// Structure collecting the song information returned
/// by searches.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SongMatch {
    /// Name of the matched song.
    pub song_name: String,
    /// The name of the artist found in the search.
    pub artist_name: String,
    /// The unique id (token) for the song. Artist tokens start with 'R',
    /// composers with 'C', songs with 'S', and genres with 'G'.
    pub music_token: String,
    /// A rating of how close the match is.
    pub score: u8,
}

/// Structure collecting the artist information returned
/// by searches.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistMatch {
    /// The name of the artist found in the search.
    pub artist_name: String,
    /// The unique id (token) for the song. Artist tokens start with 'R',
    /// composers with 'C', songs with 'S', and genres with 'G'.
    pub music_token: String,
    /// Whether the match is just a close, but not perfect, match.
    pub likely_match: bool,
    /// A rating of how close the match is.
    pub score: u8,
}

/// Structure collecting the genre-station information returned
/// by searches.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenreMatch {
    /// The unique id (token) for the song. Artist tokens start with 'R',
    /// composers with 'C', songs with 'S', and genres with 'G'.
    pub music_token: String,
    /// A rating of how close the match is.
    pub score: u8,
    /// The name of the genre station found in the search.
    pub station_name: String,
}

/// **Unsupported!**
/// Undocumented method
/// [music.shareMusic()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct ShareMusicUnsupported {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::{
        station::get_playlist, tests::session_login, user::get_station_list, Partner,
    };

    #[async_std::test]
    async fn search_test() {
        let partner = Partner::default();
        let mut session = session_login(&partner).await.expect("Failed initializing login session");

        let _search_response =
            search(&mut session, "INXS").await.expect("Failed completing search request");
        let _search_response: SearchResponse = Search::from(&"Alternative")
            .include_genre_stations(true)
            .response(&mut session).await
            .expect("Failed completing search request");
    }

    #[async_std::test]
    async fn get_track_test() {
        let partner = Partner::default();
        let mut session = session_login(&partner).await.expect("Failed initializing login session");

        for station in get_station_list(&mut session).await
            .expect("Failed getting station list to look up a track to bookmark")
            .stations
        {
            for track in get_playlist(&mut session, &station.station_token).await
                .expect("Failed completing request for playlist")
                .items
                .iter()
                .flat_map(|p| p.get_track())
            {
                if let Some(serde_json::value::Value::String(music_id)) =
                    track.optional.get("musicId")
                {
                    let _response = get_track(&mut session, &music_id).await
                        .expect("Failed getting track information");
                }
            }
        }
    }
}
