use image::GenericImageView;
use std::env;
//Im gonna add comments here to remmember what I've learnt 
fn main() {
    //colects args
    //stroes image path in var file_path
    //var img is the image provided, if it cannot open it will provide an error
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).map(|s| s.as_str()).unwrap_or("test.png");
    let img = image::open(file_path).expect("Error, could not open image");

    //creates a tuple of u32 which is width and height
    //creates new width which is 100
    //create new height which is h*(n_w/w)*0.5 and sets it to a u32
    //resizes the image with new width and height, and sets filter to nearest
    let (width, height) = img.dimensions();
    let new_width = 100;
    let new_height = (height as f32 *(new_width as f32 / width as f32) * 0.5) as u32;
    let img = img.resize_exact(new_width, new_height, image::imageops::FilterType::Nearest);

    //sets up the ascii character gradient
    let ascii_chars = r#" .'`^",:;Il!i><~+_-?][}{1)(|\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$"#;

    //loops thorugh every y in new height
    for y in 0..new_height {
        //loops through every x in new width, and this double for loop loops through ann the pixels
        for x in 0..new_width {
            //gets the specific pixel of the image
            let pixel = img.get_pixel(x, y);
            
            //gets the r, g, and b, values, along with its brightness
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;
            let brightness = (r + g + b) / 3.0;

            //divies the brightness by (255.0 *the length of ascii characters we provided - 1) as u32, and then passes it as usize
            let char_index = (brightness / 255.0 * (ascii_chars.len() - 1) as f32) as usize;
            
            //prints the ascii character aloong with its color
            let c = ascii_chars.as_bytes()[char_index] as char;
            print!("\x1b[38;2;{};{};{}m{}", pixel[0], pixel[1], pixel[2], c);
        }
        println!();
    }
    //resets terminal color
    println!("\x1b[0m");
}
