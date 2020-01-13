/*!
Test methods.
*/
// SPDX-License-Identifier: MIT AND WTFPL

use pandora_api_derive::PandoraRequest;
use serde::{Deserialize, Serialize};

use crate::errors::Error;
use crate::json::PandoraApiRequest;

/// Check whether Pandora is available in the connecting client’s country,
/// based on geoip database.  This is not strictly required since Partner
/// login enforces this restriction. The request has no parameters.
#[derive(Debug, Clone, Serialize, PandoraRequest)]
#[serde(rename_all = "camelCase")]
pub struct CheckLicensing {}

///
/// | Name    | Type  |   Description |
/// | isAllowed |   bool     | |
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckLicensingResponse {
    /// Whether the Pandora service is available to the requesting client.
    pub is_allowed: bool,
}

/// **Unsupported!**
/// Undocumented method
/// [test.echo()](https://6xq.net/pandora-apidoc/json/methods/)
pub fn echo() {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::{PandoraRequestBuilder, Partner, ToEndpoint};

    #[test]
    fn licensing_check_test() {
        let client = reqwest::blocking::Client::new();
        let partner = Partner::default();
        let pandora_request_builder = PandoraRequestBuilder::with_session(
            Some(client),
            partner.to_endpoint(),
            partner.to_session_data(),
        );

        let licensing_check = CheckLicensing {};
        let response: CheckLicensingResponse = licensing_check
            .response(&pandora_request_builder)
            .expect("Error making test.checkLicensing request");
        println!("test.checkLicensing() => {:?}", response);
    }
}
