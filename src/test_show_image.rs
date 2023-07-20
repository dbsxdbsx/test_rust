use image::GenericImageView;
use pyo3::pyfunction;
use show_image::{create_window, event, ImageInfo, ImageView};
use std::path::{Path, PathBuf};

use crate::test_image::{capture_screen, Image};

// #[show_image::main]
#[pyfunction]
pub fn test_show_image(image_path: &str) {
    let path_buf = PathBuf::from(image_path);
    ::show_image::run_context(|| user_main(path_buf));
}

fn user_main(image_path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    // way1: 从文件中读取图像
    let img = Image::from_path(image_path);

    // way2: 截屏作图像
    // let img = Image::from_screen(0, 0, 1920, 1080);

    // 加载到`ImageView`中
    let img_dims = img.get_dims();
    let img_buff = img.get_buffer();
    let img = ImageView::new(ImageInfo::rgb8(img_dims.0, img_dims.1), &img_buff);

    // 使用默认选项创建窗口并显示图像
    let mut window_options = show_image::WindowOptions::default();
    window_options.size = Some([img_dims.0, img_dims.1]);
    window_options.resizable = true;
    window_options.preserve_aspect_ratio = false; // set false to make it stretchable
    let window = create_window("image", window_options)?;
    window.set_image("image-001", &img)?;

    // 打印键盘事件，直到按下 Escape 键退出
    // 如果用户关闭窗口，通道将关闭，循环也将退出
    for event in window.event_channel()? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            println!("{:#?}", event);
            if event.input.key_code == Some(event::VirtualKeyCode::Escape)
                && event.input.state.is_pressed()
            {
                break;
            }
        }
    }

    Ok(())
}
