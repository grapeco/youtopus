use std::{
    fs::File, io::{self, BufRead, BufReader}, process::Command
};

fn run_command(args: Vec<&str>) {
    Command::new(args[0])
        .args(&args[1..])
        .status()
        .expect(format!("Failed to execute command{:#?}", &args).as_str());
}

fn download(url: &str, path: &str, media_type: &str, format: &str) {
    let mut args = vec![
        "yt-dlp",
        "--check-formats",
        "-o",
        "%(title)s.%(ext)s",
        url,
    ];

    match media_type {
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

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if !line.is_empty() {
            args[i] = Some(line);
        } else {
            args[i] = None;
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

    let mut args: Vec<String> = Vec::new();

    println!("{:?}", file_vec);

    // Path
    match file_vec[0].clone() {
        Some(value) => args.push(value),
        None => args.push("".to_string()),  
    }

    // Media type
    match file_vec[1].clone() {
        Some(value) => args.push(value),
        None => panic!("Fill the file!!!")
    }

    // Format
    match file_vec[2].clone() {
        Some(value) => {
            if args[1] == "video" && value == "custom" {
                run_command(vec![
                    "yt-dlp",
                    "-F",
                    &url,
                ]);

                let mut buf = String::new();
                println!("Enter your format or leave empty");
                io::stdin().read_line(&mut buf).unwrap();

                args.push(buf.trim().to_string());
            } else {
                args.push(value);
            }
        }
        None => args.push("".to_string())
    }

    println!("{:?}", args);

    download(&url, &args[0], &args[1], &args[2]);
}
