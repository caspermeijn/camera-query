use onvif::{schema, soap};

#[derive(Debug)]
pub struct Result {
    pub is_onvif_device: bool,
}

impl std::fmt::Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "is ONVIF device: {}", self.is_onvif_device)
    }
}

pub async fn query(device: &crate::onvif::Device) -> Result {
    Result {
        is_onvif_device: is_onvif_device(device).await,
    }
}

async fn is_onvif_device(device: &crate::onvif::Device) -> bool {
    let devicemgmt = soap::client::ClientBuilder::new(&device.get_devicemgmt_url())
        .build();
    let date = schema::devicemgmt::get_system_date_and_time(&devicemgmt, &Default::default()).await;

    date.is_ok()
}
