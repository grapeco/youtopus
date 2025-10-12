use std::{io, process::Command, time::Instant};

fn run_command(args: Vec<&str>) {
    Command::new(args[0])
        .args(&args[1..])
        .status()
        .expect(format!("Failed to execute command{:#?}", &args).as_str());
}

fn download_video(url: &str, format: &str, path: &str) {
    let start_time = Instant::now();

    let mut args = vec![
        "yt-dlp",
        "-f",
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

    println!("Video download complete in {:.2?}", start_time.elapsed());
}

fn download_audio(url: &str, format: &str, path: &str) {
    let start_time = Instant::now();

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

    println!("Audio download complete in {:.2?}", start_time.elapsed());
}

fn main() {
    let mut audio_format = String::new();
    let mut video_format = String::new();

    let mut url = String::new();
    println!("Enter video url(Youtube, Rutube, Vk video):");
    io::stdin().read_line(&mut url).unwrap();

    let mut path = String::new();
    println!("Enter path to install video(you can leave this field empty):");
    io::stdin().read_line(&mut path).unwrap();

    let mut choise = String::new();
    println!("What you want to install(video, audio):");
    io::stdin().read_line(&mut choise).unwrap();

    match choise.trim() {
        "audio" => {
            println!("Enter audio format(mp3, opus, etc):");
            io::stdin().read_line(&mut audio_format).unwrap();

            download_audio(&url, &audio_format, &path);
        } 
        "video" => {
            run_command(vec![
                "yt-dlp",
                "-F",
                &url,
            ]);

            println!("Enter video format code:");
            io::stdin().read_line(&mut video_format).unwrap();

            download_video(&url, &video_format, &path);
        }
        _ => {}
    }
}
