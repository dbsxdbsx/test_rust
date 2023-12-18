use gstreamer_player::{Player, PlayerSignalDispatcher, PlayerVideoRenderer};
use std::path::Path;
use urlencoding::encode;
// use gst;
use gstreamer_player as gst_play;

use gstreamer::prelude::*;
use gstreamer::ClockTime;
use gstreamer::ElementFactory;

pub struct MediaController {
    player: ChannelPlayer,
}

impl MediaController {
    pub fn new(media_url: &str, subtitle_url: &str) -> anyhow::Result<Self> {
        let sender = glib::Sender::new(|_| {});
        let player = ChannelPlayer::new(sender, None)?;

        player.load_uri(media_url);
        player.configure_subtitle_track(Some(SubtitleTrack::Uri(subtitle_url.to_string())));

        Ok(Self { player })
    }

    pub fn set_volume(&self, volume: f64) {
        self.player.set_volume(volume);
    }

    pub fn get_total_duration(&self) -> Option<gst::ClockTime> {
        self.player.duration()
    }

    pub fn get_current_duration(&self) -> Option<gst::ClockTime> {
        self.player.get_position()
    }

    // Other methods...
}

pub fn test(url: &str, subtitle: &str) {
    gstreamer::init().unwrap();

    let player = Player::new(None::<PlayerVideoRenderer>, None::<PlayerSignalDispatcher>);

    let url = get_valid_url(url);
    let subtitle = get_valid_url(subtitle);
    player.set_uri(Some(&url));
    player.set_subtitle_uri(Some(&subtitle));

    player.connect_error(|_, err| {
        eprintln!("Error: {:?}", err);
    });

    player.connect_state_changed(|_, new_state| {
        println!("State changed to {:?}", new_state);
    });

    // 阻塞主线程，直到播放结束
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    player.stop();
}

/// 播放带有字幕的媒体
///
/// # 参数
///
/// * `url` - 媒体的URL
/// * `subtitle` - 字幕的URL
// pub fn test(url: &str, subtitle: &str) {
//     gstreamer::init().unwrap();

//     let player = Player::new(None, None);
//     let url = get_valid_url(url);
//     let subtitle = get_valid_url(subtitle);
//     player.set_uri(Some(&url));
//     player.set_subtitle_uri(Some(&subtitle));

//     // 调整音量
//     player.set_volume(0.5);

//     player.connect_state_changed(|_, new_state| {
//         println!("State changed to {:?}", new_state);
//     });

//     player.connect_position_updated(|_, position| {
//         // 显示当前播放时间
//         println!("Position updated to {:?}", position);
//     });

//     player.connect_duration_changed(|_, duration| {
//         // 显示总时长
//         println!("Duration changed to {:?}", duration);
//     });

//     player.connect_video_dimensions_changed(|_, width, height| {
//         // 显示视频的宽度和高度
//         println!("Video dimensions changed to {}x{}", width, height);
//     });

//     player.play();

//     let bus = player.bus().unwrap();

//     while let Some(msg) = bus.timed_pop(gstreamer::ClockTime::NONE) {
//         use gstreamer::MessageView;

//         match msg.view() {
//             MessageView::Eos(..) => break,
//             MessageView::Error(err) => {
//                 eprintln!(
//                     "Error from {:?}: {} ({:?})",
//                     err.src().map(|s| s.path_string()),
//                     err.error(),
//                     err.debug()
//                 );
//                 break;
//             }
//             _ => (),
//         }
//     }

//     player.stop();
// }

/// 用于视频或字幕文件路径转换为gstreamer可接受的url路径
fn get_valid_url(path_str: &str) -> String {
    let is_local_file = Path::new(path_str).exists();
    let url = if is_local_file {
        // 如果URL指向本地文件，我们需要将文件路径转换为URL
        let path = std::path::PathBuf::from(path_str);
        let file_url = format!("file:///{}", encode(&path.to_string_lossy()));
        file_url.replace("\\", "/")
    } else {
        path_str.to_string()
    };
    url
}
