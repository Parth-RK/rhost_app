use anyhow::{Context, Result};
use aes_soft::Aes256; // Changed from aes::Aes256 to aes_soft::Aes256
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
use log::{error, info};
use rand::rngs::OsRng;
use rand::RngCore;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use walkdir::WalkDir;
use zip::write::FileOptions;
use cipher::KeyIvInit;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

// Type alias for AES-256-CBC
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn process() -> Result<()> {
    // Initialize the logger
    env_logger::init();

    // Generate AES key and IV
    let mut key = [0u8; 32]; // 256-bit key
    let mut iv = [0u8; 16];  // 128-bit IV
    OsRng.fill_bytes(&mut key);
    OsRng.fill_bytes(&mut iv);

    // Define paths using PathBuf and join
    let main_folder = Path::new("sample_data").to_path_buf();
    let zip_folder_name = Path::new("zip_data.zip").to_path_buf();
    let output_file = Path::new("encrypted_data.bin").to_path_buf();

    if !main_folder.exists() {
        error!("Folder '{}' not found.", main_folder.display());
        return Ok(());
    }

    // // Grant permissions
    // if let Err(e) = grant_permission(&main_folder, 0o700) {
    //     error!("Failed to grant permissions: {:?}", e);
    // }

    // Zip the folder
    if let Err(e) = zip_folder(&main_folder, &zip_folder_name) {
        error!("Failed to zip folder: {:?}", e);
    } else {
        info!(
            "Zipped folder '{}' to '{}'",
            main_folder.display(),
            zip_folder_name.display()
        );
    }

    thread::sleep(Duration::from_secs(4));

    // Encrypt the zipped file
    if let Err(e) = encrypt_file(&key, &iv, &zip_folder_name, &output_file) {
        error!("Failed to encrypt file: {:?}", e);
    } else {
        info!("File encrypted and saved.");
        info!("Key: {:x?}", key);
        info!("IV: {:x?}", iv);
    }

    thread::sleep(Duration::from_secs(4));

    // Decrypt the file
    if let Err(e) = decrypt_file(&key, &iv, &output_file, &zip_folder_name) {
        error!("Failed to decrypt file: {:?}", e);
    } else {
        info!("File decrypted and saved.");
    }

    thread::sleep(Duration::from_secs(4));

    // Unzip the decrypted file
    if let Err(e) = unzip_folder(&zip_folder_name, &main_folder) {
        error!("Failed to unzip folder: {:?}", e);
    } else {
        info!(
            "Unzipped '{}' to '{}'",
            zip_folder_name.display(),
            main_folder.display()
        );
    }

    // // Grant permissions to the unzipped folder
    // if let Err(e) = grant_permission(&main_folder, 0o700) {
    //     error!("Failed to grant permissions: {:?}", e);
    // }

    Ok(())
}

// Function to grant permissions (no-op for Windows)
// fn grant_permission(path: &Path, permission: u32) -> Result<()> {
//     if cfg!(target_os = "windows") {
//         // No-op for Windows
//         Ok(())
//     } else {
//         use std::os::unix::fs::PermissionsExt;
//         fs::set_permissions(path, fs::Permissions::from_mode(permission))
//             .with_context(|| format!("Setting permissions for {:?}", path))
//     }
// }

// Function to zip a folder
fn zip_folder(folder_path: &Path, output_zip: &Path) -> Result<()> {
    let file = File::create(output_zip)
        .with_context(|| format!("Creating zip file {:?}", output_zip))?;
    let mut zip = zip::ZipWriter::new(file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    for entry in WalkDir::new(folder_path) {
        let entry = entry?;
        let path = entry.path();
        let name = path
            .strip_prefix(folder_path)
            .with_context(|| format!("Stripping prefix {:?}", folder_path))?
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid UTF-8 sequence in path"))?
            .replace("\\", "/"); // Ensure UNIX-style paths

        if path.is_file() {
            zip.start_file(&name, options)
                .with_context(|| format!("Adding file {:?} to zip", path))?;
            let mut f = File::open(&path)
                .with_context(|| format!("Opening file {:?}", path))?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)
                .with_context(|| format!("Reading file {:?}", path))?;
            zip.write_all(&buffer)
                .with_context(|| format!("Writing file {:?} to zip", path))?;
        } else if path.is_dir() {
            zip.add_directory(&name, options)
                .with_context(|| format!("Adding directory {:?} to zip", path))?;
        }
    }

    zip.finish()
        .with_context(|| format!("Finalizing zip file {:?}", output_zip))?;

    // Remove the original folder after zipping
    fs::remove_dir_all(folder_path)
        .with_context(|| format!("Removing directory {:?}", folder_path))?;
    info!("Deleted folder '{}' after zipping.", folder_path.display());

    Ok(())
}

