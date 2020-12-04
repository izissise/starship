use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct StatusConfig<'a> {
    pub format: &'a str,
    pub program_error_symbol: &'a str,
    pub not_executable_symbol: &'a str,
    pub not_found_symbol: &'a str,
    pub sigint_symbol: &'a str,
    pub signal_symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for StatusConfig<'a> {
    fn new() -> Self {
        StatusConfig {
            format: "[$symbol$status]($style) ",
            program_error_symbol: "🔴",
            not_executable_symbol: "🚫",
            not_found_symbol: "🔍",
            sigint_symbol: "🧱",
            signal_symbol: "⚡",
            style: "bold red",
            disabled: true,
        }
    }
}
