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

use clap::Parser;
use std::process::exit;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Network address of ONVIF camera, eg. "http://192.0.2.123
    uri: url::Url,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let device = camera_query::onvif::Device::from(args.uri);
    match device {
        Err(error) => {
            eprintln!("Failed to parse uri: {}", error);
            exit(1);
        }
        Ok(device) => {
            println!("Inquiring {}...", device.url);
            let result = camera_query::query::query(&device).await;

            println!();
            println!("{}", result);
        }
    }
}
