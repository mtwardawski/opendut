use std::fs::File;
use std::path::Path;
use anyhow::Context;
use flate2::Compression;
use flate2::write::GzEncoder;
use crate::util::{EDGAR_IDENTIFIER, EdgarArch};

pub fn create_edgar(
    carl_install_directory: &Path,
) -> anyhow::Result<()> {

    for arch in EdgarArch::arch_iterator() {
        let edgar_file = carl_install_directory.join(EDGAR_IDENTIFIER).join(&arch.name()).join(EDGAR_IDENTIFIER);
        let file_name = format!("{}.tar.gz", &arch.name());
        let file_path = carl_install_directory.join(EDGAR_IDENTIFIER).join(&file_name);

        let tar_gz = File::create(&file_path)
            .context(format!("Could not create path '{}' for EDGAR archive.", &file_path.display()))?;

        let enc = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = tar::Builder::new(enc);
        tar.append_file(
            &arch.name(),
            &mut File::open(&edgar_file)
                .context(format!("Failed to open EDGAR executable file '{}'", edgar_file.display()))?
        )?;
        tar.into_inner()?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use assert_fs::fixture::{FileTouch, PathChild};
    use assert_fs::TempDir;
    use googletest::assert_that;
    use googletest::prelude::eq;

    use crate::provisioning::edgar::create_edgar;
    use crate::util::{EDGAR_IDENTIFIER, EdgarArch};

    #[tokio::test()]
    async fn creating_edgar_succeeds() -> anyhow::Result<()> {

        let temp = TempDir::new().unwrap();
        let dir = temp.child(EDGAR_IDENTIFIER);
        std::fs::create_dir_all(dir).unwrap();

        for arch in EdgarArch::arch_iterator() {
            let edgar_dir = temp.child(PathBuf::from(EDGAR_IDENTIFIER).join(arch.name()));
            std::fs::create_dir_all(edgar_dir).unwrap();

            let file = temp.child(PathBuf::from(EDGAR_IDENTIFIER).join(arch.name()).join(EDGAR_IDENTIFIER));
            file.touch().unwrap();
        }

        create_edgar(
            &temp.to_path_buf(),
        )?;

        for arch in EdgarArch::arch_iterator() {
            assert_that!(temp.join("opendut-edgar").join(format!("{}.tar.gz",arch.name())).exists(), eq(true));
        }

        Ok(())
    }
}