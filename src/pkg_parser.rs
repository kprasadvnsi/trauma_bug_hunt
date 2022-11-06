use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::data::Package;

pub fn parse_pkg_desc(pkg_desc_path: String) -> Package {
    let f = File::open(pkg_desc_path).expect("Unable to open file");
    let f = BufReader::new(f);
    let mut lines = f.lines();

    let mut filename = String::new();
    let mut name = String::new();
    let mut base = String::new();
    let mut version = String::new();
    let mut desc = String::new();
    let mut groups = String::new();
    let mut csize = String::new();
    let mut isize = String::new();
    let mut md5eum = String::new();
    let mut sha256sum = String::new();
    let mut pgpsig = String::new();
    let mut url = String::new();
    let mut license = String::new();
    let mut arch = String::new();
    let mut builddate = String::new();
    let mut packager = String::new();
    let mut replaces = String::new();

    while let Some(line) = lines.next() {
        let key = line.unwrap();
        if key.starts_with("%") && key.ends_with("%") {
            if let Some(next_line) = lines.next() {
                let value = next_line.unwrap();

                match key.as_str() {
                    "%FILENAME%" => filename = value.to_owned(),
                    "%NAME%" => name = value.to_owned(),
                    "%BASE%" => base = value.to_owned(),
                    "%VERSION%" => version = value.to_owned(),
                    "%DESC%" => desc = value.to_owned(),
                    "%GROUPS%" => groups = value.to_owned(),
                    "%CSIZE%" => csize = value.to_owned(),
                    "%ISIZE%" => isize = value.to_owned(),
                    "%MD5SUM%" => md5eum = value.to_owned(),
                    "%SHA256SUM%" => sha256sum = value.to_owned(),
                    "%PGPSIG%" => pgpsig = value.to_owned(),
                    "%URL%" => url = value.to_owned(),
                    "%LICENSE%" => license = value.to_owned(),
                    "%ARCH%" => arch = value.to_owned(),
                    "%BUILDDATE%" => builddate = value.to_owned(),
                    "%PACKAGER%" => packager = value.to_owned(),
                    "%REPLACES%" => replaces = value.to_owned(),

                    _ => println!("Unknown key: {}", key.as_str()),
                }
            }
        }
    }

    Package {
        filename,
        name,
        base,
        version,
        desc,
        groups,
        csize,
        isize,
        md5eum,
        sha256sum,
        pgpsig,
        url,
        license,
        arch,
        builddate,
        packager,
        replaces,
    }
}
