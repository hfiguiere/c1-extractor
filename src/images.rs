/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use std::fmt;

use super::CoId;

#[derive(Debug, Default)]
pub enum ImageFormat {
    #[default]
    Unknown,
    Jpeg,
    Raw,
    Movie,
}

impl From<&str> for ImageFormat {
    fn from(val: &str) -> Self {
        match val {
            "JPEG" => ImageFormat::Jpeg,
            "RAW" => ImageFormat::Raw,
            "MOVIE" => ImageFormat::Movie,
            _ => ImageFormat::Unknown,
        }
    }
}

impl fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ImageFormat::Jpeg => f.pad("JPEG"),
            ImageFormat::Raw => f.pad("RAW"),
            ImageFormat::Movie => f.pad("MOVIE"),
            _ => f.pad("UNKNOWN"),
        }
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
            let mut rows = stmt.query([&entity]).unwrap();
            while let Ok(Some(row)) = rows.next() {
                images.push(Image {
                    id: row.get(0).unwrap(),
                    uuid: row.get(1).unwrap(),
                    folder: row.get(2).unwrap(),
                    display_name: row.get(3).unwrap(),
                    file_name: row.get(4).unwrap(),
                    class: row.get(5).unwrap(),
                    format: ImageFormat::from(row.get::<usize, String>(6).unwrap().as_str()),
                    gps_alt: row.get::<usize, f64>(7).ok(),
                    gps_lat: row.get::<usize, f64>(8).ok(),
                    gps_long: row.get::<usize, f64>(9).ok()
                });
            }
        }

        images
    }
}
