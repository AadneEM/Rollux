use regex::Regex;
use thiserror::Error;
use anyhow::Result;
use rand::Rng;
use rand::seq::SliceRandom;

#[cfg(test)]
mod lib_test;

#[derive(Debug, PartialEq, Clone)]
enum DiceFilter {
  DropLowest(usize), DropHighest(usize), KeepLowest(usize), KeepHighest(usize),
}

#[derive(Debug, PartialEq, Clone)]
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
  #[error("No dice size was given")]
  SizeIsMissing,
  #[error("Invalid filter operator: {op}")]
  InvalidFilterOperator{op: String},
  #[error("Incomplete filter")]
  IncompleteFilter,
}

fn construct_dice_filter(op: &str, amount: usize) -> Result<DiceFilter> {
  match op.to_lowercase().as_str() {
    "d" | "dl" => Ok(DiceFilter::DropLowest(amount)),
    "dh" => Ok(DiceFilter::DropHighest(amount)),
    "k" | "kh" => Ok(DiceFilter::KeepHighest(amount)),
    "kl" => Ok(DiceFilter::KeepLowest(amount)),
    _ => Err(SegmentError::InvalidFilterOperator{op: op.to_owned()})?,
  }
}

fn construct_dice_segment(cap: regex::Captures<'_>) -> Result<Segment> {
  let op = cap.name("op").and_then(|i| i.as_str().chars().next()).unwrap_or('+');
  let modifier = cap.name("mod").map(|i| i.as_str().parse()).transpose()?;

  if let Some(amount) = modifier {
    return Ok(Segment::Modifier{ op, amount });
  }

  let count:i32 = cap.name("count").map(|i| i.as_str().parse()).transpose()?.unwrap_or(1);
  let size:i32 = cap.name("size").map(|i| i.as_str().parse()).transpose()?.ok_or(SegmentError::SizeIsMissing)?;
  let filter_amount = cap.name("filter").map(|i| i.as_str().parse::<usize>()).transpose()?;
  let filter_op = cap.name("filter_op").map(|op| op.as_str());
  /*
  dbg!(filter_amount);
  dbg!(filter_op);
  */
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

#[derive(Debug, PartialEq, Clone)]
struct RollWithModifier {
  diceroll: Option<Segment>,
  modifiers: Vec<Segment>
}

fn group_modifiers_to_dicerolls(segments: &[Segment]) -> Vec<RollWithModifier> {
  let mut results = Vec::new();

  let mut current_diceroll : Option<Segment> = None;
  let mut current_modifiers = Vec::new();

  for segment in segments {
    if let Segment::DiceRoll {..} = segment {
      if current_diceroll.is_some() || current_modifiers.len() > 0 {
        let with_modifier = RollWithModifier {
          diceroll: current_diceroll.clone(),
          modifiers: current_modifiers.clone(),
        };

        current_modifiers.clear();

        results.push(with_modifier);
      }

      current_diceroll = Some(segment.clone());
    }
    else if let Segment::Modifier { .. } = segment {
      current_modifiers.push(segment.clone());
    }
  }

  if current_diceroll.is_some() || current_modifiers.len() > 0 {
    let with_modifier = RollWithModifier {
      diceroll: current_diceroll.clone(),
      modifiers: current_modifiers.clone(),
    };

    current_modifiers.clear();

    results.push(with_modifier);
  }

  results
}

#[derive(Debug, PartialEq, Clone)]
struct Roll {
  operator: char,
  results: Vec<i32>,
  total: i32,
}

#[derive(Error, Debug, PartialEq, Clone)]
enum RollError {
  #[error("Roll failed. Empty RollWithModifier")]
  EmptyRollWithModifier,
  #[error("Roll failed. Invalid filter operator '{0}'")]
  InvalidFilterOperator(char),
  #[error("Roll failed. Invalid segment operator '{0}'")]
  InvalidOperator(char),
}

fn roll_dice_segments<R: Rng>(rwms: &[RollWithModifier], mut rng: R) -> Result<Vec<Roll>, RollError> {
  rwms.iter()
    .map(|rwm| {
      let mut results = Vec::new();
      let mut total = 0;
      
      // store first operator
      let operator = if let Some(Segment::DiceRoll{op, ..}) = rwm.diceroll {
        op
      } else if let Some(Segment::Modifier{op, ..}) = rwm.modifiers.iter().next() {
        *op
      } else {
        return Err(RollError::EmptyRollWithModifier);
      };

      if let Some(Segment::DiceRoll{count, size, filter, ..}) = &rwm.diceroll {
        results = (0..*count).map(|_| {
          rng.gen_range(1, size + 1)
        }).collect();

        results.sort();
        if let Some(filter) = filter {
          match filter {
            DiceFilter::DropLowest(n) => {
              results = results.into_iter().skip(*n).collect();
            },
            DiceFilter::DropHighest(n) => {
              let len = results.len();
              results = results.into_iter().take(len - n).collect();
            },
            DiceFilter::KeepLowest(n) => {
              results = results.into_iter().take(*n).collect();
            },
            DiceFilter::KeepHighest(n) => {
              let len = results.len();
              results = results.into_iter().skip(len - n).collect();
            },
          }
        }
        results.shuffle(&mut rng);
        
        total = results.iter().sum();
      }

      for segment in &rwm.modifiers {
        if let Segment::Modifier{op,amount} = segment {
          match op {
            '+' => { total += amount; },
            '-' => { total -= amount; },
            '/' => { total /= amount; },
            '*' => { total *= amount; },
            _ => {
              return Err(RollError::InvalidFilterOperator(*op));
            },
          }
        }
      }

      Ok(Roll{operator, results, total})
    })
    .collect::<Result<Vec<_>,_>>()
}

fn roll_dice<R : Rng>(s: &str, mut rng: R) -> Result<(i32, Vec<(Roll, Vec<Segment>)>)> {
  let segments = parse_dice_segments(s)?;

  let groups = group_modifiers_to_dicerolls(&segments);

  let rolls = roll_dice_segments(&groups, &mut rng)?;

  let total = rolls.iter().try_fold(0, |acc,roll| match roll.operator {
    '+' => Ok(acc + roll.total),
    '-' => Ok(acc - roll.total),
    '*' => Ok(acc * roll.total),
    '/' => Ok(acc / roll.total),
    _ => Err(RollError::InvalidOperator(roll.operator))
  })?;

  let rolls_with_modifiers = rolls.into_iter()
    .zip(groups.into_iter().map(|grp| grp.modifiers)).collect();

  Ok((total, rolls_with_modifiers))
}
