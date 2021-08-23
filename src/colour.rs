use std::fmt::Display;

#[derive(Copy,Clone)]
pub enum Colour {
    Red,
    Orange,
    Green,
    Blue,
    Cyan,
    Magenta,
    LightGrey,
    Black,
    FallbackDefault,
}

#[derive(Copy,Clone)]
#[allow(dead_code)]
pub enum Layer {
    Background,
    ForeGround
}

impl Colour {
    fn to_code(&self, layer: Layer) -> &str {
        match layer {
            Layer::Background => {
                match self {
                    Colour::Black =>            "\x1B[40m",
                    Colour::Red =>              "\x1B[41m",
                    Colour::Green =>            "\x1B[42m",
                    Colour::Orange =>           "\x1B[43m",
                    Colour::Blue =>             "\x1B[44m",
                    Colour::Magenta =>          "\x1B[45m",
                    Colour::Cyan =>             "\x1B[46m",
                    Colour::LightGrey =>        "\x1B[47m",
                    Colour::FallbackDefault =>  "\x1B[49m",
                }
            },
            Layer::ForeGround => {
                match self {
                    Colour::Black =>            "\x1B[30m",
                    Colour::Red =>              "\x1B[31m",
                    Colour::Green =>            "\x1B[32m",
                    Colour::Orange =>           "\x1B[33m",
                    Colour::Blue =>             "\x1B[34m",
                    Colour::Magenta =>          "\x1B[35m",
                    Colour::Cyan =>             "\x1B[36m",
                    Colour::LightGrey =>        "\x1B[37m",
                    Colour::FallbackDefault =>  "\x1B[39m",
                }
            },
        }
    }
}


pub fn fmt_in_colour<D: Display>(text: D, c: Colour, l: Layer) -> String {
    format!("{}{}{}",
    c.to_code(l),text,
    Colour::FallbackDefault.to_code(l))
}


#[cfg(test)]
mod test {
    use super::*;
    fn print_in_colour<D: Display>(text: D, c: Colour, l: Layer) {
        print!("{}", fmt_in_colour(text,c,l));
    }
    #[test]
    #[ignore]
    fn test_colour() {
        print!("{}",Colour::Green.to_code(Layer::ForeGround));
        print!("hello world!");
        println!("{}",Colour::FallbackDefault.to_code(Layer::ForeGround));
    }

    #[test]
    #[ignore]
    fn test_rainbow() {
        let colours = [
            Colour::Red,
            Colour::Orange,
            Colour::Green,
            Colour::Blue,
            Colour::Cyan,
            Colour::Magenta];
        for (index,character) in "rainbow\n".to_owned().chars().enumerate() {
            let current_colour = colours[index%colours.len()];
            print_in_colour(character, current_colour, Layer::ForeGround);
        }
    }
}

