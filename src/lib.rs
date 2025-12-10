use std::fs;

use zed_extension_api::{
    Architecture, Command, DownloadedFileType, Extension, GithubReleaseOptions, LanguageServerId,
    Os, Result, Worktree, current_platform, download_file, latest_github_release,
    make_file_executable, register_extension,
};

struct JustExtension {
    cached_binary_path: Option<String>,
}

impl JustExtension {
    fn language_server_binary_path(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<String> {
        // Check if already cached
        if let Some(path) = &self.cached_binary_path
            && fs::metadata(path).is_ok_and(|stat| stat.is_file())
        {
            return Ok(path.clone());
        }

        // Check if just-lsp is on PATH
        if let Some(path) = worktree.which("just-lsp") {
            self.cached_binary_path = Some(path.clone());
            return Ok(path);
        }

        // Download and install just-lsp
        let release = latest_github_release(
            "terror/just-lsp",
            GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (os, arch) = current_platform();
        let asset_name = format!(
            "just-lsp-{version}-{target}.{extension}",
            version = release.version,
            target = match (os, arch) {
                (Os::Mac, Architecture::Aarch64) => "aarch64-apple-darwin",
                (Os::Mac, Architecture::X8664) => "x86_64-apple-darwin",
                (Os::Linux, Architecture::Aarch64) => "aarch64-unknown-linux-gnu",
                (Os::Linux, Architecture::X8664) => "x86_64-unknown-linux-gnu",
                (Os::Windows, Architecture::Aarch64) => "aarch64-pc-windows-msvc",
                (Os::Windows, Architecture::X8664) => "x86_64-pc-windows-msvc",
                _ => return Err(format!("Unsupported platform: {:?} {:?}", os, arch)),
            },
            extension = match os {
                Os::Windows => "zip",
                _ => "tar.gz",
            }
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("Asset {} not found in release", asset_name))?;

        let version_dir = format!("just-lsp-{}", release.version);
        let binary_path = format!(
            "{}/just-lsp{}",
            version_dir,
            match os {
                Os::Windows => ".exe",
                _ => "",
            }
        );

        // Check if already downloaded
        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            download_file(
                &asset.download_url,
                &version_dir,
                match os {
                    Os::Windows => DownloadedFileType::Zip,
                    _ => DownloadedFileType::GzipTar,
                },
            )
            .map_err(|e| format!("Failed to download just-lsp: {}", e))?;

            make_file_executable(&binary_path)?;
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl Extension for JustExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let path = self.language_server_binary_path(language_server_id, worktree)?;

        Ok(Command {
            command: path,
            args: vec![],
            env: Default::default(),
        })
    }
}

register_extension!(JustExtension);
