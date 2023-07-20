use opencv::{
    highgui::{imshow, named_window, wait_key},
    imgcodecs::imread,
};
use pyo3::*;

#[pyfunction]
pub fn test_opencv() {
    // Load an image from file
    // let img = imread("path/to/image.jpg", opencv::imgcodecs::IMREAD_COLOR).unwrap();
    let img = imread(
        // r#"C:\Users\Administrator\Desktop\除恶者封面.png"#,
        // r#"D:\DATA\BaiduSyncdisk\project\personal\test_repo\test_rust\写轮眼_128x128.png"#,
        r#"D:\DATA\BaiduSyncdisk\project\personal\test_repo\test_rust\xly.png"#,
        opencv::imgcodecs::IMREAD_COLOR,
    )
    .unwrap();

    // Create a named window to display the image
    named_window("Display window", opencv::highgui::WINDOW_NORMAL).unwrap();

    // Display the image in the window
    imshow("Display window", &img).unwrap();

    // Wait for a key press to close the window
    wait_key(0).unwrap();
}
