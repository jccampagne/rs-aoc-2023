use std::{
    collections::HashMap,
    i32,
    ops::{Add, AddAssign, Sub},
};

type Pos = i64;
type Number = i64;

trait Gettable<V> {
    type V: Add<V, Output=V> + PartialOrd;
    fn getv(&self) -> Self::V;
    fn new(x: Self::V) -> Self;
}
trait TypeEqual {}
impl<T> TypeEqual for (T, T) {}

struct MapRange<S: Gettable<V>, T: Gettable<V>, V>
where
    (S::V, T::V): TypeEqual,
    S::V: PartialOrd,
{
    source: S,
    target: T,
    length: S::V,
}

impl<S, T, V> MapRange<S, T, V>
where
    S: Gettable<V> + Add<Output = S::V> + Sub<Output = S::V>,
    T: Gettable<V> + Add<Output = T::V>,
    (S::V, T::V): TypeEqual,

    // (<S as Gettable>::V ,  <<S as Gettable>::V as Add>::Output) : TypeEqual,
    // (<S as Gettable>::V ,  S::V) : TypeEqual,

    // T: std::cmp::PartialOrd + Gettable + std::ops::Add + std::ops::Sub,
    // (<S as Gettable>::V, <T as Gettable>::V): TypeEqual,
    // (<S as Gettable>::V , <<S as Gettable>::V as Add>::Output): TypeEqual,
    // (<S as Gettable>::V, <<S as Gettable>::V as Add>::Output): TypeEqual,
{
    fn is_in_source_range(&self, x: S) -> bool {
        let x = x.getv();
        let var_name = self.source.getv() + self.length;
        (self.source.getv() <= x) && (x < var_name)
    }
    fn compute_target_for(&self, x: S) -> T {
        todo!()
        // assert!(self.is_in_source_range(x));
        // let t = x - self.source.getv() + self.target.getv();
        // return T::new(t);
    }
}

#[derive(Debug)]
struct Soil(Pos);

#[derive(Debug)]
struct Seed(Pos);

#[derive(Debug)]
struct Fertilizer(Pos);

#[derive(Debug)]
struct Water(Pos);

#[derive(Debug)]
struct Light(Pos);
#[derive(Debug)]
struct Temperature(Pos);
#[derive(Debug)]

struct Humidity(Pos);
#[derive(Debug)]
struct Location(Pos);

impl Gettable for Seed {
    type V = Pos;
    fn getv(&self) -> Self::V {
        return self.0;
    }
    fn new(x: Self::V) -> Self {
        Self(x)
    }
}

impl Gettable for Soil {
    type V = Pos;
    fn getv(&self) -> Self::V {
        return self.0;
    }
    fn new(x: Self::V) -> Self {
        Self(x)
    }
}

impl Gettable for Fertilizer {
    type V = Pos;
    fn getv(&self) -> Self::V {
        return self.0;
    }
    fn new(x: Self::V) -> Self {
        Self(x)
    }
}

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
