use std::io::{self, stdout, stdin, Write};
use std::path::{Path, PathBuf};
use std::fs::{read, OpenOptions};
use std::error::Error;
use std::env;
use glob;
use chrono::prelude::*;

#[derive(Debug)]
struct FileToMerge {
    file_path:PathBuf,
    file_content:Vec<u8>
}

impl FileToMerge {
    fn new(file_path:PathBuf) -> Self {
        FileToMerge{file_path:file_path, file_content: vec!()}
    }

    fn set_content(&mut self, content:Vec<u8>) {
        self.file_content = content;
    }
}

fn get_stripped_stdin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    
    stdin.read_line(&mut buffer).expect("Failed to read line");
    buffer = buffer.trim_end().trim_end_matches("\"").trim_start_matches("\"").to_string();
    
    buffer
}



fn get_files_to_merge() -> Vec<FileToMerge> {
    let mut ret_file_paths:Vec<FileToMerge> = vec!();
    println!("Enter a Path to a file or a Wildcard (*) Pattern like C:\\Users\\root\\Downloads\\*");
    
    loop {

        if ret_file_paths.len() == 0 {
            print!("File Path: ");
        } 
        else if ret_file_paths.len() > 0 {
            print!("File Path or Press Enter to start merge: ");
        }
        
        stdout().flush().unwrap();

        let input = get_stripped_stdin();

        if input.is_empty() {
            if ret_file_paths.is_empty() {
                println!("Please enter at least one file path ");
            } 
            else {
                break
            }
        }
        else { // if file path is provided

            if Path::new(&input).is_file() {
                ret_file_paths.push(FileToMerge::new(PathBuf::from(&input)));
            }
            else if input.contains("*") {  // input is a glob pattern
                
                for entry in glob::glob(&input).expect("Failed to read glob pattern") {
                    match entry {
                        Ok(path) => {
                            if Path::new(&path).is_file() {
                                ret_file_paths.push(FileToMerge::new(path));
                            }
                        }
                        Err(e) => panic!("{e}"),
                    }
                }
            }
            else {
                println!("This is not a valid Path. Either input a file path or use a Wildcard (*) Pattern");
            }
            
        }
        
    }

    ret_file_paths
}


fn add_content(files_to_merge:&mut Vec<FileToMerge>) -> io::Result<()> {
    for file in files_to_merge {
        file.file_content = read(&file.file_path)?;
    }

    Ok(())
}


fn write_combined_file(files_to_merge:&Vec<FileToMerge>) -> io::Result<()> {
    let current_dir = env::current_dir().unwrap();
    let mut dest_file_path = PathBuf::new();

    loop {
        let local_time_now = Local::now().format("%d_%m_%Y_%H_%M_%S");
        print!("What should the name of the merged file be: ");
        stdout().flush().unwrap();

        // TODO: dynamic file name if no name is provided, just include the time in the filename
        let input = get_stripped_stdin();
        dest_file_path = match input.as_str() {
            "" => current_dir.join(format!("merged_file-{local_time_now}")),
            value if input.contains("\\") || input.contains("/") => PathBuf::from(value),
            value => current_dir.join(&value),
        };

        
        if dest_file_path.is_dir() {
            println!("This is a directory, you can't overwrite this path.");
        }
        else if dest_file_path.exists() {
            println!("This File already exists, please choose a new filename");
        }
        else {
            break;
        }
    }
    
    let mut merge_file = OpenOptions::new().write(true).append(true).create(true).open(&dest_file_path).unwrap();

    for file in files_to_merge {
        merge_file.write(&file.file_content)?;
    }

    println!("Successfully merged into the file: {}", &dest_file_path.display());  
    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut files_to_merge = get_files_to_merge();
    add_content(&mut files_to_merge)?;


    println!("Files that will be merged: ");
    for file in &files_to_merge {
        println!("{:?}", file.file_path);
    }
    
    write_combined_file(&files_to_merge)?;

    
    println!("Press Enter to close");
    stdin().read_line(&mut String::from(""))?;
    
    Ok(())
}
