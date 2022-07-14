use std::fmt;
use std::str::Lines;

#[derive(PartialEq)]
enum Plan {
    Horizontal,
    Vertical,
}

struct Car {
    x: u8,
    y: u8,
    length: u8,
    plan: Plan,
}

impl Car {
    pub fn parked_on(&self, x: u8, y: u8) -> bool {
        if self.plan == Plan::Horizontal {
            y == self.y && (self.x..(self.x + self.length)).contains(&x)
        } else {
            x == self.x && (self.y..(self.y + self.length)).contains(&y)
        }
    }
}

pub struct Puzzle {
    cars: Vec<Car>,
}

impl Puzzle {
    pub fn parse(content: Lines) -> Puzzle {
        let cars = content
            .filter_map(|line| -> Option<Car> {
                if line.chars().nth(0) == Some('#') {
                    None
                } else {
                    let mut data = line.split_whitespace();

                    if data.clone().count() != 4 {
                        panic!("Input file is malformed. Read : `{}`", line);
                    }

                    let x = data.nth(0)?.parse::<u8>().unwrap();
                    let y = data.nth(0)?.parse::<u8>().unwrap();
                    let plan = if data.nth(0)? == "vertical" {
                        Plan::Vertical
                    } else {
                        Plan::Horizontal
                    };
                    let length = data.nth(0)?.parse::<u8>().unwrap();

                    Some(Car {
                        x: x,
                        y: y,
                        length: length,
                        plan: plan,
                    })
                }
            })
            .collect::<Vec<Car>>();

        Puzzle { cars: cars }
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const EMPTY_LINE: String = String::new();

        let mut lines: [String; 6] = [EMPTY_LINE; 6];

        for y in 0..6 {
            lines[y as usize].push('|');

            for x in 0..6 {
                let any_car = self.cars.iter().any(|c| c.parked_on(x, y));

                if any_car {
                    lines[y as usize].push('x');
                } else {
                    lines[y as usize].push(' ');
                }
            }

            lines[y as usize].push('|');
        }

        let lines_as_str = lines.join("\n");

        write!(f, "|------|\n{}\n|------| ", lines_as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn car_parked_on_horitontal() {
        let car = Car {
            x: 0,
            y: 1,
            length: 3,
            plan: Plan::Horizontal,
        };

        assert_eq!(true, car.parked_on(0, 1));
        assert_eq!(true, car.parked_on(1, 1));
        assert_eq!(true, car.parked_on(2, 1));

        assert_ne!(true, car.parked_on(3, 1));
        assert_ne!(true, car.parked_on(4, 4));
    }

    #[test]
    fn car_parked_on_vertical() {
        let car = Car {
            x: 1,
            y: 0,
            length: 3,
            plan: Plan::Vertical,
        };

        assert_eq!(true, car.parked_on(1, 0));
        assert_eq!(true, car.parked_on(1, 1));
        assert_eq!(true, car.parked_on(1, 2));

        assert_ne!(true, car.parked_on(1, 3));
        assert_ne!(true, car.parked_on(4, 4));
    }
}