// Function to unzip a folder
fn unzip_folder(zip_path: &Path, output_folder: &Path) -> Result<()> {
    let file = File::open(zip_path)
        .with_context(|| format!("Opening zip file {:?}", zip_path))?;
    let mut archive = zip::ZipArchive::new(file)
        .with_context(|| format!("Reading zip archive {:?}", zip_path))?;

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .with_context(|| format!("Accessing file at index {}", i))?;
        let outpath = output_folder.join(file.name());

        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath)
                .with_context(|| format!("Creating directory {:?}", outpath))?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)
                        .with_context(|| format!("Creating directory {:?}", p))?;
                }
            }
            let mut outfile = File::create(&outpath)
                .with_context(|| format!("Creating file {:?}", outpath))?;
            std::io::copy(&mut file, &mut outfile)
                .with_context(|| format!("Writing to file {:?}", outpath))?;
        }

        // Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))
                    .with_context(|| format!("Setting permissions for {:?}", outpath))?;
            }
        }
    }

    // Remove the zip file after unzipping
    fs::remove_file(zip_path)
        .with_context(|| format!("Removing zip file {:?}", zip_path))?;
    Ok(())
}

// Function to encrypt a file
fn encrypt_file(key: &[u8], iv: &[u8], input_file: &Path, output_file: &Path) -> Result<()> {
    // Initialize cipher
    let cipher = Aes256Cbc::new_from_slices(key, iv)
        .with_context(|| "Initializing AES-256-CBC cipher")?;

    // Read the input file
    let mut buffer = Vec::new();
    let mut f_in = File::open(input_file)
        .with_context(|| format!("Opening input file {:?}", input_file))?;
    f_in.read_to_end(&mut buffer)
        .with_context(|| format!("Reading input file {:?}", input_file))?;

    // Encrypt the data
    let ciphertext = cipher.encrypt_vec(&buffer);

    // Write the encrypted data
    let mut f_out = File::create(output_file)
        .with_context(|| format!("Creating output file {:?}", output_file))?;
    f_out.write_all(&ciphertext)
        .with_context(|| format!("Writing to output file {:?}", output_file))?;

    // Remove the original zip file after encryption
    fs::remove_file(input_file)
        .with_context(|| format!("Removing input file {:?}", input_file))?;
    info!(
        "Deleted '{:?}' after encryption.",
        input_file.display()
    );

    Ok(())
}

// Function to decrypt a file
fn decrypt_file(key: &[u8], iv: &[u8], input_file: &Path, output_file: &Path) -> Result<()> {
    // Initialize cipher
    let cipher = Aes256Cbc::new_from_slices(key, iv)
        .with_context(|| "Initializing AES-256-CBC cipher")?;

    // Read the encrypted file
    let mut buffer = Vec::new();
    let mut f_in = File::open(input_file)
        .with_context(|| format!("Opening input file {:?}", input_file))?;
    f_in.read_to_end(&mut buffer)
        .with_context(|| format!("Reading input file {:?}", input_file))?;

    // Decrypt the data
    let decrypted_data = cipher.decrypt_vec(&buffer)
        .with_context(|| "Decrypting data")?;

    // Write the decrypted data
    let mut f_out = File::create(output_file)
        .with_context(|| format!("Creating output file {:?}", output_file))?;
    f_out.write_all(&decrypted_data)
        .with_context(|| format!("Writing to output file {:?}", output_file))?;

    // Remove the encrypted file after decryption
    fs::remove_file(input_file)
        .with_context(|| format!("Removing input file {:?}", input_file))?;
    info!(
        "Deleted '{:?}' after decryption.",
        input_file.display()
    );

    Ok(())
}
