/*!
Track support methods.
*/
// SPDX-License-Identifier: MIT AND WTFPL

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
pub fn explain_track() {}

/// **Unsupported!**
/// Undocumented method
/// [track.trackStarted()](https://6xq.net/pandora-apidoc/json/methods/)
pub fn track_started() {
    unimplemented!();
}
