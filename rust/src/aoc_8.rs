use std::io::BufRead;

const N_LAYER: usize = 100;
const WIDTH: usize = 25;
const HEIGHT: usize = 6;

pub fn aoc_8() {
    let input = std::fs::File::open("../input-8.txt").unwrap();
    let mut line = String::new();
    std::io::BufReader::new(input)
        .read_line(&mut line)
        .unwrap()
        .to_string();
    let mut image = [[[' '; WIDTH]; HEIGHT]; N_LAYER];
    let mut chars = line.chars();
    for img in image.iter_mut().take(N_LAYER) {
        for row in img.iter_mut().take(HEIGHT) {
            for cell in row.iter_mut().take(WIDTH) {
                *cell = chars.next().unwrap();
            }
        }
    }
    let mut min_zeros = num_digits(image[0], '0');
    let mut min_layer = 0;

    for (layer, img) in image.iter().enumerate().take(N_LAYER).skip(1) {
        let zeroes = num_digits(*img, '0');
        if zeroes < min_zeros {
            min_layer = layer;
            min_zeros = zeroes;
        }
    }
    let ones = num_digits(image[min_layer], '1');
    let twos = num_digits(image[min_layer], '2');
    println!(
        "min_layer: {}  ones:  {} twos:  {}  answer:  {}",
        min_layer,
        ones,
        twos,
        ones * twos
    );
    let mut final_img = [[' '; WIDTH]; HEIGHT];
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            for layer in image.iter() {
                if layer[j][i] != '2' {
                    final_img[j][i] = layer[j][i];
                    break;
                }
            }
        }
    }
    for row in final_img.iter_mut() {
        for cell in row.iter_mut() {
            print!("{}", if '1' == *cell { 'X' } else { ' ' });
        }
        println!();
    }
}

fn num_digits(image: [[char; WIDTH]; HEIGHT], digit: char) -> usize {
    let mut n = 0;
    for row in image.iter() {
        for cell in row.iter() {
            if cell == &digit {
                n += 1;
            }
        }
    }
    n
}
