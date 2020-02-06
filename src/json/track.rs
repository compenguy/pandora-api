/*!
Track support methods.
*/
// SPDX-License-Identifier: MIT AND WTFPL

use pandora_api_derive::PandoraRequest;
use serde::{Deserialize, Serialize};

use crate::errors::Error;
use crate::json::{PandoraApiRequest, PandoraSession, ToSessionTokens};

/// Get (incomplete) list of attributes assigned to song by Music Genome Project.
///
/// | Name | Type | Description |
/// | trackToken | string | See Retrieve playlist |
/// ``` json
/// {
///     "trackToken": "94f36e09e341780c2ee7ebbb3581a55c4f2066dbaa60f2ee253ede5bc407fbd3c4f6db7ed00f92312437e020e0bf0e05d2924742c2ccece2",
///     "userAuthToken": "XXX",
///     "syncTime": 1336675993
/// }
/// ```
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[serde(rename_all = "camelCase")]
pub struct ExplainTrack {
    /// The token associated with the track for which an explanation is being requested.
    pub track_token: String,
}

impl<TS: ToString> From<&TS> for ExplainTrack {
    fn from(track_token: &TS) -> Self {
        Self {
            track_token: track_token.to_string(),
        }
    }
}

/// The request returns a list of attributes. Note that the last item is not an actual attribute.
///
/// | Name | Type | Description |
/// | explanations | array |  |
/// ``` json
/// {
///     "stat": "ok",
///     "result": {
///         "explanations": [{
///             "focusTraitName": "trance roots",
///             "focusTraitId": "F7524"
///         },
///         {
///             "focusTraitName": "many other similarities identified in the Music Genome Project",
///             "focusTraitId": "F4797"
///         }]
///     }
/// }
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplainTrackResponse {
    /// A list of explanations for why the track was chosen.
    explanations: Vec<Explanation>,
}

/// Describes traits of a track that would explain why it's recommended.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Explanation {
    /// Text description of the audio trait for which the track was chosen.
    pub focus_trait_name: String,
    /// A token or identifier associated with the audio trait.
    pub focus_trait_id: String,
}

/// Convenience function to do a basic explainTrack call.
pub fn explain_track<T: ToSessionTokens>(
    session: &PandoraSession<T>,
    track_token: &str,
) -> Result<ExplainTrackResponse, Error> {
    ExplainTrack::from(&track_token).response(session)
}

/// **Unsupported!**
/// Undocumented method
/// [track.trackStarted()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct TrackStartedUnsupported {}
