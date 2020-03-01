use pandora_api::json::{auth, test, user, Partner};

#[test]
fn basic_session_test() {
    let partner = Partner::default();
    let mut session = partner.init_session();

    // This call doesn't even require partner authentication
    let licensing = test::check_licensing(&mut session)
        .expect("Failed while checking geographic licensing restrictions");
    assert!(licensing.is_allowed);

    // Do partner authentication
    let _partner_login = partner
        .login(&mut session)
        .expect("Failed during partner auth API request");

    let test_username_raw = include_str!("../test_username.txt");
    let test_username = test_username_raw.trim();
    let test_password_raw = include_str!("../test_password.txt");
    let test_password = test_password_raw.trim();

    // Check username is valid
    let user_validation = user::validate_username(&mut session, test_username)
        .expect("Failed while validating username");
    assert!(user_validation.is_valid);

    // Check that an non-email username is invalid
    let user_invalidation = user::validate_username(&mut session, "VGhlcmUgb25jZSB")
        .expect("Failed while validating username");
    assert!(!user_invalidation.is_valid);

    // Check that an almost-certainly unused email username is valid and unique
    let user_invalidation = user::validate_username(&mut session, "VGhlcmUgb25jZSB@gmail.com")
        .expect("Failed while validating username");
    assert_eq!(user_invalidation.is_unique, Some(true));

    // Test login
    let _login_response = auth::user_login(&mut session, &test_username, &test_password)
        .expect("Failed while logging user in");

    // Get user usage info
    let _usage_info =
        user::get_usage_info(&mut session).expect("Failed while getting account usage information");

    // Check user subscription status
    let _can_subscribe = user::can_subscribe(&mut session)
        .expect("Failed while verifying account subscription status");

    let _change_settings = user::change_settings(&mut session, &test_username, &test_password)
        .expect("Failed while changing user settings");
    // TODO
}
