mod image_tool;
use crate::image_multi::image_tool::Image;
use minifb::{Key, Window, WindowOptions};
use std::cell::RefCell;

thread_local! {
    static WINDOW: RefCell<Option<Window>> = RefCell::new(None);
}

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
    show_window(title, width, height, image);

    // way 2 with `show-image`
    // refer1: https://github.com/robohouse-delft/show-image-rs/issues/34#issuecomment-1657187599
    // refer2: https://github.com/robohouse-delft/show-image-rs/issues/33#issuecomment-1664958112
    // show_window2(title, width, height, image);
}

use rand::Rng;
fn create_random_buffer(width: usize, height: usize) -> Vec<u32> {
    let mut buffer = vec![0; width * height];
    let mut rng = rand::thread_rng();
    for pixel in buffer.iter_mut() {
        *pixel = rng.gen();
    }
    buffer
}

fn show_window(title: Option<String>, width: u32, height: u32, image: Image) {
    let mut window = Window::new(
        &title.clone().unwrap_or_default(),
        width as usize,
        height as usize,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .unwrap();
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let buffer = image.get_buffer_u32();
    // let buffer = create_random_buffer(width as usize, height as usize);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, width as usize, height as usize)
            .unwrap();
    }
}

fn show_window_muti_thread(title: Option<String>, width: u32, height: u32, image: Image) {
    WINDOW.with(|window_cell| {
        let mut window = window_cell.borrow_mut();
        if window.is_none() {
            let new_window = Window::new(
                &title.clone().unwrap_or_default(),
                width as usize,
                height as usize,
                WindowOptions {
                    resize: true,
                    ..WindowOptions::default()
                },
            )
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });
            *window = Some(new_window);
        }
        let window = window.as_mut().unwrap();

        // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        let buffer = image.get_buffer_u32();
        while window.is_open() && !window.is_key_down(Key::Escape) {
            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            window
                .update_with_buffer(&buffer, width as usize, height as usize)
                .unwrap();
        }
    });
}

// use show_image::{create_window, event};
// fn show_window2(title: Option<String>, width: u32, height: u32, image: image_tool::Image) {
//     // Convert image_tool::Image to image::DynamicImage
//     let dynamic_image = image.get_image().clone();
//     let buff  = image.get_buffer_u32();
//     // Convert image::DynamicImage to show_image::Image
//     let show_image = show_image::Image::from(dynamic_image);

//     // Create a window and display the image.
//     let window = create_window("image", Default::default()).unwrap();
//     // window.set_image("image-001", show_image).unwrap();
//     window.set_image("image-001", buff.into_raw()).unwrap();

//     // Print keyboard events until Escape is pressed, then exit.
//     // If the user closes the window, the channel is closed and the loop also exits.
//     for event in window.event_channel().unwrap() {
//         if let event::WindowEvent::KeyboardInput(event) = event {
//             println!("{:#?}", event);
//             if event.input.key_code == Some(event::VirtualKeyCode::Escape)
//                 && event.input.state.is_pressed()
//             {
//                 break;
//             }
//         }
//     }
// }
