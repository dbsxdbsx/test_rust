use image::GenericImageView;
use pyo3::pyfunction;
use show_image::{create_window, event, ImageInfo, ImageView};
use std::path::{Path, PathBuf};

// #[show_image::main]
#[pyfunction]
pub fn test_show_image(image_path: &str) {
    let path_buf = PathBuf::from(image_path);
    ::show_image::run_context(|| user_main(path_buf));
}

fn user_main(image_path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    // 从文件中读取图像
    let img = image::open(image_path)?;
    let img_dimensions = img.dimensions();
    let img = img.into_rgb8();
    let image = ImageView::new(ImageInfo::rgb8(img_dimensions.0, img_dimensions.1), &img);

    // 使用默认选项创建窗口并显示图像
    let mut window_options = show_image::WindowOptions::default();
    window_options.size = Some([img_dimensions.0, img_dimensions.1]);
    // window_options.background_color = show_image::Color::from_u32(0);
    let window = create_window("image", window_options)?;
    window.set_image("image-001", &image)?;

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
