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
    None(RangeInclusive<i64>),
    // MAPPING   345      -> 789 (+4)
    //   RANGE  234       // > > < > // 2 78    // X
    //   RANGE     567    // < < < = // 9 67    // X
    //   RANGE    456     // < < < > // 89 6    // X
    //   RANGE 123        // > > = > // 12 7    // X
    //   RANGE 1234       // > = < > // 12 78   //
    Simple(RangeInclusive<i64>, RangeInclusive<i64>),
    // MAPPING 1234       -> 5678 (+4)
    //   RANGE  234       // < = < > // 678     // X
    //   RANGE 123        // = > = > // 567     // X
    Contained(RangeInclusive<i64>),
    // MAPPING 12345      -> 56789 (+4)
    //   RANGE  234       // < > < > // 789     // X
    //   RANGE 12345      // = = = = // 56789   // X
    Complete(RangeInclusive<i64>),
    // MAPPING  234       -> 678 (+4)
    //   RANGE 12345      // > < < > // 1 678 5 // X
    Overreaching(
        RangeInclusive<i64>,
        RangeInclusive<i64>,
        RangeInclusive<i64>,
    ),
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
            (Ordering::Greater, Ordering::Equal, Ordering::Less, Ordering::Greater) => {
                Overlap::Simple(
                    *range.start()..=self.source_range.start() - 1,
                    *self.destination_range.start()..=range.end() + self.diff(),
                )
            }
            (Ordering::Greater, Ordering::Less, Ordering::Less, Ordering::Greater) => {
                Overlap::Overreaching(
                    *range.start()..=self.source_range.start() - 1,
                    self.destination_range.clone(),
                    self.source_range.end() + 1..=*range.end(),
                )
            }
            (Ordering::Less, Ordering::Greater, Ordering::Less, Ordering::Greater) => {
                Overlap::Complete(range.start() + self.diff()..=range.end() + self.diff())
            }
            (Ordering::Equal, Ordering::Equal, Ordering::Equal, Ordering::Equal) => {
                Overlap::Complete(self.destination_range.clone())
            }
            (Ordering::Greater, Ordering::Greater, Ordering::Greater, Ordering::Greater)
            | (Ordering::Less, Ordering::Less, Ordering::Less, Ordering::Less) => {
                Overlap::None(range.clone())
            }
            (Ordering::Less, Ordering::Equal, Ordering::Less, Ordering::Greater)
            | (Ordering::Equal, Ordering::Greater, Ordering::Equal, Ordering::Greater) => {
                Overlap::Contained(range.start() + self.diff()..=range.end() + self.diff())
            }
            (Ordering::Greater, Ordering::Greater, Ordering::Less, Ordering::Greater) => {
                Overlap::Simple(
                    *range.start()..=self.source_range.start() - 1,
                    *self.destination_range.start()..=range.end() + self.diff(),
                )
            }
            (Ordering::Less, Ordering::Less, Ordering::Less, Ordering::Greater)
            | (Ordering::Less, Ordering::Less, Ordering::Less, Ordering::Equal) => Overlap::Simple(
                *range.start() + self.diff()..=*self.destination_range.end(),
                self.source_range.end() + 1..=*range.end(),
            ),
            (Ordering::Greater, Ordering::Greater, Ordering::Equal, Ordering::Greater) => {
                Overlap::Simple(
                    *range.start()..=self.source_range.start() - 1,
                    *self.destination_range.start()..=range.end() + self.diff(),
                )
            }
            (a, b, c, d) => unreachable!("{:?} {:?} {:?} {:?}", a, b, c, d),
        };

        match &result {
            Overlap::Contained(value) => assert_eq!(
                range.end() - range.start() + 1,
                value.end() - value.start() + 1
            ),
            Overlap::Complete(value) => assert_eq!(
                range.end() - range.start() + 1,
                value.end() - value.start() + 1
            ),
            Overlap::None(value) => assert_eq!(
                range.end() - range.start() + 1,
                value.end() - value.start() + 1
            ),
            Overlap::Simple(value1, value2) => assert_eq!(
                range.end() - range.start() + 1,
                (value1.end() - value1.start() + 1) + (value2.end() - value2.start() + 1)
            ),

            Overlap::Overreaching(value1, value2, value3) => assert_eq!(
                range.end() - range.start() + 1,
                (value1.end() - value1.start() + 1)
                    + (value2.end() - value2.start() + 1)
                    + (value3.end() - value3.start() + 1)
            ),
        }

        result
    }

    fn source_overlaps(&self, range: &RangeInclusive<i64>) -> bool {
        self.source_range.end() >= range.start() && self.source_range.start() <= range.end()
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
            destination_range: destination_start..=destination_start + length,
            source_range: source_start..=source_start + length,
        })
    }
}

