use regex::Regex;
use thiserror::Error;
use anyhow::Result;

#[cfg(test)]
mod lib_test;

#[derive(Debug, PartialEq)]
enum DiceFilter {
  DropLowest(i32), DropHighest(i32), KeepLowest(i32), KeepHighest(i32),
}

#[derive(Debug, PartialEq)]
enum Segment {
  DiceRoll{
    op: char,
    count: i32,
    size: i32,
    filter: Option<DiceFilter>,
  },
  Modifier{
    op: char,
    amount: i32,
  },
}

#[derive(Error, Debug, PartialEq)]
enum SegmentError {
  #[error("Invalid segment")]
  InvalidSegment,
  #[error("No dice size was given")]
  SizeIsMissing,
  #[error("Invalid filter operator: {op}")]
  InvalidFilterOperator{op: String},
  #[error("Incomplete filter")]
  IncompleteFilter,
}

fn construct_dice_filter(op: &str, amount: i32) -> Result<DiceFilter> {
  match op.to_lowercase().as_str() {
    "d" | "dl" => Ok(DiceFilter::DropLowest(amount)),
    "dh" => Ok(DiceFilter::DropHighest(amount)),
    "k" | "kh" => Ok(DiceFilter::KeepHighest(amount)),
    "kl" => Ok(DiceFilter::KeepLowest(amount)),
    _ => Err(SegmentError::InvalidFilterOperator{op: op.to_owned()})?,
  }
}

fn construct_dice_segment(cap: regex::Captures) -> Result<Segment> {
  let op = cap.name("op").and_then(|i| i.as_str().chars().next()).unwrap_or('+');
  let modifier = cap.name("mod").map(|i| i.as_str().parse()).transpose()?;

  if let Some(amount) = modifier {
    return Ok(Segment::Modifier{ op, amount });
  }

  let count:i32 = cap.name("count").map(|i| i.as_str().parse()).transpose()?.unwrap_or(1);
  let size:i32 = cap.name("size").map(|i| i.as_str().parse()).transpose()?.ok_or(SegmentError::SizeIsMissing)?;
  let filter_amount = cap.name("filter").map(|i| i.as_str().parse::<i32>()).transpose()?;
  let filter_op = cap.name("filter_op").map(|op| op.as_str());
  dbg!(filter_amount);
  dbg!(filter_op);
  if filter_amount.is_some() != filter_op.is_some() {
    Err(SegmentError::IncompleteFilter)?;
  }
  let filter = filter_amount.and_then(|amount| (filter_op.map(|op| construct_dice_filter(op, amount)))).transpose()?;

  Ok(Segment::DiceRoll{op, count, size, filter})
}

fn parse_dice_segments(cmd: &str) -> Result<Vec<Segment>> {
  let regex = Regex::new(r#"(?x)
    (?P<op>[+\-/*])?
    \s*
    (?:
      (?:
        (?P<count>\d+)? # count (optional)
        d
        (?P<size>\d+) # dice size
        (?P<filter_op>[dk][hl]?)?
        (?P<filter>\d+)?
      ) | (?:
        (?P<mod>\d+)
      )
    )
  "#).expect("Failed to compile regex");

  regex.captures_iter(cmd).map(construct_dice_segment).collect()
}
