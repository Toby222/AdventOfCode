use std::{cmp::Ordering, fmt::Debug, ops::RangeInclusive, str::FromStr};

pub(crate) struct Mapping {
    source_range: std::ops::RangeInclusive<i64>,
    destination_range: std::ops::RangeInclusive<i64>,
}

impl Debug for Mapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} => {:?}", self.source_range, self.destination_range)
    }
}

impl std::fmt::Display for Mapping {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} => {:?}", self.source_range, self.destination_range)
    }
}

enum Overlap {
    // MAPPING    456     -> 789 (+3)
    //   RANGE       789  // < < < < // 789     // X
    //   RANGE 123        // > > > > // 123     // X
    None {
        rest: RangeInclusive<i64>,
    },
    // MAPPING   345      -> 789 (+4)
    //   RANGE  234       // > > < > // 2 78    // X
    //   RANGE     567    // < < < = // 9 67    // X
    //   RANGE    456     // < < < > // 89 6    // X
    //   RANGE 123        // > > = > // 12 7    // X
    //   RANGE 1234       // > = < > // 12 78   // X
    //   RANGE   3456     // = < < > // 789 X   //
    Simple {
        contained: RangeInclusive<i64>,
        rest: RangeInclusive<i64>,
    },
    // MAPPING 1234       -> 5678 (+4)
    //   RANGE  234       // < = < > // 678     // X
    //   RANGE 123        // = > < > // 567     // X
    Contained {
        contained: RangeInclusive<i64>,
    },
    // MAPPING 12345      -> 56789 (+4)
    //   RANGE  234       // < > < > // 789     // X
    //   RANGE 12345      // = = < > // 56789   // X
    Complete {
        contained: RangeInclusive<i64>,
    },
    // MAPPING  234       -> 678 (+4)
    //   RANGE 12345      // > < < > // 1 678 5 // X
    Overreaching {
        left: RangeInclusive<i64>,
        contained: RangeInclusive<i64>,
        right: RangeInclusive<i64>,
    },
}

impl Mapping {
    const fn diff(&self) -> i64 {
        *self.destination_range.start() - *self.source_range.start()
    }
    fn apply(&self, range: &RangeInclusive<i64>) -> Overlap {
        let result = match (
            i64::cmp(self.source_range.start(), range.start()),
            i64::cmp(self.source_range.end(), range.end()),
            i64::cmp(self.source_range.start(), range.end()),
            i64::cmp(self.source_range.end(), range.start()),
        ) {
            (Ordering::Equal, Ordering::Less, Ordering::Less, Ordering::Greater) => {
                Overlap::Simple {
                    contained: self.destination_range.clone(),
                    rest: self.source_range.end() + 1..=*range.end(),
                }
            }
            (Ordering::Greater, Ordering::Equal, Ordering::Less, Ordering::Greater) => {
                Overlap::Simple {
                    contained: *self.destination_range.start()..=range.end() + self.diff(),
                    rest: *range.start()..=self.source_range.start() - 1,
                }
            }
            (Ordering::Greater, Ordering::Less, Ordering::Less, Ordering::Greater) => {
                Overlap::Overreaching {
                    left: *range.start()..=self.source_range.start() - 1,
                    contained: self.destination_range.clone(),
                    right: self.source_range.end() + 1..=*range.end(),
                }
            }
            (Ordering::Less, Ordering::Greater, Ordering::Less, Ordering::Greater) => {
                Overlap::Complete {
                    contained: range.start() + self.diff()..=range.end() + self.diff(),
                }
            }
            (Ordering::Equal, Ordering::Equal, Ordering::Less, Ordering::Greater) => {
                Overlap::Complete {
                    contained: self.destination_range.clone(),
                }
            }
            (Ordering::Greater, Ordering::Greater, Ordering::Greater, Ordering::Greater)
            | (Ordering::Less, Ordering::Less, Ordering::Less, Ordering::Less) => Overlap::None {
                rest: range.clone(),
            },
            (Ordering::Less, Ordering::Equal, Ordering::Less, Ordering::Greater)
            | (Ordering::Equal, Ordering::Greater, Ordering::Less, Ordering::Greater) => {
                Overlap::Contained {
                    contained: range.start() + self.diff()..=range.end() + self.diff(),
                }
            }
            (Ordering::Greater, Ordering::Greater, Ordering::Less, Ordering::Greater) => {
                Overlap::Simple {
                    rest: *range.start()..=self.source_range.start() - 1,
                    contained: *self.destination_range.start()..=range.end() + self.diff(),
                }
            }
            (Ordering::Less, Ordering::Less, Ordering::Less, Ordering::Greater)
            | (Ordering::Less, Ordering::Less, Ordering::Less, Ordering::Equal) => {
                Overlap::Simple {
                    contained: *range.start() + self.diff()..=*self.destination_range.end(),
                    rest: self.source_range.end() + 1..=*range.end(),
                }
            }
            (Ordering::Greater, Ordering::Greater, Ordering::Equal, Ordering::Greater) => {
                Overlap::Simple {
                    rest: *range.start()..=self.source_range.start() - 1,
                    contained: *self.destination_range.start()..=range.end() + self.diff(),
                }
            }
            (a, b, c, d) => unreachable!("{:?} {:?} {:?} {:?}", a, b, c, d),
        };

        match &result {
            Overlap::Contained { contained } => assert_eq!(
                range.end() - range.start() + 1,
                contained.end() - contained.start() + 1
            ),
            Overlap::Complete { contained } => assert_eq!(
                range.end() - range.start() + 1,
                contained.end() - contained.start() + 1
            ),
            Overlap::None { rest } => assert_eq!(
                range.end() - range.start() + 1,
                rest.end() - rest.start() + 1
            ),
            Overlap::Simple { rest, contained } => assert_eq!(
                range.end() - range.start() + 1,
                (rest.end() - rest.start() + 1) + (contained.end() - contained.start() + 1),
                "{:?} ; {} ; {:?} ; {:?}",
                range,
                self,
                contained,
                rest,
            ),

            Overlap::Overreaching {
                left,
                contained,
                right,
            } => assert_eq!(
                range.end() - range.start() + 1,
                (left.end() - left.start() + 1)
                    + (contained.end() - contained.start() + 1)
                    + (right.end() - right.start() + 1)
            ),
        }

        result
    }
}

