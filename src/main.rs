extern crate rand;

use std::vec;
use rand::Rng;

fn main() {
    let mut n = 0;

    println!("Original numbers:\n");

    let numbers = init_nums((0, 1079));
    let sorted_numbers = sort(numbers.clone());

    println!("\nSorted numbers:\n");

    while n < 191 {
        println!("{}", sorted_numbers[n].to_string());
        n += 1;
    }

    image_test(numbers, String::from("frames/frame1.png"));
}

fn init_nums(range: (u32, u32)) -> Vec<u32> {
    let mut rng = rand::thread_rng();

    let mut number_set = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut n = 0;

    while n < 191 {
        number_set[n] = rng.gen_range(range.0..=range.1);
        println!("{}", number_set[n].to_string());

        n += 1;
    }

    return number_set;
}

fn sort(numbers: Vec<u32>) -> Vec<u32> {
    let mut numbers_clone = numbers.clone();
    let mut frame = 2;

    loop {
        let mut swaps = 0;

        for n in 0..190 {
            if numbers_clone[n] > numbers_clone[n + 1] {
                let n1 = numbers_clone[n];
                let n2 = numbers_clone[n + 1];
                numbers_clone[n] = n2;
                numbers_clone[n + 1] = n1;

                swaps += 1;
            }
        }

        if swaps == 0 {
            break;
        }

        image_test(numbers_clone.clone(), format!("frames/frame{}.png", frame.to_string()));

        frame += 1;
    }

    return numbers_clone;
}

extern crate image;

use image::{ImageBuffer, Rgb};

fn image_test(lines: Vec<u32>, path: String) {
    let width = 1920;
    let height = 1080;

    // Create a new image buffer
    let mut img = ImageBuffer::new(width, height);

    let mut x_value = 1;

    // Iterate through the columns of the image
    for &x in &lines {
        // Determine whether the current column should be black or white based on the column number
        let white = Rgb([255u8, 255u8, 255u8]);

        // Iterate through the pixels in the current column
        for n in 0..x {
            for y in 0..n {
                img.put_pixel(x_value, 1079 - y, white); x_value += 1;
                img.put_pixel(x_value, 1079 - y, white); x_value += 1;
                img.put_pixel(x_value, 1079 - y, white); x_value += 1;
                img.put_pixel(x_value, 1079 - y, white); x_value += 1;
                img.put_pixel(x_value, 1079 - y, white); x_value += 1;
                img.put_pixel(x_value, 1079 - y, white); x_value += 1;
                img.put_pixel(x_value, 1079 - y, white); x_value += 1;
                img.put_pixel(x_value, 1079 - y, white); x_value += 1;
                img.put_pixel(x_value, 1079 - y, white); x_value += 1;
                img.put_pixel(x_value, 1079 - y, white);
                x_value -= 9;
            }
        }

        x_value += 10;
    }

    // Save the image to a file (you can change the format as needed)
    img.save(path).unwrap();
}