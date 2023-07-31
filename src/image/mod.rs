mod image_tool;
use crate::image::image_tool::Image;
use minifb::{Window, WindowOptions};
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

    println!("get buffer ok");

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
        let _window = window.as_mut().unwrap();
        println!("hello1");

        let buffer = image.get_buffer_u32();

        _window
            .update_with_buffer(&buffer, width as usize, height as usize)
            .unwrap();
        println!("hello2");
    });
}
