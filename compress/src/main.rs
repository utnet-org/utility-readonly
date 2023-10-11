
extern crate flate2;
extern crate zip;

use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use flate2::write::GzEncoder;
use flate2::write::GzDecoder;
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

fn compress_file(input_filename:&str,output_filename:&str)  -> std::io::Result<()> {
    let input_file = File::open(input_filename).expect("find input_filename wrong");
    let output_file = File::create(output_filename).expect("create output_filename wrong");
    let mut encoder = GzEncoder::new(output_file, flate2::Compression::default());
    let mut buffer = Vec::new();
    input_file.take(1024).read_to_end(&mut buffer).expect("read buffer wrong");

    encoder.write_all(&buffer).expect("write buffer wrong");

    Ok(())
}
fn decompress_file(input_filename: &str, output_filename: &str) -> std::io::Result<()> {
    let input_file = File::open(input_filename).expect("find input_filename wrong");
    let mut output_file = File::create(output_filename).expect("create output_filename wrong");
    let mut decoder = GzDecoder::new(input_file);

    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer).expect("read buffer wrong");

    output_file.write_all(&buffer).expect("write buffer wrong");

    Ok(())
}
fn compress_file_zip(input_filename: &str, output_filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(output_filename)?;
    let mut zip = ZipWriter::new(file);

    let options = FileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .unix_permissions(0o755);

    zip.start_file("compressed_file.txt", options)?;

    let mut input_file = File::open(input_filename)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    zip.write_all(&buffer)?;

    Ok(())
}

fn decompress_file_zip(zip_filename: &str, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(zip_filename)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        let output_path = format!("{}/{}", output_dir, file.name());
        if file.is_dir() {
            std::fs::create_dir_all(&output_path)?;
        } else {
            let mut output_file = File::create(&output_path)?;
            std::io::copy(&mut file, &mut output_file)?;
        }
    }

    Ok(())
}

fn compress_directory(directory_path: &str, zip_filename: &str) -> std::io::Result<()> {
    let output_file = File::create(zip_filename)?;
    let mut zip = ZipWriter::new(output_file);

    // 设置压缩选项
    let options = FileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .unix_permissions(0o755);

    // 递归遍历文件夹并将文件添加到ZIP
    add_directory_to_zip(&mut zip, directory_path, directory_path, options)?;

    Ok(())
}

fn add_directory_to_zip<W>(
    zip: &mut ZipWriter<W>,
    base_path: &str,
    directory_path: &str,
    options: FileOptions,
) -> std::io::Result<()>
    where
        W: Write + io::Seek,
{
    for entry in std::fs::read_dir(directory_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_str().unwrap();
            zip.start_file_from_path(Path::new(&format!("{}/", dir_name)), options)?;
            add_directory_to_zip(zip, base_path, &path.to_string_lossy(), options)?;
        } else if path.is_file() {
            let mut file = File::open(&path)?;
            let relative_path = path.strip_prefix(base_path).expect("strip_prefix wrong");

            zip.start_file_from_path(Path::new(relative_path), options).expect("strip_prefix wrong");
            std::io::copy(&mut file, zip)?;
        }
    }

    Ok(())
}
fn decompress_directory(zip_filename: &str, output_directory: &str) -> std::io::Result<()> {
    let zip_file = File::open(zip_filename)?;
    let reader = std::io::BufReader::new(zip_file);
    let mut archive = ZipArchive::new(reader)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_path = file.sanitized_name();

        let output_path = Path::new(output_directory).join(file_path);

        if file.is_dir() {
            std::fs::create_dir_all(&output_path)?;
        } else {
            if let Some(parent) = output_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut output_file = File::create(&output_path)?;
            std::io::copy(&mut file, &mut output_file)?;
        }
    }

    Ok(())
}

fn main() {
    println!("hello world")

}

#[cfg(test)]
mod tests {
    use crate::{ compress_file, compress_file_zip, decompress_file, decompress_file_zip,compress_directory,decompress_directory};

    #[test]
    fn test_compress_file() -> Result<(), Box<dyn std::error::Error>>{
        let input_filename = "src/input.txt";
        let compressed_filename = "src/outcome/compressed.gz";
        let compressed_filename_zip = "src/outcome/compressed.zip";
        let decompressed_filename = "src/outcome/decompressed.txt";
        let decompressed_filename_zip = "src/outcome";

        // 压缩文件
        compress_file(input_filename, compressed_filename).expect("compress fail");
        compress_file_zip(input_filename,compressed_filename_zip).expect("compress fail");
        // 解压缩文件
        decompress_file(compressed_filename, decompressed_filename).expect("decompress fail");
        decompress_file_zip(compressed_filename_zip,decompressed_filename_zip).expect("decompress fail");

        Ok(())
    }

    #[test]
    fn test_compress_directory() -> Result<(), Box<dyn std::error::Error>> {
        let input_directory = "src/graph";
        let compressed_zip = "src/outcome/compressed_directory.zip";
        let output_directory = "src/outcome/extracted_graph";

        // 压缩文件夹
        compress_directory(input_directory, compressed_zip).expect("compress file fail");
        //
        // // 解压缩文件夹
        decompress_directory(compressed_zip, output_directory).expect("decompress file fail");

        Ok(())
    }
}







