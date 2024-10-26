use zed_extension_api::{self as zed, Result};

struct GrenExtension;

impl zed::Extension for GrenExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(GrenExtension);
