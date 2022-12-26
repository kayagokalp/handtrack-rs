use anyhow::{anyhow, Result};
use downloader::Downloader;
use std::{fs, path::PathBuf};

/// Default folder to be placed in the home directory which will hold the frozen graph.
pub(crate) const DOWNLOAD_FOLDER: &str = ".handtrack-rs";
/// The URL that hosts the desired frozen graph.
pub(crate) const DOWNLOAD_URL: &str  = "https://raw.githubusercontent.com/victordibia/handtracking/master/hand_inference_graph/frozen_inference_graph.pb";
/// Default name of the downloaded frozen graph
pub(crate) const DOWNLOAD_NAME: &str = "frozen_inference_graph.pb";

/// Downloads the frozen graph from github URL.
pub(crate) fn download_frozen_graph() -> Result<()> {
    let home_dir = home::home_dir().ok_or_else(|| anyhow!("Cannot get home dir"))?;
    let download_dir = home_dir.join(DOWNLOAD_FOLDER);
    // If the download dir does not exists, create it.
    fs::create_dir_all(&download_dir)?;

    let mut downloader = Downloader::builder()
        .download_folder(&download_dir)
        .parallel_requests(1)
        .build()?;
    let dl = downloader::Download::new(DOWNLOAD_URL);

    println!(
        "Starting to download the frozen graph from {:?}",
        DOWNLOAD_URL
    );
    let result = downloader.download(&[dl])?;
    for r in result {
        let summary = r?;
        println!("Frozen graph downloaded: {:?}", summary.file_name);
    }
    Ok(())
}

/// Returns the default name of the frozen graph to be downloaded.
pub(crate) fn frozen_graph_path() -> Result<PathBuf> {
    let home_dir = home::home_dir().ok_or_else(|| anyhow!("Cannot get home dir"))?;
    Ok(home_dir.join(DOWNLOAD_FOLDER).join(DOWNLOAD_NAME))
}
