use sha2::{Digest, Sha512};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Verify a downloaded file against its expected SHA-512 hash
pub fn verify_file_hash(
    file_path: &Path,
    expected_hash: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let mut hasher = Sha512::new();
    hasher.update(&contents);
    let hash = hasher.finalize();
    let hash_hex = format!("{:x}", hash);

    let matches = hash_hex.eq_ignore_ascii_case(expected_hash);

    if !matches {
        tracing::warn!(
            "Hash mismatch for {:?}: expected {}, got {}",
            file_path,
            expected_hash,
            hash_hex
        );
    } else {
        tracing::debug!("Hash verified for {:?}", file_path);
    }

    Ok(matches)
}

/// Download a mod from a URL with retry logic
pub async fn download_mod(
    url: &str,
    output_path: &Path,
    max_retries: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut retries = 0;

    loop {
        match download_with_progress(url, output_path, &client).await {
            Ok(_) => {
                tracing::info!("Downloaded: {} to {:?}", url, output_path);
                return Ok(());
            }
            Err(e) if retries < max_retries => {
                retries += 1;
                tracing::warn!(
                    "Download failed (attempt {}/{}): {}. Retrying...",
                    retries,
                    max_retries,
                    e
                );
                tokio::time::sleep(tokio::time::Duration::from_secs(
                    std::cmp::min(2_u64.pow(retries), 30),
                ))
                .await;
            }
            Err(e) => {
                tracing::error!("Failed to download {} after {} retries: {}", url, max_retries, e);
                return Err(e);
            }
        }
    }
}

/// Download with streaming progress tracking
async fn download_with_progress(
    url: &str,
    output_path: &Path,
    client: &reqwest::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()).into());
    }

    let content = response.bytes().await?;

    // Create parent directories if they don't exist
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(output_path, content)?;
    Ok(())
}

/// Download and verify a mod file in one operation
pub async fn download_and_verify_mod(
    url: &str,
    output_path: &Path,
    expected_hash: &str,
    max_retries: u32,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Download the file
    download_mod(url, output_path, max_retries).await?;

    // Verify the hash
    verify_file_hash(output_path, expected_hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_hash_verification() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, b"Hello, World!")?;

        // Calculate the actual hash
        let mut hasher = Sha512::new();
        hasher.update(b"Hello, World!");
        let expected_hash = format!("{:x}", hasher.finalize());

        let result = verify_file_hash(&test_file, &expected_hash)?;
        assert!(result);

        // Test with wrong hash
        let wrong_hash = "0000000000000000000000000000000000000000000000000000000000000000";
        let wrong_result = verify_file_hash(&test_file, wrong_hash)?;
        assert!(!wrong_result);

        Ok(())
    }
}
