/*!
Music support methods.
*/
// SPDX-License-Identifier: MIT AND WTFPL

use pandora_api_derive::PandoraRequest;
use serde::{Deserialize, Serialize};

use crate::errors::Error;
use crate::json::{PandoraApiRequest, PandoraSession, ToSessionTokens};

/// **Unsupported!**
/// Undocumented method
/// [music.getSearchRecommendations()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct GetSearchRecommendationsUnsupported {}

/// **Unsupported!**
/// Undocumented method
/// [music.getTrack()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct GetTrackUnsupported {}

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
    /// Whether to include partial matches in the results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_near_matches: Option<bool>,
    /// Whether to include genre stations in the results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_genre_stations: Option<bool>,
}

impl<TS: ToString> From<&TS> for Search {
    fn from(search_text: &TS) -> Self {
        Self {
            search_text: search_text.to_string(),
            include_near_matches: None,
            include_genre_stations: None,
        }
    }
}

/// Convenience function to do a basic addSongBookmark call.
pub fn search<T: ToSessionTokens>(
    session: &PandoraSession<T>,
    search_text: &str,
) -> Result<SearchResponse, Error> {
    Search::from(&search_text).response(session)
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
    /// Whether any matches were found.
    pub near_matches_available: bool,
    /// Unknown
    pub explanation: String,
    /// Artists matching the search.
    pub artists: Vec<ArtistMatch>,
    /// Genre stations matching the search.
    #[serde(default)]
    pub genre_stations: Vec<GenreMatch>,
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
    use crate::json::{tests::session_login, Partner};

    #[test]
    fn search_test() {
        let partner = Partner::default();
        let session = session_login(&partner).expect("Failed initializing login session");

        let _search_response = search(&session, "INXS").expect("Failed completing search request");
        let mut search = Search::from(&"Alternative");
        search.include_genre_stations = Some(true);
        let _search_response: SearchResponse = search
            .response(&session)
            .expect("Failed completing search request");
    }
}
