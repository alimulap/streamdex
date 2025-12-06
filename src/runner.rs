use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

pub fn watch_with_ytdlp_and_vlc(
    url: String,
    res: String,
    // wait_for_video: WaitForVideo,
    range: Option<String>,
    print_command: bool,
) -> anyhow::Result<()> {
    if !print_command {
        println!("Running with yt-dlp + vlc");
    }
    // #[allow(clippy::zombie_processes)]
    let mut ytdlp = Command::new("yt-dlp");
    ytdlp
        .arg(url)
        .arg("-f")
        .arg(res)
        .arg("-q")
        .arg("-4")
        .arg("--cookies-from-browser")
        .arg("firefox")
        .arg("--mark-watched")
        // .args(match wait_for_video {
        //     WaitForVideo::Wait(range) => vec!["--wait-for-video".into(), range.clone()],
        //     WaitForVideo::NoWait => vec!["--no-wait-for-video".into()],
        // })
        .arg("--downloader")
        .arg("ffmpeg")
        .args(match range {
            Some(range) => vec!["--download-sections".to_owned(), range],
            None => vec![],
        })
        .arg("-o")
        .arg("-")
        .stdout(Stdio::piped());
    // .spawn()
    // .unwrap();
    let mut vlc = Command::new("vlc");
    let vlc = vlc.arg("-").stdout(Stdio::piped());
    if print_command {
        println!(
            "yt-dlp {} | vlc {}",
            ytdlp
                .get_args()
                .map(|arg| arg.to_str())
                .collect::<Option<Vec<&str>>>()
                .ok_or_else(|| anyhow::anyhow!("Failed to convert yt-dlp args to string"))?
                .join(" ")
                .to_owned(),
            vlc.get_args()
                .map(|arg| arg.to_str())
                .collect::<Option<Vec<&str>>>()
                .ok_or_else(|| anyhow::anyhow!("Failed to convert vlc args to string"))?
                .join(" ")
                .to_owned(),
        );
        Ok(())
    } else {
        let mut ytdlp = ytdlp.spawn()?;
        let mut vlc = vlc
            .stdin(Stdio::from(ytdlp.stdout.take().ok_or_else(|| {
                anyhow::anyhow!("Failed to take yt-dlp stdout")
            })?))
            .spawn()?;

        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        ctrlc::set_handler(move || {
            println!("\nReceived Ctrl+C, cleaning up...");
            r.store(false, Ordering::SeqCst);
        })?;

        loop {
            std::thread::sleep(Duration::from_secs(1));

            if !running.load(Ordering::SeqCst) {
                let _ = ytdlp.kill();
                let _ = ytdlp.wait();
                break;
            }

            if let Ok(Some(status)) = vlc.try_wait() {
                if let Some(mut out) = vlc.stdout.take() {
                    let mut buffer = String::new();
                    use std::io::Read;
                    out.read_to_string(&mut buffer)?;
                    println!("VLC output: {}", buffer);
                }
                println!("VLC exited with status: {}", status);

                let _ = ytdlp.kill();
                let _ = ytdlp.wait();

                break;
            }

            if let Ok(Some(status)) = ytdlp.try_wait() {
                if let Some(mut out) = ytdlp.stdout.take() {
                    let mut buffer = String::new();
                    use std::io::Read;
                    out.read_to_string(&mut buffer)?;
                    println!("yt-dlp output: {}", buffer);
                }
                println!("yt-dlp exited with status: {}", status);

                let _ = vlc.kill();
                let _ = vlc.wait();

                break;
            }
        }

        Ok(())
    }
}