pub(crate) fn part_2(input: &'static str) -> Result<i64, <Mapping as FromStr>::Err> {
    let mut lines = input.lines();
    let lines = lines.by_ref();

    let seed_values = lines
        .take_while(|line| !line.is_empty())
        .map(|line| line.split(' ').skip(1).map(|num| num.parse::<i64>()))
        .flatten()
        .collect::<Result<Vec<_>, _>>()?
        .chunks_exact(2)
        .map(|v| [v[0], v[1]])
        .map(|[start, length]| start..=(start + length - 1))
        .collect::<Vec<_>>();
    // println!("seed {:?}", values);
    let seed_to_soil = lines
        .take_while(|line| !line.is_empty())
        .skip(1)
        .map(Mapping::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    // println!("     {:?}", seed_to_soil);
    let mut soil_values = vec![];
    for seed in seed_values.iter() {
        if let Some(mapping) = seed_to_soil
            .iter()
            .find(|mapping| mapping.source_overlaps(seed))
        {
            let overlap = mapping.apply(seed);
            match overlap {
                Overlap::None(_) => unreachable!(),
                Overlap::Contained(range) | Overlap::Complete(range) => soil_values.push(range),
                Overlap::Simple(left, right) => {
                    soil_values.push(left);
                    soil_values.push(right);
                }
                Overlap::Overreaching(left, right, rest) => {
                    soil_values.push(left);
                    soil_values.push(right);
                    soil_values.push(rest);
                }
            }
        } else {
            soil_values.push(seed.clone());
        }
    }

    // println!("soil {:?}", soil_values);
    let soil_to_fertilizer = lines
        .take_while(|line| !line.is_empty())
        .skip(1)
        .map(Mapping::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    // println!("     {:?}", soil_to_fertilizer);

    let mut fertilizer_values = vec![];
    for soil in soil_values.iter() {
        if let Some(mapping) = soil_to_fertilizer
            .iter()
            .find(|mapping| mapping.source_overlaps(soil))
        {
            let overlap = mapping.apply(soil);
            match overlap {
                Overlap::None(_) => unreachable!(),
                Overlap::Contained(range) | Overlap::Complete(range) => {
                    fertilizer_values.push(range)
                }
                Overlap::Simple(left, right) => {
                    fertilizer_values.push(left);
                    fertilizer_values.push(right);
                }
                Overlap::Overreaching(left, right, rest) => {
                    fertilizer_values.push(left);
                    fertilizer_values.push(right);
                    fertilizer_values.push(rest);
                }
            }
        } else {
            fertilizer_values.push(soil.clone());
        }
    }

    // println!("fert {:?}", fertilizer_values);
    let fertilizer_to_water = lines
        .take_while(|line| !line.is_empty())
        .skip(1)
        .map(Mapping::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    // println!("     {:?}", fertilizer_to_water);

    let mut water_values = vec![];
    for fertilizer in fertilizer_values.iter() {
        if let Some(mapping) = fertilizer_to_water
            .iter()
            .find(|mapping| mapping.source_overlaps(fertilizer))
        {
            let overlap = mapping.apply(fertilizer);
            match overlap {
                Overlap::None(_) => unreachable!(),
                Overlap::Contained(range) | Overlap::Complete(range) => water_values.push(range),
                Overlap::Simple(left, right) => {
                    water_values.push(left);
                    water_values.push(right);
                }
                Overlap::Overreaching(left, right, rest) => {
                    water_values.push(left);
                    water_values.push(right);
                    water_values.push(rest);
                }
            }
        } else {
            water_values.push(fertilizer.clone());
        }
    }
    // println!("watr {:?}", water_values);
    let water_to_light = lines
        .take_while(|line| !line.is_empty())
        .skip(1)
        .map(Mapping::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    // println!("     {:?}", water_to_light);

    let mut light_values = vec![];
    for water in water_values.iter() {
        if let Some(mapping) = water_to_light
            .iter()
            .find(|mapping| mapping.source_overlaps(water))
        {
            if *water.start() == 81 && *water.end() == 94 {
                // println!("DEBUG");
            }
            let overlap = mapping.apply(water);
            match overlap {
                Overlap::None(_) => unreachable!(),
                Overlap::Contained(range) | Overlap::Complete(range) => light_values.push(range),
                Overlap::Simple(left, right) => {
                    light_values.push(left);
                    light_values.push(right);
                }
                Overlap::Overreaching(left, right, rest) => {
                    light_values.push(left);
                    light_values.push(right);
                    light_values.push(rest);
                }
            }
        } else {
            light_values.push(water.clone());
        }
    }
    // println!("lite {:?}", light_values);
    let light_to_temperature = lines
        .take_while(|line| !line.is_empty())
        .skip(1)
        .map(Mapping::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    // println!("     {:?}", light_to_temperature);

    let mut temperature_values = vec![];
    for light in light_values.iter() {
        if let Some(mapping) = light_to_temperature
            .iter()
            .find(|mapping| mapping.source_overlaps(light))
        {
            let overlap = mapping.apply(light);
            match overlap {
                Overlap::None(_) => unreachable!(),
                Overlap::Contained(range) | Overlap::Complete(range) => {
                    temperature_values.push(range)
                }
                Overlap::Simple(left, right) => {
                    temperature_values.push(left);
                    temperature_values.push(right);
                }
                Overlap::Overreaching(left, right, rest) => {
                    temperature_values.push(left);
                    temperature_values.push(right);
                    temperature_values.push(rest);
                }
            }
        } else {
            temperature_values.push(light.clone());
        }
    }

    // println!("temp {:?}", temperature_values);
    let temperature_to_humidity = lines
        .take_while(|line| !line.is_empty())
        .skip(1)
        .map(Mapping::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    // println!("     {:?}", temperature_to_humidity);

    let mut humidity_values = vec![];
    for temperature in temperature_values.iter() {
        if let Some(mapping) = temperature_to_humidity
            .iter()
            .find(|mapping| mapping.source_overlaps(temperature))
        {
            let overlap = mapping.apply(temperature);
            match overlap {
                Overlap::None(_) => unreachable!(),
                Overlap::Contained(range) | Overlap::Complete(range) => humidity_values.push(range),
                Overlap::Simple(left, right) => {
                    humidity_values.push(left);
                    humidity_values.push(right);
                }
                Overlap::Overreaching(left, right, rest) => {
                    humidity_values.push(left);
                    humidity_values.push(right);
                    humidity_values.push(rest);
                }
            }
        } else {
            humidity_values.push(temperature.clone());
        }
    }

    // println!("hmdt {:?}", humidity_values);
    let humidity_to_location = lines
        .take_while(|line| !line.is_empty())
        .skip(1)
        .map(Mapping::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    // println!("     {:?}", humidity_to_location);

    let mut location_values = vec![];
    for humidity in humidity_values.iter() {
        if let Some(mapping) = humidity_to_location
            .iter()
            .find(|mapping| mapping.source_overlaps(humidity))
        {
            let overlap = mapping.apply(humidity);
            match overlap {
                Overlap::None(_) => unreachable!(),
                Overlap::Contained(range) | Overlap::Complete(range) => location_values.push(range),
                Overlap::Simple(left, right) => {
                    location_values.push(left);
                    location_values.push(right);
                }
                Overlap::Overreaching(left, right, rest) => {
                    location_values.push(left);
                    location_values.push(right);
                    location_values.push(rest);
                }
            }
        } else {
            location_values.push(humidity.clone());
        }
    }

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

        assert!(solution < 54927065);
        assert_eq!(solution, 0);
    }
}
