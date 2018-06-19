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

    errors {
        InvalidImageSize(expected_w: u32, expected_h: u32) {
            description("Invalid image size")
            display("Size {} x {} is expected but found another", expected_w, expected_h)
        }
    }
}
