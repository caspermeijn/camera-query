/* Copyright (C) 2022 Casper Meijn <casper@meijn.net>
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

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

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn is_onvif_device_success() {
        use onvif::schema::devicemgmt::GetSystemDateAndTimeResponse;
        use onvif::soap::soap;
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use wiremock::matchers::{method, path};

        // Start mock server
        let mock_server = MockServer::start().await;
        let device = crate::onvif::Device::from(mock_server.uri().parse().unwrap()).unwrap();

        // Prepare response
        let response = GetSystemDateAndTimeResponse::default();
        let response = soap(&yaserde::ser::to_string(&response).unwrap(), &None).unwrap();

        // Setup mock server
        Mock::given(method("POST"))
            .and(path("/onvif/device_service"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_string(response))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Query mock server
        let result = super::is_onvif_device(&device).await;
        assert_eq!(result, true)
    }

    #[tokio::test]
    async fn is_onvif_device_empty_response() {
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use wiremock::matchers::{method, path};

        // Start mock server
        let mock_server = MockServer::start().await;
        let device = crate::onvif::Device::from(mock_server.uri().parse().unwrap()).unwrap();

        // Setup mock server
        Mock::given(method("POST"))
            .and(path("/onvif/device_service"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Query mock server
        let result = super::is_onvif_device(&device).await;
        assert_eq!(result, false)
    }


    #[tokio::test]
    async fn is_onvif_device_timeout() {
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use wiremock::matchers::{method, path};
        use std::time::Duration;

        // Start mock server
        let mock_server = MockServer::start().await;
        let device = crate::onvif::Device::from(mock_server.uri().parse().unwrap()).unwrap();

        // Setup mock server
        Mock::given(method("POST"))
            .and(path("/onvif/device_service"))
            .respond_with(ResponseTemplate::new(200)
                .set_delay(Duration::from_secs(1000)))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Query mock server
        let result = super::is_onvif_device(&device).await;
        assert_eq!(result, false)
    }

    #[tokio::test]
    async fn is_onvif_device_invalid_addr() {
        let device = crate::onvif::Device::from("http://192.0.2.123".parse().unwrap()).unwrap();

        let result = super::is_onvif_device(&device).await;
        assert_eq!(result, false)
    }
}

