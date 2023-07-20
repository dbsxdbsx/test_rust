use screenshots::Screen;

pub fn test_image() -> Result<(), Box<dyn std::error::Error>> {
    // Read an image from file
    let img = image::open("./写轮眼_128x128.png")?;

    // Convert the image to grayscale
    let gray_img = img.grayscale();

    // Write the grayscale image to file
    gray_img.save("myimage_gray.png")?;

    Ok(())
}

pub fn capture_screen(x: i32, y: i32, width: usize, height: usize) -> Vec<u8> {
    let _start = std::time::Instant::now();
    let screen = Screen::all().unwrap()[0];

    // let mut image = screen.capture().unwrap(); // for global screen capture
    let image = screen
        .capture_area(x, y, width as u32, height as u32)
        .unwrap();

    // println!("screen display info:{:?}", screen.display_info.id);
    // std::fs::write(format!("./{}.png", screen.display_info.id), buffer).unwrap();

    image.to_png().unwrap()
}
