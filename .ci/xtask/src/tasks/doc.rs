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

        log::info!("Placed distribution into: {}", out_dir.display());

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
    use crate::core::constants::workspace_dir;
    use super::*;

    #[tracing::instrument]
    pub fn build() -> crate::Result {
        util::install_crate(Crate::Mdbook)?;
        util::install_crate(Crate::MdbookPlantuml)?;

        let out_dir = out_dir();

        Command::new("mdbook")
            .arg("build")
            .arg("--dest-dir").arg(&out_dir.join("book"))
            .current_dir(doc_dir())
            .run_requiring_success()?;

        Command::new("cp")
            .arg("--recursive")
            .arg("--update")
            .arg(&homepage_source_dir())
            .arg(&out_dir)
            .run_requiring_success()?;

        log::info!("Placed distribution into: {}", out_dir.display());

        Ok(())
    }

    fn homepage_source_dir() -> PathBuf { workspace_dir().join("opendut-homepage/.") }

    fn doc_dir() -> PathBuf {
        workspace_dir().join("doc")
    }

    fn out_dir() -> PathBuf {
        crate::constants::target_dir().join("homepage")
    }
}
