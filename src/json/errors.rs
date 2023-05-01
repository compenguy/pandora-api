/*!
Error codes that can be returned by the Pandora API.
*/
// SPDX-License-Identifier: MIT AND WTFPL

/// https://6xq.net/pandora-apidoc/json/errorcodes/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JsonErrorKind {
    /// Code 0 - Internal error. It can denote that your account has been temporarily blocked due to having
    /// too frequent station.getPlaylist calls.
    InternalError,
    /// Code 1 - MAINTENANCE_MODE
    MaintenanceMode,
    /// Code 2 - URL_PARAM_MISSING_METHOD
    UrlParamMissingMethod,
    /// Code 3 - URL_PARAM_MISSING_AUTH_TOKEN
    UrlParamMissingAuthToken,
    /// Code 4 - URL_PARAM_MISSING_PARTNER_ID
    UrlParamMissingPartnerId,
    /// Code 5 - URL_PARAM_MISSING_USER_ID
    UrlParamMissingUserId,
    /// Code 6 - SECURE_PROTOCOL_REQUIRED
    SecureProtocolRequired,
    /// Code 7 - CERTIFICATE_REQUIRED
    CertificateRequired,
    /// Code 8 - PARAMETER_TYPE_MISMATCH
    ParameterTypeMismatch,
    /// Code 9 - PARAMETER_MISSING. Usually occurs when one or more required parameters are missing
    /// for the method called.
    ParameterMissing,
    /// Code 10 - PARAMETER_VALUE_INVALID
    ParameterValueInvalid,
    /// Code 11 - API_VERSION_NOT_SUPPORTED
    ApiVersionNotSupported,
    /// Code 12 - LICENSING_RESTRICTIONS. Pandora not available in this country.
    LicensingRestrictions,
    /// Code 13 - INSUFFICIENT_CONNECTIVITY. Bad sync time?
    InsufficientConnectivity,
    /// Code 14 - Unknown method name?
    UnknownMethodName,
    /// Code 15 - Wrong protocol (http/https)?
    WrongProtocol,
    /// Code 1000 - READ_ONLY_MODE
    ReadOnlyMode,
    /// Code 1001 - INVALID_AUTH_TOKEN. Occurs once a user auth token expires.
    InvalidAuthToken,
    /// Code 1002 - INVALID_PARTNER_LOGIN. auth.partnerLogin auth.userLogin. Can also occur for a
    /// user login.
    InvalidPartnerLogin,
    /// Code 1003 - LISTENER_NOT_AUTHORIZED. station.getPlaylist - Pandora One Subscription or
    /// Trial Expired. Possibly account suspended?
    ListenerNotAuthorized,
    /// Code 1004 - USER_NOT_AUTHORIZED. User not authorized to perform action. Is your station
    /// token correct?
    UserNotAuthorized,
    /// Code 1005 - MAX_STATIONS_REACHED. Station limit reached.
    MaxStationsReached,
    /// Code 1006 - STATION_DOES_NOT_EXIST. Station does not exist.
    StationDoesNotExist,
    /// Code 1007 - COMPLIMENTARY_PERIOD_ALREADY_IN_USE
    ComplimentaryPeriodAlreadyInUse,
    /// Code 1008 - CALL_NOT_ALLOWED. station.addFeedback - Returned when attempting to add
    /// feedback to shared station.
    CallNotAllowed,
    /// Code 1009 - DEVICE_NOT_FOUND
    DeviceNotFound,
    /// Code 1010 - PARTNER_NOT_AUTHORIZED
    PartnerNotAuthorized,
    /// Code 1011 - INVALID_USERNAME
    InvalidUsername,
    /// Code 1012 - INVALID_PASSWORD
    InvalidPassword,
    /// Code 1013 - USERNAME_ALREADY_EXISTS
    UsernameAlreadyExists,
    /// Code 1014 - DEVICE_ALREADY_ASSOCIATED_TO_ACCOUNT
    DeviceAlreadyAssociatedToAccount,
    /// Code 1015 - UPGRADE_DEVICE_MODEL_INVALID
    UpgradeDeviceModelInvalid,
    /// Code 1018 - EXPLICIT_PIN_INCORRECT
    ExplicitPinIncorrect,
    /// Code 1020 - EXPLICIT_PIN_MALFORMED
    ExplicitPinMalformed,
    /// Code 1023 - DEVICE_MODEL_INVALID
    DeviceModelInvalid,
    /// Code 1024 - ZIP_CODE_INVALID
    ZipCodeInvalid,
    /// Code 1025 - BIRTH_YEAR_INVALID
    BirthYearInvalid,
    /// Code 1026 - BIRTH_YEAR_TOO_YOUNG
    BirthYearTooYoung,
    /// Code 1027 - INVALID_COUNTRY_CODE
    InvalidCountryCode,
    /// Code 1027 - INVALID_GENDER
    InvalidGender,
    /// Code 1034 - DEVICE_DISABLED
    DeviceDisabled,
    /// Code 1035 - DAILY_TRIAL_LIMIT_REACHED
    DailyTrialLimitReached,
    /// Code 1036 - INVALID_SPONSOR
    InvalidSponsor,
    /// Code 1037 - USER_ALREADY_USED_TRIAL
    UserAlreadyUsedTrial,
    /// Code 1039 - PLAYLIST_EXCEEDED. Too many requests for a new playlist.
    PlaylistExceeded,
    /// Undocumented error code
    UnknownErrorCode(u32),
    /// No error code provided
    UnknownErrorMessage,
}