impl FromStr for Mapping {
    type Err = <i64 as FromStr>::Err;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let range = line
            .split(' ')
            .map(|num| num.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;
        let destination_start = range[0];
        let source_start = range[1];
        let length = range[2];
        Ok(Mapping {
            destination_range: destination_start..=destination_start + length - 1,
            source_range: source_start..=source_start + length - 1,
        })
    }
}

pub(crate) fn part_2(input: &'static str) -> Result<i64, <Mapping as FromStr>::Err> {
    let mut lines = input.lines();
    let lines = lines.by_ref();

    let mut seed_values = lines
        .take_while(|line| !line.is_empty())
        .map(|line| line.split(' ').skip(1).map(|num| num.parse::<i64>()))
        .flatten()
        .collect::<Result<Vec<_>, _>>()?
        .chunks_exact(2)
        .map(|v| [v[0], v[1]])
        .map(|[start, length]| start..=(start + length - 1))
        .collect::<Vec<_>>();
    println!("seed {:?}", seed_values);
    let seed_to_soil = lines
        .take_while(|line| !line.is_empty())
        .skip(1)
        .map(Mapping::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    println!("     {:?}", seed_to_soil);
    let mut soil_values = vec![];
    for mapping in &seed_to_soil {
        let mut new_seeds = vec![];
        for seed in &seed_values {
            let overlap = mapping.apply(seed);

            match overlap {
                Overlap::None { rest } => {
                    new_seeds.push(rest);
                }
                Overlap::Simple { contained, rest } => {
                    soil_values.push(contained);
                    new_seeds.push(rest);
                }
                Overlap::Contained { contained } => soil_values.push(contained),
                Overlap::Complete { contained } => soil_values.push(contained),
                Overlap::Overreaching {
                    left,
                    contained,
                    right,
                } => {
                    soil_values.push(contained);
                    new_seeds.push(left);
                    new_seeds.push(right);
                }
            }
        }
        seed_values = new_seeds;
    }
    soil_values.extend_from_slice(&seed_values);

    println!("soil {:?}", soil_values);
    let soil_to_fertilizer = lines
        .take_while(|line| !line.is_empty())
        .skip(1)
        .map(Mapping::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    println!("     {:?}", soil_to_fertilizer);

    let mut fertilizer_values = vec![];
    for mapping in &soil_to_fertilizer {
        let mut new_soils = vec![];
        for soil in &soil_values {
            let overlap = mapping.apply(soil);

            match overlap {
                Overlap::None { rest } => {
                    new_soils.push(rest);
                }
                Overlap::Simple { contained, rest } => {
                    fertilizer_values.push(contained);
                    new_soils.push(rest);
                }
                Overlap::Contained { contained } => fertilizer_values.push(contained),
                Overlap::Complete { contained } => fertilizer_values.push(contained),
                Overlap::Overreaching {
                    left,
                    contained,
                    right,
                } => {
                    fertilizer_values.push(contained);
                    new_soils.push(left);
                    new_soils.push(right);
                }
            }
        }
        soil_values = new_soils;
    }
    fertilizer_values.extend_from_slice(&soil_values);

    println!("fert {:?}", fertilizer_values);
    let fertilizer_to_water = lines
        .take_while(|line| !line.is_empty())
        .skip(1)
        .map(Mapping::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    println!("     {:?}", fertilizer_to_water);

    let mut water_values = vec![];
    for mapping in &fertilizer_to_water {
        let mut new_fertilizers = vec![];
        for fertilizer in &fertilizer_values {
            let overlap = mapping.apply(fertilizer);

            match overlap {
                Overlap::None { rest } => {
                    new_fertilizers.push(rest);
                }
                Overlap::Simple { contained, rest } => {
                    water_values.push(contained);
                    new_fertilizers.push(rest);
                }
                Overlap::Contained { contained } => water_values.push(contained),
                Overlap::Complete { contained } => water_values.push(contained),
                Overlap::Overreaching {
                    left,
                    contained,
                    right,
                } => {
                    water_values.push(contained);
                    new_fertilizers.push(left);
                    new_fertilizers.push(right);
                }
            }
        }
        fertilizer_values = new_fertilizers;
    }
    water_values.extend_from_slice(&fertilizer_values);

    println!("watr {:?}", water_values);
    let water_to_light = lines
        .take_while(|line| !line.is_empty())
        .skip(1)
        .map(Mapping::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    println!("     {:?}", water_to_light);

    let mut light_values = vec![];
    for mapping in &water_to_light {
        let mut new_waters = vec![];
        for fertilizer in &water_values {
            let overlap = mapping.apply(fertilizer);

            match overlap {
                Overlap::None { rest } => {
                    new_waters.push(rest);
                }
                Overlap::Simple { contained, rest } => {
                    light_values.push(contained);
                    new_waters.push(rest);
                }
                Overlap::Contained { contained } => light_values.push(contained),
                Overlap::Complete { contained } => light_values.push(contained),
                Overlap::Overreaching {
                    left,
                    contained,
                    right,
                } => {
                    light_values.push(contained);
                    new_waters.push(left);
                    new_waters.push(right);
                }
            }
        }
        water_values = new_waters;
    }
    light_values.extend_from_slice(&water_values);

    println!("lite {:?}", light_values);
    let light_to_temperature = lines
        .take_while(|line| !line.is_empty())
        .skip(1)
        .map(Mapping::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    println!("     {:?}", light_to_temperature);

    let mut temperature_values = vec![];
    for mapping in &light_to_temperature {
        let mut new_lights = vec![];
        for light in &light_values {
            let overlap = mapping.apply(light);

            match overlap {
                Overlap::None { rest } => {
                    new_lights.push(rest);
                }
                Overlap::Simple { contained, rest } => {
                    temperature_values.push(contained);
                    new_lights.push(rest);
                }
                Overlap::Contained { contained } => temperature_values.push(contained),
                Overlap::Complete { contained } => temperature_values.push(contained),
                Overlap::Overreaching {
                    left,
                    contained,
                    right,
                } => {
                    temperature_values.push(contained);
                    new_lights.push(left);
                    new_lights.push(right);
                }
            }
        }
        light_values = new_lights;
    }
    temperature_values.extend_from_slice(&light_values);

    println!("temp {:?}", temperature_values);
    let temperature_to_humidity = lines
        .take_while(|line| !line.is_empty())
        .skip(1)
        .map(Mapping::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    println!("     {:?}", temperature_to_humidity);

    let mut humidity_values = vec![];
    for mapping in &temperature_to_humidity {
        let mut new_temperatures = vec![];
        for temperature in &temperature_values {
            let overlap = mapping.apply(temperature);

            match overlap {
                Overlap::None { rest } => {
                    new_temperatures.push(rest);
                }
                Overlap::Simple { contained, rest } => {
                    humidity_values.push(contained);
                    new_temperatures.push(rest);
                }
                Overlap::Contained { contained } => humidity_values.push(contained),
                Overlap::Complete { contained } => humidity_values.push(contained),
                Overlap::Overreaching {
                    left,
                    contained,
                    right,
                } => {
                    humidity_values.push(contained);
                    new_temperatures.push(left);
                    new_temperatures.push(right);
                }
            }
        }
        temperature_values = new_temperatures;
    }
    humidity_values.extend_from_slice(&light_values);

    println!("hmdt {:?}", humidity_values);
    let humidity_to_location = lines
        .take_while(|line| !line.is_empty())
        .skip(1)
        .map(Mapping::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    println!("     {:?}", humidity_to_location);

    let mut location_values = vec![];
    for mapping in &humidity_to_location {
        let mut new_humidities = vec![];
        for humidity in &humidity_values {
            let overlap = mapping.apply(humidity);

            match overlap {
                Overlap::None { rest } => {
                    new_humidities.push(rest);
                }
                Overlap::Simple { contained, rest } => {
                    location_values.push(contained);
                    new_humidities.push(rest);
                }
                Overlap::Contained { contained } => location_values.push(contained),
                Overlap::Complete { contained } => location_values.push(contained),
                Overlap::Overreaching {
                    left,
                    contained,
                    right,
                } => {
                    location_values.push(contained);
                    new_humidities.push(left);
                    new_humidities.push(right);
                }
            }
        }
        humidity_values = new_humidities;
    }
    location_values.extend_from_slice(&humidity_values);
    println!("loct {:?}", location_values);

    Ok(location_values
        .iter()
        .map(RangeInclusive::start)
        .min()
        .copied()
        .unwrap())
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_sample_solution() {
        let solution = super::part_2(SAMPLE_INPUT).expect("Should run on sample input");
        assert_eq!(solution, 46)
    }

    #[test]
    fn test_with_solution() {
        let solution = super::part_2(crate::INPUT).expect("Should run on real input");

        assert_eq!(solution, 15880236);
    }
}
