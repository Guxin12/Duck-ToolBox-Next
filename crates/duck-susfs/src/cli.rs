//! `duckd susfs` subcommands.

use clap::Subcommand;
use serde::Deserialize;

use crate::apply::{self, ApplyStage};
use crate::config::SusfsConfig;
use crate::custom::{CustomListKind, CustomLists};
use crate::error::code_of;
use crate::status;
use duck_core::{BoxFuture, ClapFeature, CommandResult, Context, FeatureInfo, IntoCommandResult};

pub static FEATURE: ClapFeature<Command> = ClapFeature::new(
    FeatureInfo {
        id: "susfs",
        summary: "SuSFS manager.",
        contract: 1,
    },
    dispatch,
);

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Home / Info status: versions, features, device summary, full config + custom lists.
    Status,
    /// Return current config only.
    GetConfig,
    /// Replace config from stdin JSON (full SusfsConfig object).
    SetConfig {
        #[arg(long, required = true)]
        stdin_json: bool,
    },
    /// Return all custom lists.
    GetCustom,
    /// Replace one custom list from stdin JSON: {"kind":"sus_path","lines":["..."]}.
    SetCustom {
        #[arg(long, required = true)]
        stdin_json: bool,
    },
    /// Apply config. stage: early | late | all (default all).
    Apply {
        #[arg(long, default_value = "all")]
        stage: String,
    },
}

#[derive(Debug, Deserialize)]
struct SetCustomBody {
    kind: CustomListKind,
    lines: Vec<String>,
}

fn dispatch<'a>(command: Command, ctx: &'a Context) -> BoxFuture<'a, CommandResult> {
    Box::pin(async move {
        match command {
            Command::Status => status::collect(ctx).into_command("susfs.status", code_of),
            Command::GetConfig => SusfsConfig::load(ctx).into_command("susfs.get-config", code_of),
            Command::SetConfig { .. } => set_config(ctx).into_command("susfs.set-config", code_of),
            Command::GetCustom => CustomLists::load(ctx).into_command("susfs.get-custom", code_of),
            Command::SetCustom { .. } => set_custom(ctx).into_command("susfs.set-custom", code_of),
            Command::Apply { stage } => {
                let stage = match stage.as_str() {
                    "early" => ApplyStage::Early,
                    "late" => ApplyStage::Late,
                    _ => ApplyStage::All,
                };
                apply::apply(ctx, stage).into_command("susfs.apply", code_of)
            }
        }
    })
}

fn set_config(ctx: &Context) -> anyhow::Result<SusfsConfig> {
    let mut bytes = Vec::new();
    std::io::Read::read_to_end(&mut std::io::stdin().take(64 * 1024), &mut bytes)?;
    let cfg: SusfsConfig = serde_json::from_slice(&bytes)
        .map_err(|e| crate::error::fail("invalid_input", e.to_string()))?;
    cfg.save(ctx)?;
    Ok(cfg)
}

fn set_custom(ctx: &Context) -> anyhow::Result<CustomLists> {
    let mut bytes = Vec::new();
    std::io::Read::read_to_end(&mut std::io::stdin().take(64 * 1024), &mut bytes)?;
    let body: SetCustomBody = serde_json::from_slice(&bytes)
        .map_err(|e| crate::error::fail("invalid_input", e.to_string()))?;
    CustomLists::set(ctx, body.kind, body.lines)?;
    CustomLists::load(ctx)
}