impl From<u32> for JsonErrorKind {
    /// Create a JsonError from an error code.
    fn from(code: u32) -> Self {
        match code {
            0 => JsonErrorKind::InternalError,
            1 => JsonErrorKind::MaintenanceMode,
            2 => JsonErrorKind::UrlParamMissingMethod,
            3 => JsonErrorKind::UrlParamMissingAuthToken,
            4 => JsonErrorKind::UrlParamMissingPartnerId,
            5 => JsonErrorKind::UrlParamMissingUserId,
            6 => JsonErrorKind::SecureProtocolRequired,
            7 => JsonErrorKind::CertificateRequired,
            8 => JsonErrorKind::ParameterTypeMismatch,
            9 => JsonErrorKind::ParameterMissing,
            10 => JsonErrorKind::ParameterValueInvalid,
            11 => JsonErrorKind::ApiVersionNotSupported,
            12 => JsonErrorKind::LicensingRestrictions,
            13 => JsonErrorKind::InsufficientConnectivity,
            14 => JsonErrorKind::UnknownMethodName,
            15 => JsonErrorKind::WrongProtocol,
            1000 => JsonErrorKind::ReadOnlyMode,
            1001 => JsonErrorKind::InvalidAuthToken,
            1002 => JsonErrorKind::InvalidPartnerLogin,
            1003 => JsonErrorKind::ListenerNotAuthorized,
            1004 => JsonErrorKind::UserNotAuthorized,
            1005 => JsonErrorKind::MaxStationsReached,
            1006 => JsonErrorKind::StationDoesNotExist,
            1007 => JsonErrorKind::ComplimentaryPeriodAlreadyInUse,
            1008 => JsonErrorKind::CallNotAllowed,
            1009 => JsonErrorKind::DeviceNotFound,
            1010 => JsonErrorKind::PartnerNotAuthorized,
            1011 => JsonErrorKind::InvalidUsername,
            1012 => JsonErrorKind::InvalidPassword,
            1013 => JsonErrorKind::UsernameAlreadyExists,
            1014 => JsonErrorKind::DeviceAlreadyAssociatedToAccount,
            1015 => JsonErrorKind::UpgradeDeviceModelInvalid,
            1018 => JsonErrorKind::ExplicitPinIncorrect,
            1020 => JsonErrorKind::ExplicitPinMalformed,
            1023 => JsonErrorKind::DeviceModelInvalid,
            1024 => JsonErrorKind::ZipCodeInvalid,
            1025 => JsonErrorKind::BirthYearInvalid,
            1026 => JsonErrorKind::BirthYearTooYoung,
            /* TODO: these two error codes collide - verify value */
            1027 => JsonErrorKind::InvalidCountryCode,
            //1027 => JsonErrorKind::InvalidGender,
            1034 => JsonErrorKind::DeviceDisabled,
            1035 => JsonErrorKind::DailyTrialLimitReached,
            1036 => JsonErrorKind::InvalidSponsor,
            1037 => JsonErrorKind::UserAlreadyUsedTrial,
            1039 => JsonErrorKind::PlaylistExceeded,
            x => JsonErrorKind::UnknownErrorCode(x),
        }
    }
}

/*
impl From<String> for JsonErrorKind {
    /// Create an error message when there's no error code, only a text description.
    fn from(_msg: String) -> Self {
        JsonErrorKind::UnknownErrorMessage
    }
}
*/

