use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

// This function needed to make getting user input more convenient
// query parameter is a text we printing to console when asking user for input
fn get_input(query: &str) -> std::io::Result<String> {
    print!("{}", query);
    std::io::stdout().flush()?;

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    // returning String without redundant spaces and transitions to a new line
    Ok(buffer.trim().to_owned())
}

// This function checks is here a dir at the given path and
// organizes it by file extensions if is
fn organize_dir(dir_path: PathBuf) {
    if !dir_path.exists() {
        println!("Dir \"{}\" doesn't exist", dir_path.display());
    } else {
        let dir_files = match dir_path.read_dir() {
            Ok(files) => files,
            Err(err) => {
                println!("Error reading dir \"{}\": \"{}\"", dir_path.display(), err);
                return;
            }
        };

        for file in dir_files {
            if let Ok(file) = file {
                // We shouldn't organize nested dirs
                if file.path().is_dir() {
                    println!("Path {} is dir, skip", file.path().display());
                    continue;
                }

                // We should get a file extension to decide how
                // to organize it
                let file_extension = match file.path().extension() {
                    Some(extension) => match extension.to_str() {
                        None => continue,
                        Some(extension) => extension.to_lowercase()
                    }
                    None => {
                        println!("Error getting extension of \"{}\"", file.path().display());
                        continue;
                    }
                };

                // Constructing final path to move a file
                let extension_dir = PathBuf::from(dir_path.join(file_extension));

                // Organizing file
                create_dir_if_not_exists(&extension_dir);
                move_file(&file.path(), &extension_dir.join(file.file_name()));
            }
        }
    }
}

// This function checks is here a dir at the given path
// and creates it if not
fn create_dir_if_not_exists(dir: &PathBuf) {
    if !dir.exists() {
        if let Err(err) = fs::create_dir(dir) {
            println!("Error creating dir \"{}\": \"{}\"", dir.display(), err);
        }
    }
}

// This function moves file in specified directory
// and handles possible errors
fn move_file(from: &PathBuf, to: &PathBuf) {
    if let Err(err) = fs::rename(from, to) {
        println!("Error moving file \"{}\" to \"{}\": \"{}\"",
                 from.display(),
                 to.display(),
                 err
        );
    }
}

// Program entry point
fn main() {
    loop {
        let dir_path = match get_input("Enter a path to the dir you want to organize: ") {
            Ok(dir_path) => dir_path,
            Err(err) => {
                println!("Error getting input: {}", err);
                continue;
            }
        };

        // Time counter
        let now = Instant::now();
        organize_dir(PathBuf::from(dir_path));

        // Printing time elapsed for files organizing
        println!("Time to organize: {}\n", now.elapsed().as_secs_f64());
    }
}
