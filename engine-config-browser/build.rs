use std::env;
use std::path::{Path, PathBuf};

use engine_config::{Engine, EngineDatabase};
use xshell::{cmd, mkdir_p, pushd, pushenv, read_dir, read_file, write_file};

struct Repo {
    path: PathBuf,
    url: String,
}

impl Repo {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");

    let build_dir = env::current_dir()?.join("build");
    mkdir_p(&build_dir)?;

    let ro_repo = Repo {
        path: build_dir.join("RealismOverhaul"),
        url: "https://github.com/KSP-RO/RealismOverhaul".to_owned(),
    };
    let parser_repo = Repo {
        path: build_dir.join("ROEngineParser"),
        url: "https://github.com/al2me6/ROEngineParser".to_owned(),
    };

    for Repo { path, url } in &[&ro_repo, &parser_repo] {
        if !path.exists() {
            cmd!("git clone {url} {path}").run()?;
        }
        let _d = pushd(path)?;
        cmd!("git reset --hard").run()?;
        cmd!("git pull").run()?;
    }

    {
        let _d = pushd(&parser_repo.path.join("ROEngineParser"))?;
        let config_path = ro_repo
            .path
            .join(Path::new("GameData/RealismOverhaul/Engine_Configs"));
        let output_path = build_dir.join("json");
        cmd!("dotnet run {config_path} {output_path}").run()?;
    }

    let mut engines = Vec::new();
    for cfg in read_dir(build_dir.join("json"))? {
        if !cfg.is_file() || cfg.file_name().unwrap() == "000Template_Config.json" {
            continue;
        }
        let json = serde_json::from_str::<Engine>(&read_file(&cfg)?);
        match json {
            Ok(engine) => {
                engines.push(engine);
            }
            Err(err) => {
                eprintln!(
                    "failed to load engine config `{}`: {}",
                    cfg.to_string_lossy(),
                    err,
                );
            }
        }
    }

    let (commit, timestamp) = {
        let _d = pushd(ro_repo.path)?;
        let _e = pushenv("TZ", "UTC");
        let commit = cmd!("git rev-parse HEAD").read()?;
        let time = cmd!("git show -s --format=%cd {commit} --date=iso-local")
            .read()?
            .replace(" +0000", "");
        (commit, time)
    };

    let database = EngineDatabase {
        commit,
        timestamp,
        engines: engines
            .into_iter()
            .map(|cfg| (cfg.title.clone(), cfg))
            .collect(),
    };

    let out_dir = env::var("OUT_DIR")?;
    write_file(
        Path::new(&out_dir).join("data.bin"),
        bincode::serialize(&database)?,
    )?;

    Ok(())
}
