pub trait RawJsonValueExt {
    fn invalid<E>(self, error: E) -> nojson::JsonParseError
    where
        E: Into<Box<dyn Send + Sync + std::error::Error>>;
}

impl<'text, 'json> RawJsonValueExt for nojson::RawJsonValue<'text, 'json> {
    fn invalid<E>(self, error: E) -> nojson::JsonParseError
    where
        E: Into<Box<dyn Send + Sync + std::error::Error>>,
    {
        nojson::JsonParseError::invalid_value(self, error)
    }
}
