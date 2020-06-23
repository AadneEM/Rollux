use super::*;

#[test]
fn test_roll_dice() {
  let mut results = parse_dice_segments("2d6").expect("Failed to unpack results");
  
  assert_eq!(results, vec![ Segment::DiceRoll{ op: '+', count: 2, size: 6, filter: None } ]);
  
  results = parse_dice_segments("2d6 + 1d20").expect("Failed to unpack results");
  
  assert_eq!(results, vec![ 
    Segment::DiceRoll{ op: '+', count: 2, size: 6, filter: None },
    Segment::DiceRoll{ op: '+', count: 1, size: 20, filter: None },
  ]);
  
  results = parse_dice_segments("4d6d1").expect("Failed to unpack results");
  
  assert_eq!(results, vec![ 
    Segment::DiceRoll{ op: '+', count: 4, size: 6, filter: Some(DiceFilter::DropLowest(1)) },
  ]);
    
  results = parse_dice_segments("3d8+8").expect("Failed to unpack results");

  assert_eq!(results, vec![ 
    Segment::DiceRoll{ op: '+', count: 3, size: 8, filter: None },
    Segment::Modifier{ op: '+', amount: 8 },
  ]);
  
  results = parse_dice_segments("3d8kl3 - 2d1dh1 / 2").expect("Failed to unpack results");
  
  assert_eq!(results, vec![ 
    Segment::DiceRoll{ op: '+', count: 3, size: 8, filter: Some(DiceFilter::KeepLowest(3)) },
    Segment::DiceRoll{ op: '-', count: 2, size: 1, filter: Some(DiceFilter::DropHighest(1)) },
    Segment::Modifier{ op: '/', amount: 2 },
  ]);
}
