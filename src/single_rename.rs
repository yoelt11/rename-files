use std::path::PathBuf;
use std::ffi::OsStr;
use std::fs;

pub fn rename(pattern:&String, idx:i32, fulldir:&PathBuf, ext:&String){
    let mut outdir = PathBuf::from("../tmp_file_rename/name");
    
    // if a new pattern is given set it as file name
    if pattern.len() > 0 {
        outdir.set_file_name(pattern.to_owned() +"_"+ idx.to_string().as_str());
    } else {
        let mut new_name = fulldir.file_name().unwrap();
        outdir.set_file_name(new_name);
    }
    
    // initialize file_extension variable
    let mut file_extension = "";
   
    // if extension is given, this replaces extension
    if ext.len() > 0 {
        file_extension = ext;
    } else if !fulldir.extension().is_none(){
        // else keep original extension
        file_extension = fulldir.extension().and_then(OsStr::to_str).unwrap();
    }

    if !file_extension.eq(&String::from("no-ext")){
        outdir.set_extension(file_extension); // replace extension
    } else {
        outdir.set_extension(""); // replace extension
    }
    fs::copy(fulldir, outdir);  
    fs::remove_file(fulldir);
}
