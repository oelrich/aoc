use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::multi::separated_nonempty_list;
use nom::sequence::tuple;
use nom::IResult;

use super::CompoundType;

fn from_other(input: &str) -> IResult<&str, CompoundType> {
  let (input, name) = alpha1(input)?;
  Ok((input, CompoundType::Other(name.to_owned())))
}
fn from_ore(input: &str) -> IResult<&str, CompoundType> {
  let (input, _ore) = tag("ORE")(input)?;
  Ok((input, CompoundType::Ore))
}
fn from_fuel(input: &str) -> IResult<&str, CompoundType> {
  let (input, _fuel) = tag("FUEL")(input)?;
  Ok((input, CompoundType::Fuel))
}

fn from_compound_type(input: &str) -> IResult<&str, CompoundType> {
  let (input, cpt) = alt((from_fuel, from_ore, from_other))(input)?;

  Ok((input, cpt))
}

use super::Compound;
fn from_compound(input: &str) -> IResult<&str, Compound> {
  let (input, (_, quantity, _, cpt)) = tuple((space0, digit1, space1, from_compound_type))(input)?;
  let quantity = quantity.parse::<i32>().unwrap();
  let compound = match cpt {
    CompoundType::Fuel => Compound::Fuel(quantity),
    CompoundType::Ore => Compound::Ore(quantity),
    CompoundType::Other(name) => Compound::Named(quantity, name),
  };

  Ok((input, compound))
}
fn many1_compound(input: &str) -> IResult<&str, Vec<Compound>> {
  let (rem, res) = separated_nonempty_list(char(','), from_compound)(input)?;
  Ok((rem, res))
}
use super::Reaction;
pub fn from_reaction(input: &str) -> IResult<&str, Reaction> {
  let (input, (compound_list, _, _, _, _, compound)) = tuple((
    many1_compound,
    space0,
    char('='),
    char('>'),
    space0,
    from_compound,
  ))(input)?;

  Ok((
    input,
    Reaction {
      input: compound_list,
      output: compound,
    },
  ))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{Compound, Reaction};
  #[test]
  fn compound_type_0() {
    let (rem, res) = from_compound_type("ORE").unwrap();
    assert_eq!(rem, "");
    assert_eq!(res, CompoundType::Ore);
  }
  #[test]
  fn compound_type_1() {
    let (rem, res) = from_compound_type("FUEL").unwrap();
    assert_eq!(rem, "");
    assert_eq!(res, CompoundType::Fuel);
  }
  #[test]
  fn compound_type_2() {
    let (rem, res) = from_compound_type("OTTER").unwrap();
    assert_eq!(rem, "");
    assert_eq!(res, CompoundType::Other("OTTER".to_owned()));
  }
  #[test]
  fn from_compound_0() {
    let (rem, res) = from_compound("9 ORE").unwrap();
    assert_eq!(rem, "");
    assert_eq!(res, Compound::Ore(9));
  }
  #[test]
  fn many_compound_0() {
    let (_rem, res) = many1_compound("9 ORE, 1232 BOB").unwrap();
    assert_eq!(
      res,
      vec![Compound::Ore(9), Compound::Named(1232, "BOB".to_owned())]
    );
  }
  #[test]
  fn from_reaction_0() {
    let (_rem, res) = from_reaction("9 ORE => 1232 BOB").unwrap();
    assert_eq!(
      res,
      Reaction {
        input: vec![Compound::Ore(9)],
        output: Compound::Named(1232, "BOB".to_owned())
      }
    );
  }
}
