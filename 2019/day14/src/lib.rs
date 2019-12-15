mod parser;
use helpers::Solvers;
pub fn solve() -> Solvers {
    Solvers::new(a::run, b::run)
}
mod a {
    use super::*;
    pub fn run() -> String {
        let reactions: Vec<Reaction> = helpers::loader::read_as_iter("../day/14/input.cmds")
            .map(parse_line)
            .collect();
        let (req, _prod) = NanoFactory::build(reactions).get_fuel();
        format!("{}", req)
    }
}
mod b {
    pub fn run() -> String {
        "bah".into()
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum CompoundType {
    Ore,
    Fuel,
    Other(String),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Compound {
    Fuel(i32),
    Ore(i32),
    Named(i32, String),
}
impl Compound {
    fn to_tuple(&self) -> (CompoundType, i32) {
        match self {
            Compound::Fuel(ammount) => (CompoundType::Fuel, *ammount),
            Compound::Ore(ammount) => (CompoundType::Ore, *ammount),
            Compound::Named(ammount, name) => (CompoundType::Other(name.clone()), *ammount),
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct RequiresToProduce(i32, i32);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Reaction {
    input: Vec<Compound>,
    output: Compound,
}

impl Reaction {
    fn requires(&self) -> Vec<(CompoundType, i32)> {
        self.input.iter().map(|c| c.to_tuple()).collect()
    }
    fn required(&self) -> Vec<CompoundType> {
        self.input.iter().map(|c| c.to_tuple().0).collect()
    }
    fn is_primary(&self) -> bool {
        match self.output {
            Compound::Fuel(_) => true,
            _ => false,
        }
    }
}

use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
struct NanoFactory {
    required_ore: HashMap<CompoundType, RequiresToProduce>,
}

impl NanoFactory {
    fn get_fuel(&self) -> (i32, i32) {
        let RequiresToProduce(req, prod) = self.required_ore[&CompoundType::Fuel];
        (req, prod)
    }
    fn has_all(data: &HashMap<CompoundType, RequiresToProduce>, keys: Vec<CompoundType>) -> bool {
        keys.iter().all(|k| data.contains_key(k))
    }
    fn get_first_satisfied(
        reactions: &HashSet<Reaction>,
        compounds: &HashMap<CompoundType, RequiresToProduce>,
    ) -> Option<Reaction> {
        for reaction in reactions {
            if NanoFactory::has_all(compounds, reaction.required()) {
                return Some(reaction.clone());
            }
        }
        None
    }
    pub fn build(reaction_list: Vec<Reaction>) -> NanoFactory {
        let mut reactions: HashSet<Reaction> = HashSet::new();
        let mut required_ore = HashMap::new();
        reaction_list.iter().for_each(|r| {
            if r.is_primary() {
                let (_compound_in, requires) = r.input[0].to_tuple();
                if required_ore
                    .insert(compound_out, RequiresToProduce(requires, produces))
                    .is_some()
                {
                    panic!("replaced something when creating the table");
                }
            } else {
                reactions.insert(r.clone());
            }
        });

        NanoFactory { required_ore }
    }
}

//helpers::loader::read_as_iter(path: &str)

fn parse_line(line: String) -> Reaction {
    match parser::from_reaction(&line) {
        Ok(("", reaction)) => reaction,
        Ok((rem, _)) => panic!("did not finish: {}", rem),
        Err(err) => panic!("{:?}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_0() {
        let lines: Vec<Reaction> = vec![
            "9 ORE => 2 A",
            "8 ORE => 3 B",
            "7 ORE => 5 C",
            "3 A, 4 B => 1 AB",
            "5 B, 7 C => 1 BC",
            "4 C, 1 A => 1 CA",
            "2 AB, 3 BC, 4 CA => 1 FUEL",
        ]
        .iter()
        .map(|l| parse_line(l.to_string()))
        .collect();
        let nf = NanoFactory::build(lines);
        //   assert_eq!(format!("{:?}", nf), "");
        let (ore, fuel) = nf.get_fuel();
        assert_eq!(ore, 165);
    }
    #[test]
    fn parse_1() {
        let lines: Vec<Reaction> = vec![
            "157 ORE => 5 NZVS",
            "165 ORE => 6 DCFZ",
            "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL",
            "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ",
            "179 ORE => 7 PSHF",
            "177 ORE => 5 HKGWZ",
            "7 DCFZ, 7 PSHF => 2 XJWVT",
            "165 ORE => 2 GPVTF",
            "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        ]
        .iter()
        .map(|l| parse_line(l.to_string()))
        .collect();
        let nf = NanoFactory::build(lines);
        //   assert_eq!(format!("{:?}", nf), "");
        let (ore, fuel) = nf.get_fuel();
        assert_eq!(ore, 13312);
    }
    #[test]
    fn parse_2() {
        let lines: Vec<Reaction> = vec![
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG",
            "17 NVRVD, 3 JNWZP => 8 VPVL",
            "53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL",
            "22 VJHF, 37 MNCFX => 5 FWMGM",
            "139 ORE => 4 NVRVD",
            "144 ORE => 7 JNWZP",
            "5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC",
            "5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV",
            "145 ORE => 6 MNCFX",
            "1 NVRVD => 8 CXFTF",
            "1 VJHF, 6 MNCFX => 4 RFSQX",
            "176 ORE => 6 VJHF",
        ]
        .iter()
        .map(|l| parse_line(l.to_string()))
        .collect();
        let nf = NanoFactory::build(lines);
        //   assert_eq!(format!("{:?}", nf), "");
        let (ore, fuel) = nf.get_fuel();
        assert_eq!(ore, 180697);
    }
    #[test]
    fn parse_3() {
        let lines: Vec<Reaction> = vec![
            "171 ORE => 8 CNZTR",
            "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL",
            "114 ORE => 4 BHXH",
            "14 VRPVC => 6 BMBT",
            "6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL",
            "6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT",
            "15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW",
            "13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW",
            "5 BMBT => 4 WPTQ",
            "189 ORE => 9 KTJDG",
            "1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP",
            "12 VRPVC, 27 CNZTR => 2 XDBXC",
            "15 KTJDG, 12 BHXH => 5 XCVML",
            "3 BHXH, 2 VRPVC => 7 MZWV",
            "121 ORE => 7 VRPVC",
            "7 XCVML => 6 RJRHP",
            "5 BHXH, 4 VRPVC => 5 LTCX",
        ]
        .iter()
        .map(|l| parse_line(l.to_string()))
        .collect();
        let nf = NanoFactory::build(lines);
        //   assert_eq!(format!("{:?}", nf), "");
        let (ore, fuel) = nf.get_fuel();
        assert_eq!(ore, 2210736);
    }
}