impl std::fmt::Display for JsonErrorKind {
    /// Format this type for display
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // Seen in the wild, accompanied with a message "Service method not found"
            JsonErrorKind::InternalError => write!(f, "Internal"),
            JsonErrorKind::MaintenanceMode => write!(f, "Maintenance Mode"),
            JsonErrorKind::UrlParamMissingMethod => write!(f, "Url Param Missing Method"),
            JsonErrorKind::UrlParamMissingAuthToken => write!(f, "Url Param Missing Auth Token"),
            JsonErrorKind::UrlParamMissingPartnerId => write!(f, "Url Param Missing Partner ID"),
            JsonErrorKind::UrlParamMissingUserId => write!(f, "Url Param Missing User ID"),
            JsonErrorKind::SecureProtocolRequired => write!(f, "Secure Protocol Required"),
            JsonErrorKind::CertificateRequired => write!(f, "Certificate Required"),
            JsonErrorKind::ParameterTypeMismatch => write!(f, "Parameter Type Mismatch"),
            JsonErrorKind::ParameterMissing => write!(f, "Parameter Missing."),
            JsonErrorKind::ParameterValueInvalid => write!(f, "Parameter Value Invalid"),
            JsonErrorKind::ApiVersionNotSupported => write!(f, "API Version Not Supported"),
            JsonErrorKind::LicensingRestrictions => write!(f, "Licensing Restriction"),
            JsonErrorKind::InsufficientConnectivity => write!(f, "Insufficient Connectivity"),
            JsonErrorKind::UnknownMethodName => write!(f, "Unknown Method Name"),
            JsonErrorKind::WrongProtocol => write!(f, "Incorrect Protocol"),
            JsonErrorKind::ReadOnlyMode => write!(f, "Read Only Mode"),
            JsonErrorKind::InvalidAuthToken => write!(f, "Invalid Auth Token"),
            JsonErrorKind::InvalidPartnerLogin => write!(f, "Invalid Partner Login"),
            JsonErrorKind::ListenerNotAuthorized => write!(f, "Listener Not Authorized"),
            JsonErrorKind::UserNotAuthorized => write!(f, "User Not Authorized"),
            JsonErrorKind::MaxStationsReached => write!(f, "Max Stations Reached"),
            JsonErrorKind::StationDoesNotExist => write!(f, "Station Does Not Exist"),
            JsonErrorKind::ComplimentaryPeriodAlreadyInUse => {
                write!(f, "Complimentary Period Already In Use")
            }
            JsonErrorKind::CallNotAllowed => write!(f, "Call Not Allowed"),
            JsonErrorKind::DeviceNotFound => write!(f, "Device Not Found"),
            JsonErrorKind::PartnerNotAuthorized => write!(f, "Partner Not Authorized"),
            JsonErrorKind::InvalidUsername => write!(f, "Invalid Username"),
            JsonErrorKind::InvalidPassword => write!(f, "Invalid Password"),
            JsonErrorKind::UsernameAlreadyExists => write!(f, "Username Already Exists"),
            JsonErrorKind::DeviceAlreadyAssociatedToAccount => {
                write!(f, "Device Already Associated to Account")
            }
            JsonErrorKind::UpgradeDeviceModelInvalid => write!(f, "Upgrade Device Model Invalid"),
            JsonErrorKind::ExplicitPinIncorrect => write!(f, "Explicit Pin Incorrect"),
            JsonErrorKind::ExplicitPinMalformed => write!(f, "Explicit Pin Malformed"),
            JsonErrorKind::DeviceModelInvalid => write!(f, "Device Model Invalid"),
            JsonErrorKind::ZipCodeInvalid => write!(f, "Zip Code Invalid"),
            JsonErrorKind::BirthYearInvalid => write!(f, "Birth Year Invalid"),
            JsonErrorKind::BirthYearTooYoung => write!(f, "Birth Year Too Young"),
            JsonErrorKind::InvalidCountryCode => write!(f, "Invalid Country Code"),
            JsonErrorKind::InvalidGender => write!(f, "Invalid Gender"),
            JsonErrorKind::DeviceDisabled => write!(f, "Device Disabled"),
            JsonErrorKind::DailyTrialLimitReached => write!(f, "Daily Trial Limit Reached"),
            JsonErrorKind::InvalidSponsor => write!(f, "Invalid Sponsor"),
            JsonErrorKind::UserAlreadyUsedTrial => write!(f, "User Already Used Trial"),
            JsonErrorKind::PlaylistExceeded => write!(
                f,
                "Playlist Exceeded. Too many requests for a new playlist."
            ),
            JsonErrorKind::UnknownErrorCode(x) => write!(f, "Unrecognized Error Code ({x})"),
            JsonErrorKind::UnknownErrorMessage => write!(f, "Missing Error Code."),
        }
    }
}

/// Pandora JSON API call error description
#[derive(Debug, Clone, PartialEq)]
pub struct JsonError {
    pub(crate) kind: JsonErrorKind,
    pub(crate) message: Option<String>,
}

impl JsonError {
    /// Initialize a JsonError from some combination of code and message.
    pub fn new(code: Option<u32>, message: Option<String>) -> Self {
        let kind = match code {
            Some(code) => JsonErrorKind::from(code),
            None => JsonErrorKind::UnknownErrorMessage,
        };
        JsonError { kind, message }
    }

    /// Return what kind of error this is.
    pub fn kind(&self) -> JsonErrorKind {
        self.kind
    }
}

impl std::error::Error for JsonError {
    /// Get the source error, if any, for this error.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for JsonError {
    /// Format this error for display
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Pandora API Call Error ({} Error)", self.kind)?;
        if let Some(msg) = &self.message {
            write!(f, ": {msg}")?;
        }
        Ok(())
    }
}
