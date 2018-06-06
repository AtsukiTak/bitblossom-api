error_chain! {
    types {
        Error, ErrorKind, ResultExt;
    }

    foreign_links {
        Hyper(::hyper::error::Error);
        SerdeJson(::serde_json::error::Error);
        Image(::image::ImageError);
    }
}
