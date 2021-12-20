/*!
Bookmark support messages.

Users can bookmark artists or songs.
*/
// SPDX-License-Identifier: MIT AND WTFPL
use pandora_api_derive::PandoraRequest;
use serde::{Deserialize, Serialize};

use crate::errors::Error;
use crate::json::{PandoraApiRequest, PandoraSession, Timestamp};

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
#[pandora_request(encrypted = true)]
#[serde(rename_all = "camelCase")]
pub struct AddArtistBookmark {
    /// The unique id (token) for this track.
    pub track_token: String,
}

impl<TS: ToString> From<&TS> for AddArtistBookmark {
    fn from(track_token: &TS) -> Self {
        Self {
            track_token: track_token.to_string(),
        }
    }
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

/// Convenience function to do a basic addArtistBookmark call.
pub async fn add_artist_bookmark(
    session: &mut PandoraSession,
    track_token: &str,
) -> Result<AddArtistBookmarkResponse, Error> {
    AddArtistBookmark::from(&track_token)
        .response(session)
        .await
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
#[pandora_request(encrypted = true)]
#[serde(rename_all = "camelCase")]
pub struct AddSongBookmark {
    /// The unique id (token) for this track.
    pub track_token: String,
}

impl<TS: ToString> From<&TS> for AddSongBookmark {
    fn from(track_token: &TS) -> Self {
        Self {
            track_token: track_token.to_string(),
        }
    }
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

/// Convenience function to do a basic addSongBookmark call.
pub async fn add_song_bookmark(
    session: &mut PandoraSession,
    track_token: &str,
) -> Result<AddSongBookmarkResponse, Error> {
    AddSongBookmark::from(&track_token).response(session).await
}

/// Bookmarks can be deleted
///
/// | Name |   Type |   Description |
/// | bookmarkToken | string |  |
/// ``` json
/// {
///     "bookmarkToken": "3738252050522320365",
///     "userAuthToken": "XXX",
///     "syncTime": 1404910760
/// }
/// ```
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[pandora_request(encrypted = true)]
#[serde(rename_all = "camelCase")]
pub struct DeleteArtistBookmark {
    /// The unique id (token) for the bookmark submission that should be deleted.
    pub bookmark_token: String,
}

impl<TS: ToString> From<&TS> for DeleteArtistBookmark {
    fn from(bookmark_token: &TS) -> Self {
        Self {
            bookmark_token: bookmark_token.to_string(),
        }
    }
}

/// This method does not return data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteArtistBookmarkResponse {}

/// Convenience function to do a basic deleteArtistBookmark call.
pub async fn delete_artist_bookmark(
    session: &mut PandoraSession,
    bookmark_token: &str,
) -> Result<DeleteArtistBookmarkResponse, Error> {
    DeleteArtistBookmark::from(&bookmark_token)
        .response(session)
        .await
}

/// Bookmarks can be deleted
///
/// | Name |   Type |   Description |
/// | bookmarkToken | string |  |
/// ``` json
/// {
///     "bookmarkToken": "3738252050522320365",
///     "userAuthToken": "XXX",
///     "syncTime": 1404910760
/// }
/// ```
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[pandora_request(encrypted = true)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSongBookmark {
    /// The unique id (token) for the bookmark submission that should be deleted.
    pub bookmark_token: String,
}

impl<TS: ToString> From<&TS> for DeleteSongBookmark {
    fn from(bookmark_token: &TS) -> Self {
        Self {
            bookmark_token: bookmark_token.to_string(),
        }
    }
}

/// This method does not return data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSongBookmarkResponse {}

/// Convenience function to do a basic deleteSongBookmark call.
pub async fn delete_song_bookmark(
    session: &mut PandoraSession,
    bookmark_token: &str,
) -> Result<DeleteSongBookmarkResponse, Error> {
    DeleteSongBookmark::from(&bookmark_token)
        .response(session)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::{
        station::get_playlist, tests::session_login, user::get_bookmarks, user::get_station_list,
        Partner,
    };

    #[async_std::test]
    async fn bookmark_test() {
        let partner = Partner::default();
        let mut session = session_login(&partner).await.expect("Failed initializing login session");

        if let Some(station) = get_station_list(&mut session).await
            .expect("Failed getting station list to look up a track to bookmark")
            .stations
            .first()
        {
            if let Some(track) = get_playlist(&mut session, &station.station_token).await
                .expect("Failed completing request for playlist")
                .items
                .iter()
                .flat_map(|p| p.get_track())
                .next()
            {
                let artist_bookmark = add_artist_bookmark(&mut session, &track.track_token).await
                    .expect("Failed submitting artist bookmark creation request");
                println!("Bookmark creation result: {:?}", artist_bookmark);

            /* TODO: song bookmark deletion doesn't seem to work yet, so lets
             * not go creating more with each run.
            let song_bookmark = add_song_bookmark(&mut session, &track.track_token)
                .expect("Failed submitting song bookmark creation request");
            println!("Bookmark creation result: {:?}", song_bookmark);
            */
            } else {
                panic!("Playlist request returned no bookmarkable results.");
            }
        } else {
            panic!("Station list request returned no results, so no bookmarkable content.");
        }

        let user_bookmarks =
            get_bookmarks(&mut session).await.expect("Failed submitting request for user bookmarks");

        for artist_bookmark in user_bookmarks.artists {
            let _del_bookmark =
                delete_artist_bookmark(&mut session, &artist_bookmark.bookmark_token).await
                    .expect("Failed submitting artist bookmark deletion request");
        }

        /* TODO: song bookmark deletion is borken, can't figure out why
        for song_bookmark in user_bookmarks.songs {
            let _del_bookmark = delete_artist_bookmark(&mut session, &song_bookmark.bookmark_token)
                .expect("Failed submitting song bookmark deletion request");
        }
        */
    }
}
