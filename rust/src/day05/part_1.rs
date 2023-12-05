use std::str::FromStr;

pub(crate) struct Mapping {
    destination_range: std::ops::RangeInclusive<i64>,
    source_range: std::ops::RangeInclusive<i64>,
}

impl FromStr for Mapping {
    type Err = std::num::ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let range = line
            .split(' ')
            .map(|num| num.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(range.len(), 3);
        Ok(Mapping {
            destination_range: range[0]..=range[0] + range[2],
            source_range: range[1]..=range[1] + range[2],
        })
    }
}

pub(crate) fn part_1(input: &'static str) -> Result<i64, <Mapping as FromStr>::Err> {
    let mut lines = input.lines();
    let lines = lines.by_ref();

    let mut values = lines
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.split(' ')
                .skip(1)
                .map(|num| num.parse().expect("Should be a valid i64"))
        })
        .flatten()
        .collect::<Vec<_>>();
    // println!("seed {:?}", values);
    // Seed to soil
    let seed_to_soil = lines
        .take_while(|line| !line.is_empty())
        .skip(1)
        .map(Mapping::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    for value in values.iter_mut() {
        if let Some(mapping) = seed_to_soil
            .iter()
            .find(|group| group.source_range.contains(&value))
        {
            *value += mapping.destination_range.start() - mapping.source_range.start();
        };
    }
    // println!("soil {:?}", values);
    let soil_to_fertilizer = lines
        .take_while(|line| !line.is_empty())
        .skip(1)
        .map(Mapping::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    for value in values.iter_mut() {
        if let Some(mapping) = soil_to_fertilizer
            .iter()
            .find(|group| group.source_range.contains(&value))
        {
            *value += mapping.destination_range.start() - mapping.source_range.start();
        };
    }
    // println!("fert {:?}", values);
    let fertilizer_to_water = lines
        .take_while(|line| !line.is_empty())
        .skip(1)
        .map(Mapping::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    for value in values.iter_mut() {
        if let Some(mapping) = fertilizer_to_water
            .iter()
            .find(|group| group.source_range.contains(&value))
        {
            *value += mapping.destination_range.start() - mapping.source_range.start();
        };
    }
    // println!("watr {:?}", values);
    let water_to_light = lines
        .take_while(|line| !line.is_empty())
        .skip(1)
        .map(Mapping::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    for value in values.iter_mut() {
        if let Some(mapping) = water_to_light
            .iter()
            .find(|group| group.source_range.contains(&value))
        {
            *value += mapping.destination_range.start() - mapping.source_range.start();
        };
    }
    // println!("lite {:?}", values);
    let light_to_temperature = lines
        .take_while(|line| !line.is_empty())
        .skip(1)
        .map(Mapping::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    for value in values.iter_mut() {
        if let Some(mapping) = light_to_temperature
            .iter()
            .find(|group| group.source_range.contains(&value))
        {
            *value += mapping.destination_range.start() - mapping.source_range.start();
        };
    }
    // println!("temp {:?}", values);
    let temperature_to_humidity = lines
        .take_while(|line| !line.is_empty())
        .skip(1)
        .map(Mapping::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    for value in values.iter_mut() {
        if let Some(mapping) = temperature_to_humidity
            .iter()
            .find(|group| group.source_range.contains(&value))
        {
            *value += mapping.destination_range.start() - mapping.source_range.start();
        };
    }
    // println!("hmdt {:?}", values);
    let humidity_to_location = lines
        .take_while(|line| !line.is_empty())
        .skip(1)
        .map(Mapping::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    for value in values.iter_mut() {
        if let Some(mapping) = humidity_to_location
            .iter()
            .find(|group| group.source_range.contains(&value))
        {
            *value += mapping.destination_range.start() - mapping.source_range.start();
        };
    }
    // println!("loct {:?}", values);
    Ok(*values.iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(
            super::part_1(SAMPLE_INPUT).expect("Should run on sample input"),
            35
        )
    }

    #[test]
    fn test_with_solution() {
        assert_eq!(
            super::part_1(crate::INPUT).expect("Should run on real input"),
            621354867
        );
    }
}
