use clap::*;
use std::error::Error;
use std::path::PathBuf;

use tracing::*;

mod bundle;
mod sign;

#[cfg(not(target_os = "macos"))]
compile_error!("This tool only works on macOS");

/// A utility tool to patch Mach-O binaries to allow code injection by `DYLD_INSERT_LIBRARIES` environment variable.
#[derive(Debug, Parser)]
struct Arguments {
    /// Do not print any log.
    #[clap(long)]
    slient: bool,
    /// Force to override output file/bundle directory if it exists.
    #[clap(short, long)]
    force: bool,
    /// Patch input path inplace.
    #[clap(long)]
    inplace: bool,
    /// Path to the Mach-O executable or Application bundle
    #[clap(value_name = "input")]
    input_path: PathBuf,
    /// Path to the output path.
    ///
    /// If input is a Mach-O binary then output will be the path to the new Mach-O binary.
    ///
    /// If input is an Application bundle then output will be the path to the new Application bundle.
    #[clap(value_name = "output")]
    output_path: Option<PathBuf>,
}

fn inner_main(args: Arguments) -> Result<(), Box<dyn Error>> {
    if !args.slient {
        tracing_subscriber::fmt().without_time().init();
    }

    let output_path = if args.inplace {
        args.input_path.clone()
    } else if let Some(output_path) = args.output_path {
        if output_path.exists() {
            if args.force {
                if output_path.is_file() {
                    std::fs::remove_file(&output_path)?;
                } else if output_path.is_dir() {
                    std::fs::remove_dir_all(&output_path)?;
                }
            } else {
                error!("Output path already exists");
                error!("if you want to override it, use -f, --force option");
                return Ok(());
            }
        }
        output_path
    } else {
        error!("Output path is required");
        error!("if you want to patch input inplace, use --inplace option");
        return Ok(());
    };

    info!("Input: {}", args.input_path.to_string_lossy());

    if let Ok((bundle_path, _bundle)) = bundle::find_dir_bundle(&args.input_path) {
        info!(
            "Found Application bundle: {}",
            bundle_path.to_string_lossy()
        );
        if !args.inplace {
            info!("Copying bundle to {}", output_path.to_string_lossy());
            std::process::Command::new("cp")
                .arg("-r")
                .arg(bundle_path)
                .arg(&output_path)
                .spawn()?
                .wait()?;
        }
        info!("Removing signature from {}", output_path.to_string_lossy());
        sign::remove_sign(&output_path)?;
        info!("Resigning {}", output_path.to_string_lossy());
        sign::sign(&output_path, None::<&str>)?;
    } else if apple_codesign::path_is_macho(&args.input_path).unwrap_or(false) {
        info!("Found Mach-O file: {}", args.input_path.to_string_lossy());
        if !args.inplace {
            info!("Copying file to {}", output_path.to_string_lossy());
            if let Some(parent) = output_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::copy(&args.input_path, &output_path)?;
        }
        info!("Removing signature from {}", output_path.to_string_lossy());
        sign::remove_sign(&output_path)?;
    } else {
        error!("No Application bundle or Mach-O executable found");
    }

    Ok(())
}

fn main() {
    if let Err(err) = inner_main(Arguments::parse()) {
        error!("{}", err);
        std::process::exit(1);
    }
    info!("Finished");
}
