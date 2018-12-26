/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use rusqlite;
use super::CoId;

#[derive(Debug)]
pub enum ImageFormat {
    Unknown,
    Jpeg,
    Raw,
    Movie
}

impl From<&str> for ImageFormat {
    fn from(val: &str) -> Self {
        match val {
            "JPEG" => ImageFormat::Jpeg,
            "RAW" => ImageFormat::Raw,
            "MOVIE" => ImageFormat::Movie,
            _ => ImageFormat::Unknown
        }
    }
}

impl Default for ImageFormat {
    fn default() -> Self {
        ImageFormat::Unknown
    }
}

#[derive(Default)]
pub struct Image {
    pub id: CoId,
    pub uuid: String,
    pub folder: CoId,
    pub class: i32,
    pub format: ImageFormat,
    pub display_name: String,
    pub file_name: String,
    pub gps_alt: Option<f64>,
    pub gps_lat: Option<f64>,
    pub gps_long: Option<f64>,
}


impl Image {

    pub fn load_objects(conn: &rusqlite::Connection, entity: CoId) -> Vec<Image> {
        let mut images: Vec<Image> = vec![];
        if let Ok(mut stmt) = conn.prepare("SELECT Z_PK, ZIMAGEUUID, ZIMAGELOCATION, ZDISPLAYNAME, ZIMAGEFILENAME, ZIMAGECLASSIFICATION, ZEXP_FORMAT, ZGPSALTITUDE, ZGPSLATITUDE, ZGPSLONGITUDE FROM ZIMAGE WHERE Z_ENT=?1") {
            let mut rows = stmt.query(&[&entity]).unwrap();
            while let Some(Ok(row)) = rows.next() {
                images.push(Image {
                    id: row.get(0),
                    uuid: row.get(1),
                    folder: row.get(2),
                    display_name: row.get(3),
                    file_name: row.get(4),
                    class: row.get(5),
                    format: ImageFormat::from(row.get::<usize, String>(6).as_str()),
                    gps_alt: row.get_checked(7).ok(),
                    gps_lat: row.get_checked(8).ok(),
                    gps_long: row.get_checked(9).ok()
                });
            }
        }

        images
    }
}
