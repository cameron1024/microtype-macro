microtype_macro::microtype! {
    out secret String {
        Token
    }
    secret String {
        Password
    }
}


fn main() {
    use microtype_core::SecretMicrotype;

    let token = Token::new("asdf".into());
    assert_serialize(token);
    assert_deserialize(token);

    let password = Password::new("asdf".into());
    assert_serialize(password);
}

fn assert_serialize<T: serde::Serialize>(_t: T) {}
fn assert_deserialize<T: serde::Serialize>(_t: T) {}
