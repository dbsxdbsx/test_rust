mod image_tool;

use image_tool::Image;
use show_image::{create_window, ImageInfo, ImageView};

#[derive(PartialEq, Debug, Clone)]
pub enum ImageType {
    Screenshot { x: i32, y: i32, w: usize, h: usize },
    ImagePath(String),
}

pub fn test_show_image(title: Option<String>, image_type: ImageType) {
    let image = match image_type {
        ImageType::Screenshot { x, y, w, h } => Image::from_screen(x, y, w, h),
        ImageType::ImagePath(path) => Image::from_path(path),
    };
    let (width, height) = image.get_dims();
    let buffer = image.get_buffer_u8();

    let raw = buffer.into_raw();
    let image_view = ImageView::new(ImageInfo::rgb8(width, height), &raw);

    let window = create_window(title.unwrap_or_default(), Default::default()).unwrap();
    window.set_image("image", image_view).unwrap();
}

// mod image_tool;
// use std::cell::RefCell;

// use crate::image_multi::image_tool::Image;
// use minifb::{Key, Window, WindowOptions};

// #[derive(PartialEq, Debug, Clone)]
// pub enum ImageType {
//     Screenshot { x: i32, y: i32, w: usize, h: usize },
//     ImagePath(String),
// }

// thread_local! {
//     static WINDOW: RefCell<Option<Window>> = RefCell::new(None);
// }

// pub fn test_show_image(title: Option<String>, image_type: ImageType) {
//     let image = match image_type {
//         ImageType::Screenshot { x, y, w, h } => Image::from_screen(x, y, w, h),
//         ImageType::ImagePath(path) => Image::from_path(path),
//     };
//     let (width, height) = image.get_dims();

//     WINDOW.with(|window_cell| {
//         let mut window_opt = window_cell.borrow_mut();
//         let should_close = if let Some(window) = window_opt.as_mut() {
//             // Update window info
//             window.set_title(&title.unwrap_or_default());
//             // TODO: window.set_size(width as usize, height as usize);

//             // Directly use the window variable
//             if window.is_open() && !window.is_key_down(Key::Escape) {
//                 show_image(&image, width, height, window);
//                 false
//             } else {
//                 true
//             }
//         } else {
//             let new_window = Window::new(
//                 &title.unwrap_or_default(),
//                 width as usize,
//                 height as usize,
//                 WindowOptions {
//                     resize: true, // resizable by mouse
//                     ..WindowOptions::default()
//                 },
//             )
//             .unwrap_or_else(|e| {
//                 panic!("{}", e);
//             });
//             *window_opt = Some(new_window);
//             false
//         };

//         if should_close {
//             // close the window safely
//             drop_window();
//         }
//     });
// }

// fn show_image(image: &Image, width: u32, height: u32, window: &mut Window) {
//     let buffer = image.get_buffer_u32();
//     window
//         .update_with_buffer(&buffer, width as usize, height as usize)
//         .unwrap();
// }

// fn drop_window() {
//     WINDOW.with(|window_cell| {
//         let mut window_opt = window_cell.borrow_mut();
//         if let Some(window) = window_opt.take() {
//             drop(window);
//         }
//     });
// }

// //↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑test↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑
