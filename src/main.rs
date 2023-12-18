mod test_gstreamer;
fn main() {
    let url =
        "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm";
    // let url = "https://jmc-live.ercdn.net/alistiqama/alistiqama.m3u8";
    // let url = r"D:\MEDIA\VIDEO\小露一手.mp4";
    let url = r"D:\MEDIA\VIDEO\中文\夏颖瑜.mp4";
    // let url = format!("file:///{}", url.replace("\\", "/"));

    // let url = r"D:\MEDIA\VIDEO\ttt.mp4";
    let url = r"D:\MEDIA\VIDEO\喧嚣学院\[アニメ DVD] スクールランブル ♯06「放課後のサバイバル！ 告白の時、アライバル！ 二人きりのホスピタル！」(720p x264 10bit AAC chap) .mp4";
    let subtitle = r"D:\MEDIA\VIDEO\喧嚣学院\[アニメ DVD] スクールランブル ♯06「放課後のサバイバル！ 告白の時、アライバル！ 二人きりのホスピタル！」(720p x264 10bit AAC chap) .ssa";
    test_gstreamer::test(url, subtitle);
}
