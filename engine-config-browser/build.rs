use std::env;
use std::ffi::OsStr;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use engine_config::{Engine, EngineDatabase};
use fs_err::{self as fs, File, PathExt};
use mapped_command::{Command, ReturnNothing};

struct Repo {
    path: PathBuf,
    url: String,
}

impl Repo {}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    let build_dir = env::current_dir()?.join("build");

    if !build_dir.exists() {
        fs::create_dir(&build_dir).context("failed to create build directory")?;
    }

    let ro_repo = Repo {
        path: build_dir.join("RealismOverhaul"),
        url: "https://github.com/KSP-RO/RealismOverhaul".to_owned(),
    };
    let parser_repo = Repo {
        path: build_dir.join("ROEngineParser"),
        url: "https://github.com/al2me6/ROEngineParser".to_owned(),
    };

    for repo in &[&ro_repo, &parser_repo] {
        if !repo.path.exists() {
            Command::new("git", ReturnNothing)
                .with_arguments(&[
                    OsStr::new("clone"),
                    OsStr::new(&repo.url),
                    repo.path.as_os_str(),
                ])
                .run()?;
        }
        Command::new("git", ReturnNothing)
            .with_working_directory_override(Some(&repo.path))
            .with_arguments(&["reset", "--hard"])
            .run()?;
        Command::new("git", ReturnNothing)
            .with_working_directory_override(Some(&repo.path))
            .with_arguments(&["pull"])
            .run()?;
    }

    Command::new("dotnet", ReturnNothing)
        .with_working_directory_override(Some(&parser_repo.path.join("ROEngineParser")))
        .with_arguments(&[
            OsStr::new("run"),
            ro_repo
                .path
                .join(Path::new("GameData/RealismOverhaul/Engine_Configs"))
                .as_os_str(),
            build_dir.join("json").as_os_str(),
        ])
        .run()?;

    let mut engines = Vec::new();
    for entry in build_dir.join("json").fs_err_read_dir()? {
        let entry = entry?;
        if !entry.path().is_file() || entry.file_name() == "000Template_Config.json" {
            continue;
        }
        let reader = BufReader::new(File::open(entry.path())?);
        let json = serde_json::from_reader::<_, Engine>(reader);
        match json {
            Ok(engine) => {
                engines.push(engine);
            }
            Err(err) => {
                eprintln!(
                    "failed to load engine config `{}`: {}",
                    entry.path().to_string_lossy(),
                    err,
                );
            }
        }
    }

    let out_dir = env::var("OUT_DIR")?;
    let mut bin = BufWriter::new(File::create(Path::new(&out_dir).join("data.bin"))?);

    let database = EngineDatabase {
        engines: engines
            .into_iter()
            .map(|cfg| (cfg.title.clone(), cfg))
            .collect(),
    };

    bin.write_all(&bincode::serialize(&database)?)?;
    bin.flush()?;

    Ok(())
}
