use std::fmt;

struct Beer {
    current_index: u8,
    next_index: u8
}

impl Beer {
    fn new(index: u8) -> Beer {
        Beer {
            current_index: index,
            next_index: Beer::next_index(index)
        }
    }

    fn fmt_bottle(index: u8, capitalize: bool) -> String {
        match index {
            0 => {
                let no_more = if capitalize { "No more" } else { "no more" };
                format!("{} {}", no_more, "bottles")
            },
            1 => format!("{} {}", index, "bottle"),
            _ => format!("{} {}", index, "bottles")
        }
    }

    fn fmt_takedown(index: u8) -> String {
        match index {
            0 => "Go to the store and buy some more".to_string(),
            1 => "Take it down and pass it around".to_string(),
            _ => "Take one down and pass it around".to_string()
        }
    }

    fn next_index(index: u8) -> u8 {
        match index.overflowing_sub(1u8) {
            (n, false) => n,
            _ => 99u8
        }
    }
}



impl fmt::Display for Beer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} of beer on the wall, {} of beer.\n{}, {} of beer on the wall.\n",
            Beer::fmt_bottle(self.current_index, true),
            Beer::fmt_bottle(self.current_index, false),
            Beer::fmt_takedown(self.current_index),
            Beer::fmt_bottle(self.next_index, false)
        )
    }
}

struct BeerIterator {
    beers: Vec<Beer>
}

impl BeerIterator {
    fn new(start_index: u8, end_index: u8) -> BeerIterator {
        let mut beer_indices: Vec<u8> = (end_index..start_index).collect::<Vec<u8>>();
        beer_indices.push(start_index); // XXX Hack around half-open range

        BeerIterator {
            beers: beer_indices.into_iter().map(|i| Beer::new(i)).collect::<Vec<Beer>>()
        }
    }
}

impl Iterator for BeerIterator {
    type Item = Beer;

    fn next(&mut self) -> Option<Beer> {
        self.beers.pop()
    }
}

pub fn verse(index: u8) -> String {
    let beer = Beer::new(index);
    beer.to_string()
}

pub fn sing(start_index: u8, end_index: u8) -> String {
    let beer_iterator = BeerIterator::new(start_index, end_index);
    let with_newlines = |mut song: String, beer: Beer| {
        song += beer.to_string().as_str();
        if beer.current_index > end_index {
            song += "\n";
        }
        song
    };

    beer_iterator
        .take_while(|b| b.current_index >= end_index)
        .fold("".to_string(), with_newlines)
}
