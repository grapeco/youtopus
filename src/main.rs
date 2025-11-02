use std::{
    fs::File, io::{self, BufRead, BufReader}, process::Command
};

fn run_command(args: Vec<&str>) {
    Command::new(args[0])
        .args(&args[1..])
        .status()
        .expect(format!("Failed to execute command{:#?}", &args).as_str());
}

fn download(url: &str, path: &str, output: &str, format: &str) {
    let mut args = vec![
        "yt-dlp",
        "-o",
        "%(title)s.%(ext)s",
        url,
    ];

    match output {
        "audio" => {
            args.push("-x");
            if !format.is_empty() {
                args.push("--audio-format");
                args.push(format);
            }
        }
        "video" => {
            if !format.is_empty() {
                args.push("-f");
                args.push(format);
            }
        }
        _ => panic!("Wrong media type")
    }

    if !path.is_empty() {
        args.push("-P");
        args.push(path);
    }

    run_command(args);
}

fn args_in_file(file: &File) -> Vec<Option<String>> {
    let reader = BufReader::new(file);
    let mut args: Vec<Option<String>> = vec![None, None, None];

    for line in reader.lines() {
        let line = line.unwrap();

        if !line.is_empty() {
            args.push(Some(line));
        } else {
            args.push(None);
        }
    }

    args
}

fn main() {
    let mut url = String::new();
    println!("Enter video url(Youtube, Rutube, Vk video):");
    io::stdin().read_line(&mut url).unwrap();

    let file = File::open("args.txt").expect("Could not open file");
    let file_vec = args_in_file(&file);

    let mut vec: Vec<String> = Vec::new();

    // Path
    match file_vec[0].clone() {
        Some(value) => vec.push(value),
        None => {
            let mut buf = String::new();
            println!("Enter your path(you can leave this field empty)");
            io::stdin().read_line(&mut buf).unwrap();
            vec.push(buf.trim().to_string());
        }   
    }

    // Output
    match file_vec[1].clone() {
        Some(value) => vec.push(value),
        None => {
            let mut buf = String::new();
            println!("Enter your media type(you can leave this field empty)");
            io::stdin().read_line(&mut buf).unwrap();
            vec.push(buf.trim().to_string());
        }
    }

    // Format
    match file_vec[2].clone() {
        Some(value) => vec.push(value),
        None => {
            match vec[1].as_str() {
                "audio" => {
                    let mut buf = String::new();
                    println!("Enter format(mp3, opus, m4a, wav, aac, alac, flac, vorbis) or leave empty");
                    io::stdin().read_line(&mut buf).unwrap();

                    vec.push(buf.trim().to_string());
                }
                "video" => {
                    let mut buf = String::new();
                    println!("Wanna see all formats for video?(yes or no)");
                    io::stdin().read_line(&mut buf).unwrap();

                    if buf.trim() == "yes" {
                        run_command(vec![
                            "yt-dlp",
                            "-F",
                            &url,
                        ]);
                        
                        buf = String::new();
                        println!("Enter your format code or leave empty");
                        io::stdin().read_line(&mut buf).unwrap();

                        vec.push(buf.trim().to_string());
                    } else if buf.trim() == "no" {
                        buf = String::new();
                        println!("Enter your format code or leave empty");
                        io::stdin().read_line(&mut buf).unwrap();

                        vec.push(buf.trim().to_string());
                    }
                }
                _ => panic!("Wrong!!!")
            }
        }
    }

    println!("{:?}", vec);

    download(&url, &vec[0], &vec[1], &vec[2]);
}
