# Simple File Organizer

A simple file organizer written in Rust.

I made this because my Downloads folder was a complete mess and I got tired of organizing it by hand.

## What It Does

Turns this:

```text
Downloads/
├── image.png
├── song.mp3
├── movie.mp4
└── document.pdf
```

Into this:

```text
Pictures/image.png
Music/song.mp3
Videos/movie.mp4
Documents/document.pdf
```

The program scans a directory, detects supported file types, and moves them to the appropriate system folders.

## Building

### Linux / macOS / BSD

Run:

```bash
./Build.sh
```

### Windows

Run:

```bat
Build.bat
```

## Usage

### Verbose Mode

Use `-v` to enable verbose output.

```bash
organize -v ~/Pictures
```

### Specifying a Directory

You can provide a directory before or after `-v`.

Examples:

```bash
organize ~/Pictures
```

```bash
organize -v ~/Pictures
```

### Default Behavior

If no directory is specified, the program will use your Downloads folder.

```bash
organize
```

## Supported File Types

### Images

* png
* jpg
* jpeg
* webp
* gif
* tif
* tiff
* bmp
* svg

### Audio

* mp3
* wav
* aiff
* flac
* alac
* aac

### Videos

* mp4
* avi
* mov
* wmv
* flv

### Documents

* iso
* txt
* pdf
* doc
* docx
* rtf
* odt

Anything else is ignored.

## Platform Support

Tested on Linux and macOS.

Windows support is currently untested. If someone wants to make it work properly on Windows, pull requests are welcome.

## Todo

* [ ] Maybe update this someday
* [ ] Or maybe not :D

## License

This project is licensed under the DO WHATEVER THE FUCK YOU WANT WITH IT LICENSE (DWWYWWI).

See the LICENSE file for details.
