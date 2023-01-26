/*
 * The contents of this file are subject to the terms of the
 * Common Development and Distribution License, Version 1.1 only
 * (the "License").  You may not use this file except in compliance
 * with the License.
 *
 * See the file LICENSE in this distribution for details.
 * A copy of the CDDL is also available via the Internet at
 * https://spdx.org/licenses/CDDL-1.1.html
 *
 * When distributing Covered Code, include this CDDL HEADER in each
 * file and include the contents of the LICENSE file from this
 * distribution.
 */

use chrono::{DateTime, Timelike, Utc};
use chrono_tz::Tz;
use chrono_tz::Europe::Berlin;
use mastodon_async::helpers::toml;
use mastodon_async::prelude::*;
use mastodon_async::{Language, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Mastodon-Login:
    let mastodon = if let Ok(data) = toml::from_file("kuckuck.toml") {
        Mastodon::from(data)
    } else {
        // Probably WILL fail. Maybe it will help people read the README...
        Mastodon::from(Data::default())
    };

    // Aktuelle Stunde:
    let stunde: DateTime<Tz> = Utc::now().with_timezone(&Berlin);
    let (_, stunde_12) = stunde.hour12();
    let stunde_usize = stunde_12 as usize;

    let kuckuck_text = "Kuckuck! ".repeat(stunde_usize);

    // Und ab damit:
    let status = StatusBuilder::new()
        .status(kuckuck_text)
        .visibility(Visibility::Public)
        .language(Language::Deu)
        .build()?;

    mastodon.new_status(status).await?;

    Ok(())
}
