use std::io::Read;

use anyhow::Context;

pub(super) const FILES_INDEX_URL: &str = "https://raw.githubusercontent.com/aviutl2/aviutl2-community-translation/main/locales/files.json";

const LOCALES_BASE_URL: &str = "https://raw.githubusercontent.com/aviutl2/aviutl2-community-translation/main/locales/";

pub(super) fn locale_file_url(file_name: &str) -> String {
    format!("{LOCALES_BASE_URL}{file_name}")
}

pub(super) fn fetch_url_bytes(url: &str) -> anyhow::Result<Vec<u8>> {
    let response = ureq::get(url)
        .call()
        .with_context(|| format!("failed to fetch URL: {url}"))?;
    let mut reader = response.into_body().into_reader();
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}
