use std::fs;
use std::io;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    verbose: bool,

    directory: Option<String>,
}

fn main() {
    println!("Starting Organizer :D!");
    let args = Args::parse();   
    let directory = args.directory.map(PathBuf::from)
        .unwrap_or_else(|| dirs::download_dir().expect("No downloads dir"));
    start_organizer( directory, args.verbose);
    
}

fn start_organizer(directory: PathBuf, verbose: bool) {
    if let Err(e) = sort_files(directory, verbose) {
        eprintln!("Error: {}", e);
    }
}


fn sort_files(directory: PathBuf, verbose: bool) -> io::Result<()> {
    

    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(name) = path.file_name() {
            let name = name.to_string_lossy().to_string();
            sort_file(name, path, verbose);
        }
    }

    Ok(())
}


fn sort_file(name: String, filepath: PathBuf, verbose: bool) {
    if !name.starts_with('.') {
        if verbose {
            println!("try sort file: {}", name);
        }
        
        if name.ends_with(".png")
            || name.ends_with(".jpg")
            || name.ends_with(".jpeg")
            || name.ends_with(".webp")
        {
            let path = dirs::picture_dir()
                .expect("No picture dir")
                .join(&name);

            fs::rename(&filepath, &path)
                .expect("Failed to move file");
        }
        if name.ends_with(".mp3")
            || name.ends_with(".wov")
        {
            let path = dirs::audio_dir()
                .expect("No audio dir")
                .join(&name);

            fs::rename(&filepath, &path)
                .expect("Failed to move file");
        }
    }
}