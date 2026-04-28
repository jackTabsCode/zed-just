use zed_extension_api::{
    Worktree,
    serde_json::Value,
    settings::{CommandSettings, LspSettings},
};

pub(super) fn get_initialization_options(
    language_server_id: &str,
    worktree: &Worktree,
) -> Option<Value> {
    LspSettings::for_worktree(language_server_id, worktree)
        .ok()
        .and_then(|lsp_settings| lsp_settings.initialization_options)
}

pub(super) fn get_workspace_configuration(
    language_server_id: &str,
    worktree: &Worktree,
) -> Option<Value> {
    LspSettings::for_worktree(language_server_id, worktree)
        .ok()
        .and_then(|lsp_settings| lsp_settings.settings)
}

pub(super) fn get_binary_settings(
    language_server_id: &str,
    worktree: &Worktree,
) -> Option<CommandSettings> {
    LspSettings::for_worktree(language_server_id, worktree)
        .ok()
        .and_then(|lsp_settings| lsp_settings.binary)
}

pub(super) fn get_binary_path(binary_settings: &Option<CommandSettings>) -> Option<String> {
    binary_settings
        .as_ref()
        .and_then(|binary_settings| binary_settings.path.clone())
}

pub(super) fn get_binary_args(binary_settings: &Option<CommandSettings>) -> Option<Vec<String>> {
    binary_settings
        .as_ref()
        .and_then(|binary_settings| binary_settings.arguments.clone())
}

pub(super) fn get_binary_env(
    binary_settings: &Option<CommandSettings>,
) -> Option<Vec<(String, String)>> {
    binary_settings
        .as_ref()
        .and_then(|binary_settings| binary_settings.env.clone())
        .map(|env| {
            env.iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect()
        })
}
