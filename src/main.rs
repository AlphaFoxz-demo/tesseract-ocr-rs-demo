use tesseract::Tesseract;
fn main() {
    let img_path = "C:\\Users\\Wong\\Pictures\\Screenshots\\Screenshot 2024-02-17 224553.png";
    let lang = "chi_sim";
    let tes = Tesseract::new(None, Some(lang)).expect("Failed to init Tesseract");
    let start = std::time::Instant::now();
    let mut tes = tes.set_image(img_path).expect("Failed to set image");
    let text = tes.get_text().expect("Failed to get text");
    println!("got text: {}", text);
    println!("time: {:?}", start.elapsed());
}
