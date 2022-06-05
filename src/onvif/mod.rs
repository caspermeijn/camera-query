use url::Url;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("schema should be `http` or `https`")]
    InvalidSchema,
}

pub struct Device {
    pub url: Url,
}

impl Device {
    pub fn from(url: Url) -> Result<Self, ParseError> {
        if url.cannot_be_a_base() {
            return Err(ParseError::InvalidSchema);
        }
        if url.scheme() != "http" && url.scheme() != "https" {
            return Err(ParseError::InvalidSchema);
        }
        Ok(Self { url })
    }

    pub fn get_devicemgmt_url(&self) -> Url {
        let service_path = "onvif/device_service";
        self.url.join(service_path).unwrap()
    }
}
