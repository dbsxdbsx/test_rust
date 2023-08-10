mod image_tool;
use crate::image_multi::image_tool::Image;
use minifb::{Key, Window, WindowOptions};

#[derive(PartialEq, Debug, Clone)]
pub enum ImageType {
    Screenshot { x: i32, y: i32, w: usize, h: usize },
    ImagePath(String),
}
//↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓test↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
fn show_image(image: &Image, width: u32, height: u32, window: &mut Window) {
    let buffer = image.get_buffer_u32();
    window
        .update_with_buffer(&buffer, width as usize, height as usize)
        .unwrap();
}

// I want user use this method directly,
pub fn test_show_image_3(title: Option<String>, image_type: ImageType) {
    let image = match image_type {
        ImageType::Screenshot { x, y, w, h } => Image::from_screen(x, y, w, h),
        ImageType::ImagePath(path) => Image::from_path(path),
    };
    let (width, height) = image.get_dims();

    // better not reinitial the window every time the func is called
    let mut window = Window::new(
        &title.unwrap_or_default(),
        width as usize,
        height as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // the code stuck here
    // while window.is_open() && !window.is_key_down(Key::Escape) {
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // TODO: Setup new image here
        show_image(&image, width, height, &mut window);
    }
}

//↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑test↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑

// 以下为原始的代码
// pub fn test_show_image(title: Option<String>, image_type: ImageType) {
//     let image = match image_type {
//         ImageType::Screenshot { x, y, w, h } => Image::from_screen(x, y, w, h),
//         ImageType::ImagePath(path) => Image::from_path(path),
//     };
//     let (width, height) = image.get_dims();

//     // Pass the WINDOW variable to the show_window function
//     show_window(title, width, height, image, Arc::clone(&WINDOW));
// }

// fn show_window(
//     title: Option<String>,
//     width: u32,
//     height: u32,
//     image: Image,
//     window_arc: Arc<Mutex<Option<Window>>>,
// ) {
//     let mut window_guard = window_arc.lock().unwrap();

//     if window_guard.is_none() {
//         let new_window = Window::new(
//             &title.clone().unwrap_or_default(),
//             width as usize,
//             height as usize,
//             WindowOptions {
//                 resize: true,
//                 ..WindowOptions::default()
//             },
//         )
//         .unwrap();
//         *window_guard = Some(new_window);
//     }

//     let window = window_guard.as_mut().unwrap();

//     let buffer = image.get_buffer_u32();

//     window
//         .update_with_buffer(&buffer, width as usize, height as usize)
//         .unwrap();

//     // Drop the window_guard to unlock the Mutex
//     drop(window_guard);
// }
