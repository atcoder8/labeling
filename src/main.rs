use std::io;

/// Characters representing the foreground.
const BACKGROUND_CHARACTER: char = '0';

/// Characters representing the background.
const FOREGROUND_CHARACTER: char = '1';

fn main() {
    // Reads binary images from standard input.
    let mut binary_image: Vec<Vec<bool>> = vec![];
    for line in io::stdin().lines() {
        let binary_image_row = line
            .unwrap()
            .chars()
            .map(|x| match x {
                BACKGROUND_CHARACTER => false,
                FOREGROUND_CHARACTER => true,
                _ => panic!(
                    "The character entered must be a {} or {}.",
                    BACKGROUND_CHARACTER, FOREGROUND_CHARACTER
                ),
            })
            .collect();
        binary_image.push(binary_image_row);
    }

    // Output image size to standard output.
    println!(
        "Image size is {}x{}.",
        binary_image[0].len(),
        binary_image.len()
    );

    // Output the loaded binary image to standard output.
    // (Background: ".", Foreground: "#")
    println!("\n[Binary image]");
    for labeled_image_row in binary_image.iter() {
        labeled_image_row.iter().for_each(|&x| {
            if x {
                print!("#");
            } else {
                print!(".");
            }
        });
        println!();
    }

    // Create labeled images based on four neighborhoods.
    let labeled_image_based_on_four_nei = labeling::four_neighborhood_based_labeling(&binary_image);
    // Outputs images labeled based on the four neighborhoods to standard output.
    println!("\n[Labeled image (Based on four neighborhoods)]");
    for labeled_image_row in labeled_image_based_on_four_nei.iter() {
        labeled_image_row.iter().for_each(|&x| {
            if x >= 0 {
                print!("{}", x);
            } else {
                print!(".");
            }
        });
        println!();
    }

    // Create labeled images based on eight neighborhoods.
    let labeled_image_based_on_eight_nei =
        labeling::eight_neighborhood_based_labeling(&binary_image);
    // Outputs images labeled based on the eight neighborhoods to standard output.
    println!("\n[Labeled image (Based on eight neighborhoods)]");
    for labeled_image_row in labeled_image_based_on_eight_nei.iter() {
        labeled_image_row.iter().for_each(|&x| {
            if x >= 0 {
                print!("{}", x);
            } else {
                print!(".");
            }
        });
        println!();
    }
}
