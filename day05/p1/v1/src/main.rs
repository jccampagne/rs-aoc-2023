use std::{
    collections::{binary_heap::Iter, HashMap},
    ffi::NulError,
    i32,
    ops::{Add, AddAssign, Sub}, env, fs,
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

#[derive(Debug)]
struct RangeMap<S, T, V>
where
    S: Gettable,
    T: Gettable,
    // T: Gettable<V = <S as Gettable>::V>,
{
    pub source: S,
    pub target: T,
    length: V,
}

impl<S, T> RangeMap<S, T, Number>
where
    S: Gettable<V = Number>,
    T: Gettable<V = Number>,
{
    fn is_in_source_range(&self, x: &S) -> bool {
        let x = x.getv();
        let var_name = self.source.getv() + self.length;
        (self.source.getv() <= x) && (x < var_name)
    }
    pub fn compute_target_for(&self, x: &S) -> Option<T> {
        let true = self.is_in_source_range(&x) else {
            return None;
        };
        let t = x.getv() - self.source.getv() + self.target.getv();
        return Some(T::new(t));
    }
}

#[derive(Debug)]
struct Map<S, T>
where
    S: Gettable<V = Number>,
    T: Gettable<V = Number>,
{
    range_maps: Vec<RangeMap<S, T, Number>>,
}

impl<S, T> Map<S, T>
where
    S: Gettable<V = Number>,
    T: Gettable<V = Number>,
{
    fn compute_target_for(&self, x: &S) -> Option<T> {
        for rm in &self.range_maps {
            match rm.compute_target_for(&x) {
                None => continue,
                Some(t) => return Some(t),
            }
        }
        let v = x.getv();
        Some(T::new(v))
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
impl Gettable for Soil {
    type V = Number;
    fn getv(&self) -> Self::V {
        return self.0;
    }
    fn new(x: Self::V) -> Self {
        Self(x)
    }
}

impl Gettable for Fertilizer {
    type V = Number;
    fn getv(&self) -> Self::V {
        return self.0;
    }
    fn new(x: Self::V) -> Self {
        Self(x)
    }
}

impl Gettable for Water {
    type V = Number;
    fn getv(&self) -> Self::V {
        return self.0;
    }
    fn new(x: Self::V) -> Self {
        Self(x)
    }
}

impl Gettable for Light {
    type V = Number;
    fn getv(&self) -> Self::V {
        return self.0;
    }
    fn new(x: Self::V) -> Self {
        Self(x)
    }
}

impl Gettable for Temperature {
    type V = Number;
    fn getv(&self) -> Self::V {
        return self.0;
    }
    fn new(x: Self::V) -> Self {
        Self(x)
    }
}
impl Gettable for Humidity {
    type V = Number;
    fn getv(&self) -> Self::V {
        return self.0;
    }
    fn new(x: Self::V) -> Self {
        Self(x)
    }
}

impl Gettable for Location {
    type V = Number;
    fn getv(&self) -> Self::V {
        return self.0;
    }
    fn new(x: Self::V) -> Self {
        Self(x)
    }
}

type SeedToSoilMap = Map<Seed, Soil>;
type SoilToFertilizerMap = Map<Soil, Fertilizer>;
type FertilizerToWaterMap = Map<Fertilizer, Water>;
type WaterToLightMap = Map<Water, Light>;
type LightToTemperatureMap = Map<Light, Temperature>;
type TemperatureToHumidityMap = Map<Temperature, Humidity>;
type HumidityToLocation = Map<Humidity, Location>;

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
    // fn parse

    fn location_for(&self, seed: &Seed) -> Option<Location> {
        let soil = dbg!(self.seed_to_soil.compute_target_for(seed)?);
        let fert = dbg!(self.soil_to_fertilizer.compute_target_for(&soil)?);
        let water = dbg!(self.fertilizer_to_water.compute_target_for(&fert)?);
        let light = dbg!(self.water_to_light.compute_target_for(&water)?);
        let temp = dbg!(self.light_to_temperature.compute_target_for(&light)?);
        let humi = dbg!(self.temperature_to_humidity.compute_target_for(&temp)?);
        let loc = dbg!(self.humidity_to_location.compute_target_for(&humi)?);
        return Some(loc);
    }
}

fn main() {
    let mut args = env::args();
    args.next();
    let input_file = args.next().unwrap();
    let contents = fs::read_to_string(input_file).unwrap();
    let result = parse(&contents);
    dbg!(&result);

}

fn parse_seeds(lines: &mut std::str::Lines) -> Vec<Seed> {
    let line = lines.next().unwrap();
    let mut splits = line.split(":");
    let _ = splits.next();
    let numbers = splits.next().unwrap().trim();
    let mut numbers = numbers.split(" ");
    let mut res: Vec<Seed> = Vec::new();
    for ns in numbers {
        let nn = ns.parse::<Number>().unwrap();
        res.push(Seed(nn));
    }
    return res;
}

fn parse_map<S, T>(lines: &mut std::str::Lines) -> Map<S, T>
where
    S: Gettable<V = Number>,
    T: Gettable<V = Number>,
{
    let _ = lines.next().unwrap();

    let mut res = Map::<S, T> {
        range_maps: Vec::new(),
    };

    loop {
        let line = lines.next();
        let Some(line) = line else {
            break;
        };
        let mut splits = line.split(" ");
        let Some(s_end) = splits.next() else {
            break;
        };
        if s_end == "" {
            break;
        };
        let s_start = splits.next().unwrap();
        let s_len = splits.next().unwrap();

        let n_end = s_end.parse::<Number>().unwrap();
        let n_start = s_start.parse::<Number>().unwrap();
        let n_len = s_len.parse::<Number>().unwrap();

        let map = RangeMap {
            source: S::new(n_start),
            target: T::new(n_end),
            length: n_len,
        };
        res.range_maps.push(map);
        // let mut compare
        res.range_maps.sort_by(|a, b| {
            let av = a.source.getv();
            let bv = b.source.getv();
            if av < bv {
                std::cmp::Ordering::Less
            } else if av == bv {
                std::cmp::Ordering::Equal
            } else {
                std::cmp::Ordering::Greater
            }
        });
    }

    return res;
}

fn parse(input: &str) -> Location {
    let mut lines = input.lines();
    let seeds = parse_seeds(&mut lines);
    dbg!(&seeds);

    lines.next(); // skip empty line
    let seed_to_soil_map = parse_map::<Seed, Soil>(&mut lines);
    dbg!(&seed_to_soil_map);
    let soil_to_fert_map = parse_map::<Soil, Fertilizer>(&mut lines);
    let fert_to_wate_map = parse_map::<Fertilizer, Water>(&mut lines);
    let wate_to_ligh_map = parse_map::<Water, Light>(&mut lines);
    let ligh_to_temp_map = parse_map::<Light, Temperature>(&mut lines);
    let temp_to_humi_map = parse_map::<Temperature, Humidity>(&mut lines);
    let humi_to_loca_map = parse_map::<Humidity, Location>(&mut lines);
    let parser = Parser {
        seeds,
        seed_to_soil: seed_to_soil_map,
        soil_to_fertilizer: soil_to_fert_map,
        fertilizer_to_water: fert_to_wate_map,
        water_to_light: wate_to_ligh_map,
        light_to_temperature: ligh_to_temp_map,
        temperature_to_humidity: temp_to_humi_map,
        humidity_to_location: humi_to_loca_map,
    };

    // for seed in &parser.seeds {
    //     let loc = parser.location_for(&seed);
    //     dbg!(loc);
    // };

    let locs: &Vec<Option<Location>> = &((&parser.seeds).into_iter())
        .map(|s: &Seed| parser.location_for(&s))
        .collect();
    assert!(locs.into_iter().all(|o: &Option<Location>| o.is_some()));
    let loc = locs
        .into_iter()
        .map(|o: &Option<Location>| match o {
            Some(loc) => loc,
            None => todo!(),
        })
        .min_by(|a, b| a.getv().cmp(&b.getv()));

    match loc {
        Some(loc) => (*loc).clone(),
        None => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // seeds: 79 14 55 13
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
