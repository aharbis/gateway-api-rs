//! Generates a crate's Rust bindings for a Kubernetes API group from its Go
//! source. See `codegen/README.md`.
//!
//! Generation has two steps. gofacts, the Go package in `codegen/gofacts`,
//! reads the Go packages and CRDs a crate's config names and writes one IR
//! file per profile. This crate reads the IR and writes the Rust.

#![deny(unsafe_code)]
#![warn(clippy::all, clippy::pedantic, rust_2018_idioms)]
#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::struct_excessive_bools,
    // `quote!` blocks are long but flat
    clippy::too_many_lines
)]

pub mod config;
mod emit;
pub mod index;
pub mod ir;
mod model;
pub mod naming;
mod write;

use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result, bail};

pub use crate::{
    config::Config,
    index::Index,
    ir::Ir,
    write::{Generated, GeneratedProfile, Resource, ResourceVersion},
};

/// A crate's codegen: its config and each profile's IR, in the config's
/// profile order.
pub struct Codegen {
    /// The repository root.
    pub root: PathBuf,
    pub config: Config,
    pub irs: Vec<Ir>,
}

impl Codegen {
    /// Runs gofacts for the crate whose codegen directory is `dir`, relative
    /// to `root`, then loads what it wrote.
    pub fn generate_ir(root: &Path, dir: &Path) -> Result<Self> {
        let codegen_dir = root.join(dir);
        let dir_display = dir.display();
        eprintln!("reading the Go source with {dir_display}");
        let status = Command::new("go")
            .args(["run", ".", "-root"])
            .arg(root)
            .current_dir(&codegen_dir)
            .status()
            .context("failed to run go — is it installed?")?;
        if !status.success() {
            bail!("gofacts failed in {dir_display}");
        }
        Self::load(root, dir)
    }

    /// Loads the config and IR in the crate's codegen directory `dir`,
    /// relative to `root`.
    pub fn load(root: &Path, dir: &Path) -> Result<Self> {
        let codegen_dir = root.join(dir);
        let config = Config::load(&codegen_dir.join("config.yaml"))?;
        let mut irs = Vec::new();
        for profile in &config.profiles {
            let name = &profile.name;
            let ir = Ir::load(&codegen_dir.join("ir").join(format!("{name}.json")))?;
            if ir.profile != *name || ir.module != config.module {
                bail!(
                    "{dir}/ir/{name}.json is for another profile or module",
                    dir = dir.display()
                );
            }
            irs.push(ir);
        }
        Ok(Self {
            root: root.to_path_buf(),
            config,
            irs,
        })
    }

    /// The module version the IR was read at, e.g. `v1.5.1`.
    pub fn version(&self) -> &str {
        &self.irs[0].version
    }
}
