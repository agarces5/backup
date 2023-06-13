use std::{path::PathBuf, process::Command};

use clap::Parser;

/// Simple program to backup directories.
/// Enter full origin/dest path.
/// Example: -o "\\PC-01\\C$\Users\user-01" -d "\\PC-01-NEW\C$\Users\user-01" *.txt
#[derive(Parser, Debug)]
#[command(
    help_template = "{about-section} \n {usage-heading} {usage} \n {all-args} {tab} \n\n Autor: {author-with-newline}"
)]
#[command(author = "Antonio Garcés", version, about, long_about)]
struct Args {
    /// Path with withspaces should be quoted ("Ej. \\PC00000\C$\Users o "C:\Program Files"")
    #[arg(short, long)]
    origen: String,

    /// Path with withspaces should be quoted ("Ej. \\PC00000\C$\Users o "C:\Program Files"")
    #[arg(short, long)]
    destino: String,

    /// File to copy (name/wildcard: by default "*.*")
    file: Option<String>,
}

fn make_backup(args: Args) {
    let origen = PathBuf::from(&args.origen);
    let destino = PathBuf::from(&args.destino);
    let mut vec_args = vec![origen.to_str().unwrap()];
    vec_args.push(destino.to_str().unwrap());
    if let Some(file) = &args.file {
        vec_args.push(file);
    }
    vec_args.extend_from_slice(&["/Z", "/COPYALL", "/R:2", "/W:2", "/MIR"]);

    Command::new("robocopy")
        .args(vec_args)
        .status()
        .expect("Failed to execute process");
}

fn main() {
    let args = Args::parse();
    make_backup(args);
}
