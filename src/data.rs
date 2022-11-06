#[derive(Debug)]
pub struct Context {
    pub mirror: String,
    pub arch: String,
    pub cache_dir: String,
    pub tmp_dir: String,
}

#[derive(Debug)]
pub struct Package {
    pub filename: String,
    pub name: String,
    pub base: String,
    pub version: String,
    pub desc: String,
    pub groups: String,
    pub csize: String,
    pub isize: String,
    pub md5eum: String,
    pub sha256sum: String,
    pub pgpsig: String,
    pub url: String,
    pub license: String,
    pub arch: String,
    pub builddate: String,
    pub packager: String,
    pub replaces: String,
}
