use std::{
    collections::HashMap,
    i32,
    ops::{Add, AddAssign, Sub},
};

type Pos = i64;
type Number = i64;

trait Gettable {
    type V: Add<Self::V, Output = Self::V> + PartialOrd;
    fn getv(&self) -> Self::V;
    fn new(x: Self::V) -> Self;
}
trait TypeEqual {}
impl<T> TypeEqual for (T, T) {}

type GetV<T> = <T as Gettable>::V;

struct MapRange<S, T, V>
where
    S: Gettable,
    T: Gettable,
    // T: Gettable<V = <S as Gettable>::V>,
{
    source: S,
    target: T,
    length: V,
}

impl<S, T> MapRange<S, T, Number>
where
    S: Gettable<V = Number>,
    T: Gettable<V = Number>,
{
    fn is_in_source_range(&self, x: &S) -> bool {
        let x = x.getv();
        let var_name = self.source.getv() + self.length;
        (self.source.getv() <= x) && (x < var_name)
    }
    fn compute_target_for(&self, x: S) -> T {
        assert!(self.is_in_source_range(&x));
        let t = x.getv() - self.source.getv() + self.target.getv();
        return T::new(t);
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Soil(Pos);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Seed(Pos);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Fertilizer(Pos);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Water(Pos);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Light(Pos);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Temperature(Pos);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Humidity(Pos);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Location(Pos);

impl Gettable for Seed {
    type V = Number;
    fn getv(&self) -> Self::V {
        return self.0;
    }
    fn new(x: Self::V) -> Self {
        Self(x)
    }
}

// impl Gettable<Pos> for Soil {
//     type V = Pos;
//     fn getv(&self) -> Self::V {
//         return self.0;
//     }
//     fn new(x: Self::V) -> Self {
//         Self(x)
//     }
// }

// impl Gettable<Pos> for Fertilizer {
//     type V = Pos;
//     fn getv(&self) -> Self::V {
//         return self.0;
//     }
//     fn new(x: Self::V) -> Self {
//         Self(x)
//     }
// }

type SeedToSoilMap = HashMap<Seed, Soil>;
type SoilToFertilizerMap = HashMap<Soil, Fertilizer>;
type FertilizerToWaterMap = HashMap<Fertilizer, Water>;
type WaterToLightMap = HashMap<Water, Light>;
type LightToTemperatureMap = HashMap<Light, Temperature>;
type TemperatureToHumidityMap = HashMap<Temperature, Humidity>;
type HumidityToLocation = HashMap<Humidity, Location>;

struct Parser {
    seeds: Vec<Seed>,
    seed_to_soil: SeedToSoilMap,
    soil_to_fertilizer: SoilToFertilizerMap,
    fertilizer_to_water: FertilizerToWaterMap,
    water_to_light: WaterToLightMap,
    light_to_temperature: LightToTemperatureMap,
    temperature_to_humidity: TemperatureToHumidityMap,
    humidity_to_location: HumidityToLocation,
}

impl Parser {
    fn location_for(&self, seed: &Seed) -> Option<&Location> {
        let soil = self.seed_to_soil.get(&seed)?;
        let fert = self.soil_to_fertilizer.get(&soil)?;
        let water = self.fertilizer_to_water.get(&fert)?;
        let light = self.water_to_light.get(&water)?;
        let temp = self.light_to_temperature.get(&light)?;
        let humi = self.temperature_to_humidity.get(&temp)?;
        let loc = self.humidity_to_location.get(&humi)?;
        return Some(loc);
    }
}

fn main() {
    println!("Hello, world!");
}

fn parse(input: &str) -> Location {
    return Location(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = {
            "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"
        };

        let result = parse(&input);
        dbg!(result);
    }
}
