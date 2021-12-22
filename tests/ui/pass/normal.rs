use microtype_core::Microtype;

microtype_macro::microtype! {
    String {
        Email
    }
}


fn main() {
    let email = Email::new("hello".into());
    assert_eq!(email.into_inner(), "hello");
}
