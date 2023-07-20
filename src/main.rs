mod test_show_image;
fn main() {
    let image_path =
        r#"D:\DATA\BaiduSyncdisk\project\personal\test_repo\test_rust\images\16.bmp"#;
        // r#"D:\DATA\BaiduSyncdisk\project\personal\test_repo\test_rust\myimage_gray.png"#;
    test_show_image::test_show_image(image_path);
    // // test_minist::test_minist();
    // test_winapi_text::create_window("标题", "你好");

    // test_windows_image::create_window("标题", "你好", Some(&image_path));
    // test_winapi_image::create_window("标题", "你好", Some(&image_path));
    // Do something with the screenshot...

    //
    // test_opencv::test_opencv();
}

// use show_image::{create_window, ImageInfo, ImageView, event};
// #[show_image::main]
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // read image from file
//     let img = image::open("./xly.png")?;
//     let img = img.into_rgb8();
//     let image = ImageView::new(ImageInfo::rgb8(1920, 1080), &img);

//     // // Create a window with default options and display the image.
//     // let window = create_window("image", Default::default())?;
//     // window.set_image("image-001", image)?;

//     // Create a window and display the image.
//     let window = create_window("image", Default::default())?;
//     window.set_image("image-001", &image)?;

//     // Print keyboard events until Escape is pressed, then exit.
//     // If the user closes the window, the channel is closed and the loop also exits.
//     for event in window.event_channel()? {
//         if let event::WindowEvent::KeyboardInput(event) = event {
//             println!("{:#?}", event);
//             if event.input.key_code == Some(event::VirtualKeyCode::Escape)
//                 && event.input.state.is_pressed()
//             {
//                 break;
//             }
//         }
//     }

//     Ok(())
// }

// use show_image::{create_window, event, ImageInfo, ImageView};
// // #[show_image::main]
// fn main() {
//     ::show_image::run_context(user_main);
// }

// fn user_main() -> Result<(), Box<dyn std::error::Error>> {
//     // read image from file
//     let img = image::open("./xly.png")?;
//     let img = img.into_rgb8();
//     let image = ImageView::new(ImageInfo::rgb8(1920, 1080), &img);

//     // // Create a window with default options and display the image.
//     // let window = create_window("image", Default::default())?;
//     // window.set_image("image-001", image)?;

//     // Create a window and display the image.
//     let window = create_window("image", Default::default())?;
//     window.set_image("image-001", &image)?;

//     // Print keyboard events until Escape is pressed, then exit.
//     // If the user closes the window, the channel is closed and the loop also exits.
//     for event in window.event_channel()? {
//         if let event::WindowEvent::KeyboardInput(event) = event {
//             println!("{:#?}", event);
//             if event.input.key_code == Some(event::VirtualKeyCode::Escape)
//                 && event.input.state.is_pressed()
//             {
//                 break;
//             }
//         }
//     }

//     Ok(())
// }
