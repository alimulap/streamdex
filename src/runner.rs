use std::process::{Command, Stdio};

pub fn with_ytdlp(
    url: String,
    res: String,
    room: Option<&String>,
    wait_for_video: bool,
    range: Option<String>,
) {
    println!("Running with yt-dlp + vlc");
    #[allow(clippy::zombie_processes)]
    let mut ytdlp = Command::new("yt-dlp")
        .arg(url)
        .arg("-f")
        .arg(res)
        .arg("-q")
        .arg("-4")
        .arg("--cookies-from-browser")
        .arg("edge")
        .arg("--mark-watched")
        .args(match wait_for_video {
            true => vec!["--wait-for-video", "5"],
            false => vec!["--no-wait-for-video"],
        })
        .arg("--downloader")
        .arg("ffmpeg")
        .args(match range {
            Some(range) => vec!["--download-sections".to_owned(), range],
            None => vec![],
        })
        .arg("-o")
        .arg("-")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let vlc = match room {
        Some(room_id) => Command::new("cvlc")
            .arg("-")
            .stdin(Stdio::from(ytdlp.stdout.take().unwrap()))
            .arg("--sout")
            .arg(format!("#transcode{{vcodec=mp4v,vb=2048,acodec=mp4a,ab=128,channels=2,scale=1}}:standard{{access=http,mux=ts,dst=localhost:8080/streamdex-{}.mp4}}", room_id))
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start cvlc"),
        None => Command::new("vlc")
            .arg("-")
            .stdin(Stdio::from(ytdlp.stdout.take().unwrap()))
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start vlc"),
    };
    let output = vlc.wait_with_output().unwrap();
    let result = String::from_utf8(output.stdout).unwrap();
    println!("{}", result);
    ytdlp.kill().unwrap();
}

pub fn only_vlc(url: String, res: String, room: Option<&String>) {
    println!("Running with only vlc");
    let vlc = match room {
        Some(room_id) => Command::new("cvlc")
            .arg(url)
            .arg("--preferred-resolution")
            .arg(res)
            .arg("--sout")
            .arg(format!("#transcode{{vcodec=mp4v,vb=2048,acodec=mp4a,ab=128,channels=2,scale=1}}:standard{{access=http,mux=ts,dst=localhost:8080/streamdex-{}.mp4}}", room_id))
            .spawn()
            .unwrap(),
        None => Command::new("vlc")
            .arg("--preferred-resolution")
            .arg(res)
            .arg(url)
            .spawn()
            .unwrap(),
    };
    let output = vlc.wait_with_output().unwrap();
    let result = String::from_utf8(output.stdout).unwrap();
    println!("{}", result);
}

#[test]
fn test_stream() {
    let mut process = Command::new("vlc")
        .arg("http://localhost:8080")
        .arg("--loop")
        .arg("vlc://pause:10")
        .spawn()
        .unwrap();
    process.wait().unwrap();
}
