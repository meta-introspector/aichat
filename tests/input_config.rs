use aichat::config::{Config as GlobalConfig, Input, Role};

#[test]
fn test_input_from_str() {
    let config = GlobalConfig::default(); // Assuming GlobalConfig has a Default implementation
    let text = "Hello, world!";
    let role = Some(Role::default()); // Assuming Role has a Default implementation
    let authenticator = None;

    let input = Input::from_str(&config, text, role, authenticator);

    assert_eq!(input.text(), text);
}
