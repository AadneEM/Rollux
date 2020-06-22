use super::*;

#[test]
fn test_roll_dice() {
  let mut results = parse_dice_segments("2d6");
  
  assert_eq!(results.len(), 1);
  assert_eq!(results[0]["count"], *"2");
  assert_eq!(results[0]["size"], *"6");
  
  results = parse_dice_segments("2d6 + 1d20");
  
  assert_eq!(results.len(), 2);
  assert_eq!(results[1].name("op").map(|i| i.as_str()), Some("+"));
  assert_eq!(results[1].name("count").map(|i| i.as_str()), Some("1"));
  assert_eq!(results[1]["size"], *"20");

  results = parse_dice_segments("4d6d1");
  
  assert_eq!(results.len(), 1);
  assert_eq!(results[0].name("drop").map(|i| i.as_str()), Some("1"));

  results = parse_dice_segments("3d8+8");

  assert_eq!(results.len(), 2);
  assert_eq!(results[1].name("op").map(|i| i.as_str()), Some("+"));
  assert_eq!(results[1].name("mod").map(|i| i.as_str()), Some("8"));
}
