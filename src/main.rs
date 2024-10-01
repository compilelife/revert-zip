use std::{fs, io::{self, BufReader}, path::PathBuf, process::exit};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The path to the zip file that needs to be reverted
    #[arg(required = true, index = 1)]
    path: String,

    /// Perform a dry run without making any changes
    #[arg(long)]
    confirm: bool,
}

fn delete_file_or_dir(path: &PathBuf)->io::Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}

fn main() {
    let cli = Cli::parse();
    let first = cli.path;
    let confirm = cli.confirm;
    if !confirm {
        println!("running in dry run mode, use --comfirm to delete");
    }

    let abs_path = PathBuf::from(&first).canonicalize().unwrap_or_else(|err| {
        println!("invalid path: {err}");
        exit(-1);
    });

    let parent = abs_path.parent().unwrap_or_else(||{
        println!("path has no parent: {:?}", abs_path);
        exit(-1);
    });

    let file = fs::File::open(&first).unwrap_or_else(|err| {
        println!("failed to open zip file: {}", err);
        exit(-1);
    });

    let reader = BufReader::new(file);
    let archive = zip::ZipArchive::new(reader).unwrap_or_else(|err|{
        println!("failed to read archive: {}", err);
        exit(-1);
    });

    let top_level = archive.file_names().filter(|fname|{
        match fname.find('/') {
            None => true,
            Some(pos) => pos == fname.len() - 1
        }
    });

    top_level.for_each(|name| {
        let path = parent.join(name);
        print!("rm {:?}......", path);
        if confirm {
            if let Err(e) = delete_file_or_dir(&path) {
                print!("failed [{e}]\n");
            } else {
                print!("success \n");
            }
        } else {
            print!("\n");
        }
    });
}