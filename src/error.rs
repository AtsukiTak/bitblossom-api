error_chain! {
    types {
        Error, ErrorKind, ResultExt;
    }

    foreign_links {
        Hyper(::hyper::error::Error);
        SerdeJson(::serde_json::error::Error);
        Image(::image::ImageError);
        Uri(::http::uri::InvalidUri);
        Timer(::tokio::timer::Error);
        Base64Decode(::base64::DecodeError);
    }
}
