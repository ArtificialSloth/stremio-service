// Copyright (C) 2017-2024 Smart Code OOD 203358507

use clap::Parser;
use url::Url;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Whether or not to skip the updater
    ///
    /// This options is not used for `*nix` systems
    #[arg(short, long)]
    #[arg(group = "endpoint")]
    #[arg(group = "skip")]
    pub skip_updater: bool,

    /// If set, the updater will skip version check
    ///
    /// This options is not used for `*nix` systems
    #[arg(short, long)]
    #[arg(group = "skip")]
    pub force_update: bool,

    /// The endpoint to use for the updater
    ///
    /// Overrides the default endpoint
    #[clap(short, long)]
    #[arg(group = "endpoint")]
    pub updater_endpoint: Option<Url>,

    /// Updates the app to the latest release candidate
    ///
    /// This option is ignored when `--updater-endpoint` is set
    #[clap(short, long)]
    #[arg(group = "endpoint")]
    pub release_candidate: bool,

    /// Open an URL with a custom `stremio://` scheme.
    ///
    /// If empty URL or no url is provided, the service will skip this argument.
    #[clap(short, long)]
    pub open: Option<String>,

    /// Specifies the path to the app manifest
    ///
    /// Sent by the native messaging web extension.
    #[arg(index = 1, requires = "addon_id")]
    pub app_manifest: Option<String>,

    /// Specifies the native messaging addon ID
    ///
    /// Sent by the native messaging web extension.
    #[arg(index = 2)]
    pub addon_id: Option<String>
}
