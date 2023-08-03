use url::{Url, Host};


fn main() {
    let url_str = "https://github.com:8080/rust-lang/rust/issues";
    let url = Url::parse(url_str).unwrap();

    assert_eq!(url.host_str(), Some("github.com"));
    assert_eq!(url.port(), Some(8080));
}