use std::{
    fs::File, io::{self, BufRead, BufReader}, process::Command
};

fn run_command(args: Vec<&str>) {
    Command::new(args[0])
        .args(&args[1..])
        .status()
        .expect(format!("Failed to execute command{:#?}", &args).as_str());
}

fn download_video(url: &str, format: Option<&str>, path: &str) {
    let mut args = vec![
        "yt-dlp",
        "-o",
        "%(title)s.%(ext)s",
        &url,
    ];

    match format {
        Some(value) => {
            args.push("-f");
            args.push(value);
        }
        None => {}
    }

    if !path.is_empty() {
        args.push("-P");
        args.push(path);
    }

    run_command(args);
}

fn download_audio(url: &str, format: &str, path: &str) {
    let mut args = vec![
        "yt-dlp",
        "-x",
        "--audio-format",
        format,
        "-o",
        "%(title)s.%(ext)s",
        &url,
    ];

    if !path.is_empty() {
        args.push("-P");
        args.push(path);
    }

    run_command(args);
}

fn args_in_file(file: &File) -> Vec<String> {
    let reader = BufReader::new(file);
    let mut input: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        input.push(line);
    }

    println!("{:?}", input);

    input
}

fn is_file_empty(file: &File) -> bool {
    let metadata = file.metadata().expect("File should have metadata");
    metadata.len() == 0
}

fn manual_input(url: &str) {
    let mut path = String::new();
    println!("Enter path to install video(you can leave this field empty):");
    io::stdin().read_line(&mut path).unwrap();

    let mut output = String::new();
    println!("Output file(video, audio):");
    io::stdin().read_line(&mut output).unwrap();

    match output.trim() {
        "audio" => {
            let mut audio_format = String::new();
            println!("Enter audio format(mp3, opus, m4a, wav, aac, alac, flac, vorbis):");
            io::stdin().read_line(&mut audio_format).unwrap();

            download_audio(&url, &audio_format, &path);
        } 
        "video" => {
            let mut choise_format = String::new();
            println!("Wanna see all formats for video or keep it default?(default or custom)");
            io::stdin().read_line(&mut choise_format).unwrap();

            match choise_format.trim() {
                "default" => {
                    download_video(&url, None, &path);
                }
                "custom" => {
                    run_command(vec![
                        "yt-dlp",
                        "-F",
                        &url,
                    ]);

                    let mut video_format = String::new();
                    println!("Enter video format code:");
                    io::stdin().read_line(&mut video_format).unwrap();

                    download_video(&url, Some(&video_format), &path);
                }
                _ => panic!("Wrong answer!!!")
            }
        }
        _ => {}
    }
}

fn file_input(url: &str, file: &File) {
    let file_vec = args_in_file(&file);
    let path = &file_vec[0];
    let output = &file_vec[1];
    
    match output.trim() {
        "audio" => {
            let audio_format = &file_vec[2];
            download_audio(&url, &audio_format, &path);
        }
        "video" => {
            let video_format = file_vec.get(2);

            match video_format {
                Some(value) => {
                    if value == "default" {
                        download_video(url, None, path);
                    } else {
                        download_video(url, Some(value), &path);
                    }
                }
                None => {
                    run_command(vec![
                        "yt-dlp",
                        "-F",
                        &url,
                    ]);

                    let mut format = String::new();
                    io::stdin().read_line(&mut format).unwrap();

                    download_video(url, Some(&format), &path);
                }
            }
        }
        _ => {}
    }
}

fn main() {
    let mut url = String::new();
    println!("Enter video url(Youtube, Rutube, Vk video):");
    io::stdin().read_line(&mut url).unwrap();

    let file = File::open("args.txt").expect("Could not open file");

    match is_file_empty(&file) {
        true => manual_input(&url),
        false => file_input(&url, &file),
    }
}
