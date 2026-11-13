//! Custom SuSFS lists (path / path_loop / map / open_redirect / kernel_umount).

use serde::{Deserialize, Serialize};

use crate::paths::{self, ensure_data_dir, read_lines, write_lines};
use duck_core::Context;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CustomListKind {
    SusPath,
    SusPathLoop,
    SusMap,
    OpenRedirect,
    KernelUmount,
}

impl CustomListKind {
    fn file_name(self) -> &'static str {
        match self {
            Self::SusPath => "custom_sus_path.txt",
            Self::SusPathLoop => "custom_sus_path_loop.txt",
            Self::SusMap => "custom_sus_map.txt",
            Self::OpenRedirect => "custom_open_redirect.txt",
            Self::KernelUmount => "custom_kernel_umount.txt",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomLists {
    pub sus_path: Vec<String>,
    pub sus_path_loop: Vec<String>,
    pub sus_map: Vec<String>,
    pub open_redirect: Vec<String>,
    pub kernel_umount: Vec<String>,
}

impl CustomLists {
    pub fn load(ctx: &Context) -> anyhow::Result<Self> {
        ensure_data_dir(ctx)?;
        Ok(Self {
            sus_path: read_lines(&paths::custom_file(ctx, CustomListKind::SusPath.file_name()))?,
            sus_path_loop: read_lines(&paths::custom_file(
                ctx,
                CustomListKind::SusPathLoop.file_name(),
            ))?,
            sus_map: read_lines(&paths::custom_file(ctx, CustomListKind::SusMap.file_name()))?,
            open_redirect: read_lines(&paths::custom_file(
                ctx,
                CustomListKind::OpenRedirect.file_name(),
            ))?,
            kernel_umount: read_lines(&paths::custom_file(
                ctx,
                CustomListKind::KernelUmount.file_name(),
            ))?,
        })
    }

    pub fn set(ctx: &Context, kind: CustomListKind, lines: Vec<String>) -> anyhow::Result<()> {
        ensure_data_dir(ctx)?;
        write_lines(&paths::custom_file(ctx, kind.file_name()), &lines)
    }
}
