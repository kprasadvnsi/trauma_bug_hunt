use std::path::PathBuf;

use tokio::runtime::Runtime;
use trauma::download::Download;
use trauma::downloader::DownloaderBuilder;
use trauma::Error;

extern crate lazy_static;
use lazy_static::lazy_static;

lazy_static! {
    static ref RT: Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
}

pub async fn download_bulk(paths: Vec<String>, output: &str) -> Result<(), Error> {
    let mut downloads: Vec<Download> = Vec::new();
    for path in paths {
        let down = Download::try_from(path.as_str()).unwrap();
        downloads.push(down);
    }
    let downloader = DownloaderBuilder::new()
        .directory(PathBuf::from(output))
        .concurrent_downloads(16)
        .build();
    downloader.download(&downloads).await;
    Ok(())
}

pub fn download(paths: Vec<String>, output: &str) {
    RT.block_on(download_bulk(paths, output)).unwrap();
}
