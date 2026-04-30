mod config;
mod util;

use std::fs;

use zed_extension_api::{
    Architecture, Command, DownloadedFileType, Extension, GithubReleaseOptions, LanguageServerId,
    LanguageServerInstallationStatus, Os, Result, Worktree, current_platform, download_file,
    latest_github_release, make_file_executable, register_extension, serde_json::Value,
    set_language_server_installation_status,
};

struct JustLspBinary {
    path: String,
    args: Vec<String>,
    env: Vec<(String, String)>,
}

struct JustExtension {
    cached_binary_path: Option<String>,
}

impl JustExtension {
    const LANGUAGE_SERVER_ID: &'static str = "just-lsp";

    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<JustLspBinary> {
        let (os, arch) = current_platform();
        let extension = match os {
            Os::Windows => ".exe",
            _ => "",
        };

        let binary_name = format!("{}{extension}", Self::LANGUAGE_SERVER_ID);
        let binary_settings = config::get_binary_settings(Self::LANGUAGE_SERVER_ID, worktree);
        let binary_args = config::get_binary_args(&binary_settings).unwrap_or_default();
        let binary_env = config::get_binary_env(&binary_settings).unwrap_or_default();

        // Check if already cached
        if let Some(binary_path) = &self.cached_binary_path
            && fs::metadata(binary_path).is_ok_and(|stat| stat.is_file())
        {
            return Ok(JustLspBinary {
                path: binary_path.clone(),
                args: binary_args,
                env: binary_env,
            });
        }

        // Check if just-lsp path was specified
        if let Some(binary_path) = config::get_binary_path(&binary_settings) {
            self.cached_binary_path = Some(binary_path.clone());
            return Ok(JustLspBinary {
                path: binary_path,
                args: binary_args,
                env: binary_env,
            });
        }

        // Check if just-lsp is on PATH
        if let Some(binary_path) = worktree.which(Self::LANGUAGE_SERVER_ID) {
            self.cached_binary_path = Some(binary_path.clone());
            return Ok(JustLspBinary {
                path: binary_path,
                args: binary_args,
                env: binary_env,
            });
        }

        // Download and install just-lsp
        set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::CheckingForUpdate,
        );

        // Check if already downloaded when fetching the latest GitHub release fails
        let release = match latest_github_release(
            "terror/just-lsp",
            GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        ) {
            Ok(release) => release,
            Err(_) => {
                if let Some(binary_path) =
                    util::find_existing_binary(Self::LANGUAGE_SERVER_ID, &binary_name)
                {
                    self.cached_binary_path = Some(binary_path.clone());
                    return Ok(JustLspBinary {
                        path: binary_path,
                        args: binary_args,
                        env: binary_env,
                    });
                }
                return Err("Failed to download just-lsp".to_string());
            }
        };

        let asset_name = format!(
            "{}-{version}-{target}.{extension}",
            Self::LANGUAGE_SERVER_ID,
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

        let version_dir = format!("{}-{}", Self::LANGUAGE_SERVER_ID, release.version);
        let binary_path = format!("{}/{}", version_dir, binary_name);

        // Check if already downloaded latest version
        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            set_language_server_installation_status(
                language_server_id,
                &LanguageServerInstallationStatus::Downloading,
            );

            let file_type = match os {
                Os::Windows => DownloadedFileType::Zip,
                _ => DownloadedFileType::GzipTar,
            };

            download_file(&asset.download_url, &version_dir, file_type)
                .map_err(|e| format!("Failed to download just-lsp: {}", e))?;

            make_file_executable(&binary_path)?;

            util::remove_outdated_versions(Self::LANGUAGE_SERVER_ID, &version_dir)?;
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(JustLspBinary {
            path: binary_path,
            args: binary_args,
            env: binary_env,
        })
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
        let just_lsp = self.language_server_binary_path(language_server_id, worktree)?;

        Ok(Command {
            command: just_lsp.path,
            args: just_lsp.args,
            env: just_lsp.env,
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
        let settings = config::get_initialization_options(Self::LANGUAGE_SERVER_ID, worktree)
            .unwrap_or_default();

        Ok(Some(settings))
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
        let settings = config::get_workspace_configuration(Self::LANGUAGE_SERVER_ID, worktree)
            .unwrap_or_default();

        Ok(Some(settings))
    }
}

register_extension!(JustExtension);
