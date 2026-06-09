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
    let args = Args::parse();   
    let directory = args.directory.map(PathBuf::from)
        .unwrap_or_else(|| dirs::download_dir().expect("No downloads dir"));
    start_organizer( directory, args.verbose);
    
}

fn start_organizer(directory: PathBuf, verbose: bool) {
    if verbose {
        println!("start file organizer");
    }
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
            println!("try sort file: {}", filepath.to_string_lossy());
        }
        
        if name.ends_with(".png")
            || name.ends_with(".jpg")
            || name.ends_with(".jpeg")
            || name.ends_with(".webp")
            || name.ends_with(".gif")
            || name.ends_with(".tif")
            || name.ends_with(".tiff")
            || name.ends_with(".bmp")
            || name.ends_with(".svg")
        {
            let path = dirs::picture_dir()
                .expect("No picture dir")
                .join(&name);

            fs::rename(&filepath, &path)
                .expect("Failed to move file");

            if verbose {
                let path_name = path.to_string_lossy();
                let moved_file_path = filepath.to_string_lossy();
                println!("Moved file {moved_file_path} to {path_name}");
            }
        }
        if name.ends_with(".mp3")
            || name.ends_with(".wav")
            || name.ends_with(".aiff")
            || name.ends_with(".flac")
            || name.ends_with(".alac")
            || name.ends_with(".aac")
        {
            let path = dirs::audio_dir()
                .expect("No audio dir")
                .join(&name);

            fs::rename(&filepath, &path)
                .expect("Failed to move file");

            if verbose {
                let path_name = path.to_string_lossy();
                let moved_file_path = filepath.to_string_lossy();
                println!("Moved file {moved_file_path} to {path_name}");
            }
        }
        if name.ends_with(".mp4")
            || name.ends_with(".avi")
            || name.ends_with(".mov")
            || name.ends_with(".wmv")
            || name.ends_with(".flv")
        {
            let path = dirs::video_dir()
                .expect("No video dir")
                .join(&name);

            fs::rename(&filepath, &path)
                .expect("Failed to move file");

            if verbose {
                let path_name = path.to_string_lossy();
                let moved_file_path = filepath.to_string_lossy();
                println!("Moved file {moved_file_path} to {path_name}");
            }
        }
        if name.ends_with(".iso")
            || name.ends_with(".txt")
        {
            let path: PathBuf = dirs::document_dir()
                .expect("No document dir")
                .join(&name);

            fs::rename(&filepath, &path)
                .expect("Failed to move file");

            if verbose {
                let path_name = path.to_string_lossy();
                let moved_file_path = filepath.to_string_lossy();
                println!("Moved file {moved_file_path} to {path_name}");
            }
        }
    }
}