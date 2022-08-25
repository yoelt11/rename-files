#![allow(unused)]
// Disable unused code warning
use clap::Parser;
use std::ffi::OsStr;
use std::fs;
use rayon::prelude::*;
use std::path::PathBuf;
use indicatif::ProgressBar;
// modules
mod single_rename;

#[derive(Parser)]
#[clap(author="Edgar Torres, https://github.com/yoelt11", 
       version, 
       about="Renames all files in a directory", 
       long_about = "Renames all files in a directory. Mainly used for datasets
       where you might want to have all files in a foler with a certain structure
       or extension")]
struct Cli {
    /// The new name of the files
    #[clap(short, long, default_value="")]
    pattern: String,
    /// The inital conscutive number
    #[clap(short, long, default_value_t=1)]
    start_n: i32,
    /// The new file extension
    #[clap(short, long, default_value="", help="no-ext to remove extension")]
    extension: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

//TODO: add date option and parallelize
fn main() {
    // Parse the arguments into the cli sructure
    let args = Cli::parse();
    let file_list = fs::read_dir(&args.path).unwrap();
    let file_count = fs::read_dir(&args.path).unwrap().count();
    // starting file number
    let mut count = args.start_n;
    // create temporary directory
    fs::create_dir("../tmp_file_rename/");
    let progres_bar = ProgressBar::new(file_count.try_into().unwrap());
    file_list.into_iter()
             .enumerate()
             .for_each(|(i,f)| {
                       single_rename::rename(&args.pattern, count + (i as i32),
                                            &f.unwrap()
                                            .path(), &args.extension);
                        progres_bar.inc(1)});
    
    // rename temporary folder to original folder and delete temporary folder
    fs::rename("../tmp_file_rename/", args.path );
    fs::remove_dir_all("../tmp_file_rename/");
    progres_bar.finish_with_message("done");
}
