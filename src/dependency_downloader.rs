use futures::StreamExt;
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use tokio::{
    fs::File,
    io::AsyncWriteExt,
};
use yansi::Paint;

use crate::config::Dependency;
use crate::errors::DownloadError;
use crate::errors::UnzippingError;
use crate::remote::get_dependency_url_remote;
use crate::utils::read_file;
use crate::DEPENDENCY_DIR;

pub async fn download_dependencies(
    dependencies: &[Dependency],
    clean: bool,
) -> Result<(), DownloadError> {
    // clean dependencies folder if flag is true
    if clean {
        clean_dependency_directory();
    }
    // downloading dependencies to dependencies folder
    for dependency in dependencies.iter() {
        let file_name: String = format!("{}-{}.zip", dependency.name, dependency.version);
        match download_dependency(&file_name, &dependency.url).await {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        }
    }
    Ok(())
}

// un-zip-ing dependencies to dependencies folder
pub fn unzip_dependencies(dependencies: &[Dependency]) -> Result<(), UnzippingError> {
    for dependency in dependencies.iter() {
        match unzip_dependency(&dependency.name, &dependency.version) {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        }
    }
    Ok(())
}

pub async fn download_dependency_remote(
    dependency_name: &String,
    dependency_version: &String,
) -> Result<String, DownloadError> {
    let dependency_url = match get_dependency_url_remote(dependency_name, dependency_version).await
    {
        Ok(url) => url,
        Err(err) => return Err(err),
    };

    match download_dependency(
        &format!("{}-{}.zip", &dependency_name, &dependency_version),
        &dependency_url,
    )
    .await
    {
        Ok(_) => Ok(dependency_url),
        Err(err) => {
            eprintln!("Error downloading dependency: {:?}", err);
            Err(err)
        }
    }
}

pub async fn download_dependency(
    dependency_name: &str,
    dependency_url: &str,
) -> Result<(), DownloadError> {
    let dependency_directory: PathBuf = DEPENDENCY_DIR.clone();
    if !DEPENDENCY_DIR.is_dir() {
        fs::create_dir(&dependency_directory).unwrap();
    }

    let mut file = File::create(&dependency_directory.join(dependency_name))
        .await
        .unwrap();

    let mut stream = match reqwest::get(dependency_url).await {
        Ok(res) => res.bytes_stream(),
        Err(_) => {
            return Err(DownloadError {
                name: "Unknown".to_string(),
                version: "Unknown".to_string(),
            });
        }
    };

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result;
        match file.write_all(&chunk.unwrap()).await {
            Ok(_) => {}
            Err(_) => {
                return Err(DownloadError {
                    name: "Unknown".to_string(),
                    version: "Unknown".to_string(),
                });
            }
        }
    }

    match file.flush().await {
        Ok(_) => {}
        Err(_) => {
            return Err(DownloadError {
                name: "Unknown".to_string(),
                version: "Unknown".to_string(),
            });
        }
    };

    println!(
        "{}",
        Paint::green(&format!("Dependency {} downloaded! ", dependency_name))
    );

    Ok(())
}

pub fn unzip_dependency(
    dependency_name: &String,
    dependency_version: &String,
) -> Result<(), UnzippingError> {
    let file_name: String = format!("{}-{}.zip", dependency_name, dependency_version);
    let target_name: String = format!("{}-{}/", dependency_name, dependency_version);
    let current_dir: PathBuf = DEPENDENCY_DIR.join(file_name);
    let target = DEPENDENCY_DIR.join(target_name);
    let archive: Vec<u8> = read_file(current_dir.as_path().to_str().unwrap().to_string()).unwrap();

    match zip_extract::extract(Cursor::new(archive), &target, true) {
        Ok(_) => {}
        Err(_) => {
            return Err(UnzippingError {
                name: dependency_name.to_string(),
                version: dependency_version.to_string(),
            });
        }
    }
    println!(
        "{}",
        Paint::green(&format!(
            "The dependency {}-{} was unzipped!",
            dependency_name, dependency_version
        ))
    );
    Ok(())
}

pub fn clean_dependency_directory() {
    if DEPENDENCY_DIR.is_dir() {
        fs::remove_dir_all(DEPENDENCY_DIR.clone()).unwrap();
        fs::create_dir(DEPENDENCY_DIR.clone()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::MissingDependencies;
    use crate::janitor::healthcheck_dependency;
    use serial_test::serial;

    // Helper macro to run async tests
    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    #[serial]
    fn unzip_dependency_success() {
        let mut dependencies: Vec<Dependency> = Vec::new();
        dependencies.push(Dependency {
            name: "@openzeppelin-contracts".to_string(),
            version: "2.3.0".to_string(),
            url: "https://github.com/mario-eth/soldeer-versions/raw/main/all_versions/@openzeppelin-contracts~2.3.0.zip".to_string(),
        });
        let _ = aw!(download_dependencies(&dependencies, false));
        let result: Result<(), UnzippingError> =
            unzip_dependency(&dependencies[0].name, &dependencies[0].version);
        assert!(result.is_ok());
        let result: Result<(), MissingDependencies> =
            healthcheck_dependency("@openzeppelin-contracts", "2.3.0");
        assert!(result.is_ok());
    }
}
