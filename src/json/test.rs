/*!
Test methods.
*/
// SPDX-License-Identifier: MIT AND WTFPL

use pandora_api_derive::PandoraRequest;
use serde::{Deserialize, Serialize};

use crate::errors::Error;
use crate::json::{PandoraApiRequest, PandoraSession};

/// Check whether Pandora is available in the connecting client’s country,
/// based on geoip database.  This is not strictly required since Partner
/// login enforces this restriction. The request has no parameters.
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[serde(rename_all = "camelCase")]
pub struct CheckLicensing {}

impl CheckLicensing {
    /// Create a new CheckLicensing.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for CheckLicensing {
    fn default() -> Self {
        Self {}
    }
}

///
/// | Name    | Type  |   Description |
/// | isAllowed |   bool     | |
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckLicensingResponse {
    /// Whether the Pandora service is available to the requesting client.
    pub is_allowed: bool,
}

/// Convenience function to check geographic licensing restrictions.
pub async fn check_licensing(
    session: &mut PandoraSession,
) -> Result<CheckLicensingResponse, Error> {
    CheckLicensing::default().response(session).await
}

/// **Unsupported!**
/// Undocumented method
/// [test.echo()](https://6xq.net/pandora-apidoc/json/methods/)
pub struct EchoUnsupported {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::{tests::session_login, Partner};

    #[async_std::test]
    async fn licensing_check_test() {
        let partner = Partner::default();
        let mut session = session_login(&partner).await.expect("Failed initializing login session");

        let check_licensing_response =
            check_licensing(&mut session).await.expect("Error making test.checkLicensing request");
        println!("test.checkLicensing() => {:?}", check_licensing_response);
    }
}
