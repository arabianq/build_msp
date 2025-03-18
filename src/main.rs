use clap::{Error, Parser};
use std::collections::HashSet;
use std::fs::{File, ReadDir};
use std::fs::{copy, create_dir, read_dir, remove_dir_all, remove_file, set_permissions};
use std::io::Write;
use std::path::absolute;
use std::path::{Path, PathBuf};

#[cfg(target_os = "windows")]
const BUILD_ROMFS_BIN: &[u8] = include_bytes!("../switch-tools/build_romfs.exe");
#[cfg(target_os = "windows")]
const BUILD_PFS0_BIN: &[u8] = include_bytes!("../switch-tools/build_pfs0.exe");

#[cfg(not(target_os = "windows"))]
const BUILD_ROMFS_BIN: &[u8] = include_bytes!("../switch-tools/build_romfs");
#[cfg(not(target_os = "windows"))]
const BUILD_PFS0_BIN: &[u8] = include_bytes!("../switch-tools/build_pfs0");

const FILES_TO_COPY: [&str; 28] = [
    "romfs.bin",
    "exefs.nsp",
    "rtld",
    "main",
    "main.npdm",
    "compat0",
    "compat1",
    "compat2",
    "compat3",
    "compat4",
    "compat5",
    "compat6",
    "compat7",
    "compat8",
    "compat9",
    "subsdk0",
    "subsdk1",
    "subsdk2",
    "subsdk3",
    "subsdk4",
    "subsdk5",
    "subsdk6",
    "subsdk7",
    "subsdk8",
    "subsdk9",
    "sdk",
    "config.ini",
    "icon.jpg",
];

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'i', long = "input", default_value = ".")]
    input: PathBuf,

    #[arg(short = 'o', long = "output", default_value = "mod.msp")]
    output: PathBuf,

    #[arg(short = 'm', long = "manifest", default_value = "./manifest")]
    manifest: PathBuf,
}

fn list_items(path: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut items: Vec<PathBuf> = Vec::new();
    let entries: ReadDir = read_dir(path)?;

    for entry in entries {
        let path: PathBuf = entry?.path();
        if path.is_dir() {
            items.extend(list_items(&path)?);
            items.push(path);
        } else if path.is_file() {
            items.push(path);
        }
    }
    Ok(items)
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let input_path: PathBuf = absolute(args.input)?;
    let output_path: PathBuf = absolute(args.output)?;
    let manifest_path: PathBuf = absolute(args.manifest)?;

    assert!(input_path.exists(), "Input path does not exist");
    assert!(!output_path.exists(), "Output path already exists");
    assert!(manifest_path.exists(), "Manifest path does not exist");

    let current_dir: PathBuf = absolute(Path::new("."))?;
    let temp_path: PathBuf = current_dir.join("temp");
    let mod_path: PathBuf = temp_path.join("mod");

    if temp_path.exists() {
        remove_dir_all(&temp_path)?;
    }
    create_dir(&temp_path)?;
    create_dir(&mod_path)?;

    copy(&manifest_path, mod_path.join("manifest"))?;

    #[cfg(target_os = "windows")]
    let build_romfs_path = temp_path.join("build_romfs.exe");
    #[cfg(target_os = "windows")]
    let build_pfs0_path = temp_path.join("build_pfs0.exe");

    #[cfg(not(target_os = "windows"))]
    let build_romfs_path: PathBuf = temp_path.join("build_romfs");
    #[cfg(not(target_os = "windows"))]
    let build_pfs0_path: PathBuf = temp_path.join("build_pfs0");

    {
        let mut build_romfs_file: File = File::create(&build_romfs_path)?;
        build_romfs_file.write_all(BUILD_ROMFS_BIN)?;
        #[cfg(not(target_os = "windows"))]
        set_permissions(
            &build_romfs_path,
            std::os::unix::fs::PermissionsExt::from_mode(0o755),
        )?;

        let mut build_pfs0_file: File = File::create(&build_pfs0_path)?;
        build_pfs0_file.write_all(BUILD_PFS0_BIN)?;
        #[cfg(not(target_os = "windows"))]
        set_permissions(
            &build_pfs0_path,
            std::os::unix::fs::PermissionsExt::from_mode(0o755),
        )?;
    }

    let all_items: Vec<PathBuf> = list_items(&input_path)?;
    let files_to_copy: HashSet<&&str> = FILES_TO_COPY.iter().collect();
    for item in all_items {
        let name: &str = item.file_name().unwrap().to_str().unwrap();

        if item.is_dir() && name == "romfs" {
            println!("Found romfs directory. Building romfs.bin...");
            std::process::Command::new(&build_romfs_path)
                .current_dir(&mod_path)
                .arg(&item)
                .arg(&mod_path.join("romfs.bin"))
                .status()?;
            continue;
        }

        if files_to_copy.contains(&&name) {
            println!("Found {}, copying...", name);
            copy(&item, &mod_path.join(name))?;
            continue;
        }

        if name.ends_with(".ips") {
            println!("Found {}, copying...", name);
            copy(&item, &mod_path.join(name))?;
            continue;
        }
    }

    println!("Building {}...", output_path.display());
    std::process::Command::new(&build_pfs0_path)
        .current_dir(&mod_path)
        .arg(&mod_path)
        .arg(&output_path)
        .status()?;

    remove_file(&build_romfs_path)?;
    remove_file(&build_pfs0_path)?;
    remove_dir_all(&temp_path)?;

    Ok(())
}
