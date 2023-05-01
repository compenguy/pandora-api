/*!
Test methods.
*/
// SPDX-License-Identifier: MIT AND WTFPL

use pandora_api_derive::PandoraJsonRequest;
use serde::{Deserialize, Serialize};

use crate::errors::Error;
use crate::json::{PandoraJsonApiRequest, PandoraSession};

/// Check whether Pandora is available in the connecting client’s country,
/// based on geoip database.  This is not strictly required since Partner
/// login enforces this restriction. The request has no parameters.
#[derive(Debug, Clone, Default, Serialize, PandoraJsonRequest)]
#[serde(rename_all = "camelCase")]
pub struct CheckLicensing {}

impl CheckLicensing {
    /// Create a new CheckLicensing.
    pub fn new() -> Self {
        Self::default()
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

    #[tokio::test]
    async fn licensing_check_test() {
        /*
        flexi_logger::Logger::try_with_str("info, pandora_api=debug")
            .expect("Failed to set logging configuration")
            .start()
            .expect("Failed to start logger");
        */

        let partner = Partner::default();
        let mut session = session_login(&partner)
            .await
            .expect("Failed initializing login session");

        let check_licensing_response = check_licensing(&mut session)
            .await
            .expect("Error making test.checkLicensing request");
        log::debug!("test.checkLicensing() => {:?}", check_licensing_response);
    }
}
