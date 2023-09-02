mod image_tool;
use std::cell::RefCell;

use crate::image_multi::image_tool::Image;
use minifb::{Key, Window, WindowOptions};

#[derive(PartialEq, Debug, Clone)]
pub enum ImageType {
    Screenshot { x: i32, y: i32, w: usize, h: usize },
    ImagePath(String),
}

thread_local! {
    static WINDOW: RefCell<Option<Window>> = RefCell::new(None);
}

pub fn test_show_image(title: Option<String>, image_type: ImageType) {
    let image = match image_type {
        ImageType::Screenshot { x, y, w, h } => Image::from_screen(x, y, w, h),
        ImageType::ImagePath(path) => Image::from_path(path),
    };
    let (width, height) = image.get_dims();

    WINDOW.with(|window_cell| {
        let mut window_opt = window_cell.borrow_mut();
        let window = match &mut *window_opt {
            None => {
                let new_window = Window::new(
                    &title.unwrap_or_default(),
                    width as usize,
                    height as usize,
                    WindowOptions {
                        resize: true, // resizable by mouse
                        ..WindowOptions::default()
                    },
                )
                .unwrap_or_else(|e| {
                    panic!("{}", e);
                });
                *window_opt = Some(new_window);
                window_opt.as_mut().unwrap()
            }
            Some(window) => {
                // Update window info
                window.set_title(&title.unwrap_or_default());
                // TODO: window.set_size(width as usize, height as usize);
                window
            }
        };

        // Directly use the window variable
        if window.is_open() && !window.is_key_down(Key::Escape) {
            show_image(&image, width, height, window);
        } else {
            // close the window safely
            drop_window();
        }
    });
}

fn show_image(image: &Image, width: u32, height: u32, window: &mut Window) {
    let buffer = image.get_buffer_u32();
    window
        .update_with_buffer(&buffer, width as usize, height as usize)
        .unwrap();
}

fn drop_window() {
    WINDOW.with(|window_cell| {
        let mut window_opt = window_cell.borrow_mut();
        if let Some(window) = window_opt.take() {
            drop(window);
        }
    });
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
