use std::path::PathBuf;
use std::process::Command;

use crate::core::dependency::Crate;
use crate::util;
use crate::util::RunRequiringSuccess;

/// Access or build the documentation
#[derive(Debug, clap::Parser)]
pub struct DocCli {
    #[command(subcommand)]
    kind: DocKindCli,
}
#[derive(Debug, clap::Subcommand)]
enum DocKindCli {
    /// Long-form manual for openDuT
    Book {
        #[command(subcommand)]
        task: BookCli,
    },
    /// Build and pack homepage for openDuT
    Homepage {
        #[command(subcommand)]
        task: HomepageCli,
    },
}
#[derive(Debug, clap::Subcommand)]
enum BookCli {
    /// Create a distribution of the book.
    Build,
    /// Serve the book for viewing in a browser.
    Open,
}

#[derive(Debug, clap::Subcommand)]
enum HomepageCli {
    /// Build the homepage
    Build,
}

impl DocCli {
    pub fn default_handling(&self) -> crate::Result {
        match &self.kind {
            DocKindCli::Book { task } => match task {
                BookCli::Build => book::build()?,
                BookCli::Open => book::open()?,
            },
            DocKindCli::Homepage { task } => match task {
                HomepageCli::Build => homepage::build()?,
            },
        };
        Ok(())
    }
}

pub mod book {
    use tracing::info;
    use super::*;

    #[tracing::instrument]
    pub fn open() -> crate::Result {
        util::install_crate(Crate::Mdbook)?;
        util::install_crate(Crate::MdbookPlantuml)?;

        Command::new("mdbook")
            .arg("serve")
            .arg("--open")
            .arg("--port=4000")
            .arg("--dest-dir").arg(out_dir())
            .current_dir(doc_dir())
            .run_requiring_success()?;
        Ok(())
    }

    #[tracing::instrument]
    pub fn build() -> crate::Result {
        util::install_crate(Crate::Mdbook)?;
        util::install_crate(Crate::MdbookPlantuml)?;

        let out_dir = out_dir();

        Command::new("mdbook")
            .arg("build")
            .arg("--dest-dir").arg(&out_dir)
            .current_dir(doc_dir())
            .run_requiring_success()?;

        info!("Placed distribution into: {}", out_dir.display());

        Ok(())
    }

    fn doc_dir() -> PathBuf {
        crate::constants::workspace_dir().join("doc")
    }

    fn out_dir() -> PathBuf {
        crate::constants::target_dir().join("book")
    }
}

pub mod homepage {
    use tracing::info;
    use super::*;

    #[tracing::instrument]
    pub fn build() -> crate::Result {

        Command::new("mdbook")
            .arg("build")
            .arg("--dest-dir").arg(&out_dir().join("book"))
            .current_dir(doc_dir())
            .run_requiring_success()?;


        fs_extra::dir::copy(
            homepage_source_dir(),
            out_dir(),
            &fs_extra::dir::CopyOptions::default()
                .overwrite(true)
                .content_only(true)
        )?;

        fs_extra::dir::create(
            logos_out_dir(),
            true
        )?;

        for logo in RESOURCES_TO_INCLUDE {
            fs_extra::file::copy(
                &logos_source_dir().join(logo),
                &logos_out_dir().join(logo),
                &fs_extra::file::CopyOptions::default()
                    .overwrite(true)
            )?;
        }

        info!("Placed distribution into: {}", out_dir().display());

        Ok(())
    }

    fn homepage_source_dir() -> PathBuf { crate::constants::workspace_dir().join("opendut-homepage") }

    fn logos_source_dir() -> PathBuf { crate::constants::workspace_dir().join("resources").join("logos") }

    fn doc_dir() -> PathBuf { crate::constants::workspace_dir().join("doc") }

    fn out_dir() -> PathBuf { crate::constants::target_dir().join("homepage") }

    fn logos_out_dir() -> PathBuf { out_dir().join("resources/logos") }

    const RESOURCES_TO_INCLUDE: [&str; 2] = ["logo_light.png", "funded_by_the_european_union.svg"];
}
