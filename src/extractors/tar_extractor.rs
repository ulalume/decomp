use anyhow::Result;
use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use tar::Archive;
use xz2::read::XzDecoder;

// Import the i18n macro
use rust_i18n::t;

// Import common decode function
use super::common::decode_filename_as_pathbuf;

pub fn extract_tar(file_path: &Path, extract_dir: &Path) -> Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut archive = Archive::new(reader);

    fs::create_dir_all(extract_dir)?;

    // プログレスバーの設定（TARは事前にエントリ数が分からないのでスピナー形式）
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {elapsed_precise} {msg}")
            .unwrap(),
    );
    pb.set_message(format!("{}", t!("progress.extracting_tar")));

    extract_tar_entries(&mut archive, extract_dir, &pb)?;

    Ok(())
}

pub fn extract_tar_gz(file_path: &Path, extract_dir: &Path) -> Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let gz_decoder = GzDecoder::new(reader);
    let mut archive = Archive::new(gz_decoder);

    fs::create_dir_all(extract_dir)?;

    // プログレスバーの設定
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {elapsed_precise} {msg}")
            .unwrap(),
    );
    pb.set_message(format!("{}", t!("progress.extracting_tar_gz")));

    extract_tar_entries(&mut archive, extract_dir, &pb)?;

    Ok(())
}

pub fn extract_tar_xz(file_path: &Path, extract_dir: &Path) -> Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let xz_decoder = XzDecoder::new(reader);
    let mut archive = Archive::new(xz_decoder);

    fs::create_dir_all(extract_dir)?;

    // プログレスバーの設定
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {elapsed_precise} {msg}")
            .unwrap(),
    );
    pb.set_message(format!("{}", t!("progress.extracting_tar_xz")));

    extract_tar_entries(&mut archive, extract_dir, &pb)?;

    Ok(())
}

pub fn extract_tar_bz2(file_path: &Path, extract_dir: &Path) -> Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let bz_decoder = BzDecoder::new(reader);
    let mut archive = Archive::new(bz_decoder);

    fs::create_dir_all(extract_dir)?;

    // プログレスバーの設定
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {elapsed_precise} {msg}")
            .unwrap(),
    );
    pb.set_message(format!("{}", t!("progress.extracting_tar_bz2")));

    extract_tar_entries(&mut archive, extract_dir, &pb)?;

    Ok(())
}

// 共通のTARエントリ処理関数
fn extract_tar_entries<R: std::io::Read>(
    archive: &mut Archive<R>,
    extract_dir: &Path,
    pb: &ProgressBar,
) -> Result<()> {
    for entry in archive.entries()? {
        let mut entry = entry?;

        // ファイル名を適切にデコード
        let path_bytes = entry.path_bytes();
        let decoded_path = decode_filename_as_pathbuf(&path_bytes);
        let output_path = extract_dir.join(&decoded_path);

        // プログレスバーのメッセージを更新
        if let Some(file_name) = decoded_path.file_name().and_then(|s| s.to_str()) {
            pb.set_message(format!(
                "{}",
                t!("progress.extracting_file", file = file_name)
            ));
        }

        entry.unpack(&output_path)?;
        pb.inc(1);
    }
    Ok(())
}
