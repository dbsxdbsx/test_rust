pub fn test_image() -> Result<(), Box<dyn std::error::Error>> {
    // Read an image from file
    let img = image::open("./写轮眼_128x128.png")?;

    // Convert the image to grayscale
    let gray_img = img.grayscale();

    // Write the grayscale image to file
    gray_img.save("myimage_gray.png")?;

    Ok(())
}
