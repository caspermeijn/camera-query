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

use url::Url;

use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum ParseError {
    #[error("schema should be `http` or `https`")]
    InvalidSchema,
}

#[derive(Debug, Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    use crate::onvif::ParseError;

    #[test]
    fn parse_device() {
        let url = "http://192.0.2.123".parse().unwrap();
        let device = crate::onvif::Device::from(url);
        assert!(device.is_ok());

        let url = "onvif://192.0.2.123".parse().unwrap();
        let device = crate::onvif::Device::from(url);
        assert_eq!(device, Err(ParseError::InvalidSchema));

        let url = "data:text/plain,Stuff".parse().unwrap();
        let device = crate::onvif::Device::from(url);
        assert_eq!(device, Err(ParseError::InvalidSchema));
    }

    #[test]
    fn get_devicemgmt_url() {
        let url = "http://192.0.2.123".parse().unwrap();
        let device = crate::onvif::Device::from(url).unwrap();

        assert_eq!(device.get_devicemgmt_url(), "http://192.0.2.123/onvif/device_service".parse().unwrap());
    }
}
