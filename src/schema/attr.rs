use crate::error::Error;

pub fn extract_access_value(attr: &str) -> anyhow::Result<Option<String>, Error> {
    match attr.to_ascii_uppercase().as_str() {
        "RO" => Ok(Some("read-only".to_owned())),
        "RW" | "RC" | "RS" | "WRC" | "WRS" | "WSRC" | "WCRS" | "W1C" | "W1S" | "W1T" | "W0C"
        | "W0S" | "W0T" | "W1SRC" | "W1CRS" | "W0SRC" | "W0CRS" => {
            Ok(Some("read-write".to_owned()))
        }
        "WO" | "WC" | "WS" | "WOC" | "WOS" => Ok(Some("write-only".to_owned())),
        "W1" | "WO1" => Ok(Some("writeOnce".to_owned())),
        _ => Err(Error::StringError(
            "cannot find access attribute".to_owned(),
        )),
    }
}

pub fn extract_modified_write_value(attr: &str) -> Result<Option<String>, Error> {
    match attr.to_ascii_uppercase().as_str() {
        "RO" | "RW" | "RC" | "RS" | "WO" | "W1" | "WO1" => Ok(None),
        "WRC" | "W1C" | "WCRS" | "W1CRS" => Ok(Some("oneToClear".to_owned())),
        "WRS" | "W1S" | "WSRC" | "W1SRC" => Ok(Some("oneToSet".to_owned())),
        "W1T" => Ok(Some("oneToToggle".to_owned())),
        "W0C" | "W0CRS" => Ok(Some("zeroToClear".to_owned())),
        "W0S" | "W0SRC" => Ok(Some("zeroToSet".to_owned())),
        "W0T" => Ok(Some("zeroToToggle".to_owned())),
        "WC" | "WOC" => Ok(Some("clear".to_owned())),
        "WS" | "WOS" => Ok(Some("set".to_owned())),
        _ => Err(Error::StringError(
            "cannot find access attribute".to_owned(),
        )),
    }
}

pub fn extract_read_action_value(attr: &str) -> Result<Option<String>, Error> {
    match attr.to_ascii_uppercase().as_str() {
        "RO" | "RW" | "WC" | "WS" | "W1C" | "W1S" | "W1T" | "W0C" | "W0S" | "W0T" | "WO"
        | "WOC" | "WOS" | "W1" | "WO1" => Ok(None),
        "RC" | "WRC" | "WSRC" | "W1SRC" | "W0SRC" => Ok(Some("clear".to_owned())),
        "RS" | "WRS" | "WCRS" | "W1CRS" | "W0CRS" => Ok(Some("set".to_owned())),
        _ => Err(Error::StringError(
            "cannot find access attribute".to_owned(),
        )),
    }
}
