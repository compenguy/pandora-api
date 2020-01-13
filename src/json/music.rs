/*!
Music support methods.
*/
// SPDX-License-Identifier: MIT AND WTFPL

use pandora_api_derive::PandoraRequest;
use serde::{Deserialize, Serialize};

use crate::errors::Error;
use crate::json::PandoraApiRequest;

/// **Unsupported!**
/// Undocumented method
/// [music.getSearchRecommendations()](https://6xq.net/pandora-apidoc/json/methods/)
pub fn get_search_recommendations() {
    unimplemented!();
}

/// **Unsupported!**
/// Undocumented method
/// [music.getTrack()](https://6xq.net/pandora-apidoc/json/methods/)
pub fn get_track() {
    unimplemented!();
}

/// **Unsupported!**
/// Undocumented method
/// [music.publishSongShare()](https://6xq.net/pandora-apidoc/json/methods/)
pub fn publish_song_share() {
    unimplemented!();
}

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
#[serde(rename_all = "camelCase")]
pub struct Search {
    /// The text to search for in artist names or track titles.
    pub search_text: String,
}

impl From<String> for Search {
    fn from(search_text: String) -> Self {
        Self { search_text }
    }
}

impl From<&str> for Search {
    fn from(search_text: &str) -> Self {
        Self {
            search_text: search_text.to_string(),
        }
    }
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
pub fn share_music() {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::{tests::session_login, PandoraRequestBuilder, Partner, ToEndpoint};

    #[test]
    fn search_test() {
        let partner = Partner::default();
        let mut pandora_request_builder = PandoraRequestBuilder::with_session(
            None,
            partner.to_endpoint(),
            partner.to_session_data(),
        );
        session_login(&partner, &mut pandora_request_builder)
            .expect("Failed while initiating login session");

        let search_response: SearchResponse = Search::from("INXS")
            .response(&pandora_request_builder)
            .expect("Failed completing search request");
        println!("SearchResponse: {:?}", search_response);
        panic!("At the disco!");
    }
}
