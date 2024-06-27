use zed_extension_api as zed;

struct RimeExtension {}

impl RimeExtension {}

impl zed::Extension for RimeExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let path = worktree
            .which("rime_ls")
            .ok_or_else(|| "rime_ls must be installed and available in $PATH.".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        // default initialization options, users can override it in settings
        let initialization_options = r#"{
            "enabled": true,
            "shared_data_dir": "/usr/share/rime-data",
            "user_data_dir": "~/.local/share/rime-ls",
            "log_dir": "/tmp",
            "max_candidates": 9,
            "paging_characters": [",", "."],
            "trigger_characters": [],
            "schema_trigger_character": "&",
            "long_filter_text": true,
            "show_filter_text_in_label": true
        }"#;

        Ok(Some(
            zed::serde_json::from_str(initialization_options).unwrap(),
        ))
    }
}

zed::register_extension!(RimeExtension);
