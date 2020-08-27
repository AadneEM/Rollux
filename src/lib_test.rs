use super::*;
use rand::{rngs::StdRng, SeedableRng};

#[test]
fn test_parse_dice_segments() {
    let mut results = parse_dice_segments("2d6").expect("Failed to unpack results");

    assert_eq!(
        results,
        vec![Segment::DiceRoll {
            op: '+',
            count: 2,
            size: 6,
            filter: None
        }]
    );

    results = parse_dice_segments("2d6 + 1d20").expect("Failed to unpack results");

    assert_eq!(
        results,
        vec![
            Segment::DiceRoll {
                op: '+',
                count: 2,
                size: 6,
                filter: None
            },
            Segment::DiceRoll {
                op: '+',
                count: 1,
                size: 20,
                filter: None
            },
        ]
    );

    results = parse_dice_segments("4d6d1").expect("Failed to unpack results");

    assert_eq!(
        results,
        vec![Segment::DiceRoll {
            op: '+',
            count: 4,
            size: 6,
            filter: Some(DiceFilter::DropLowest(1))
        },]
    );

    results = parse_dice_segments("3d8+8").expect("Failed to unpack results");

    assert_eq!(
        results,
        vec![
            Segment::DiceRoll {
                op: '+',
                count: 3,
                size: 8,
                filter: None
            },
            Segment::Modifier { op: '+', amount: 8 },
        ]
    );

    results = parse_dice_segments("3d8kl3 - 2d1dh1 / 2").expect("Failed to unpack results");

    assert_eq!(
        results,
        vec![
            Segment::DiceRoll {
                op: '+',
                count: 3,
                size: 8,
                filter: Some(DiceFilter::KeepLowest(3))
            },
            Segment::DiceRoll {
                op: '-',
                count: 2,
                size: 1,
                filter: Some(DiceFilter::DropHighest(1))
            },
            Segment::Modifier { op: '/', amount: 2 },
        ]
    );
}

#[test]
fn test_group_modifiers_to_dicerolls() {
    let segments =
        parse_dice_segments("2d20 / 2 + 2 + 4d6 * 2 * 2").expect("Failed to unpack results");

    let results = group_modifiers_to_dicerolls(&segments);

    assert_eq!(
        results,
        vec![
            RollWithModifier {
                diceroll: Some(Segment::DiceRoll {
                    op: '+',
                    count: 2,
                    size: 20,
                    filter: None
                }),
                modifiers: vec![
                    Segment::Modifier { op: '/', amount: 2 },
                    Segment::Modifier { op: '+', amount: 2 },
                ]
            },
            RollWithModifier {
                diceroll: Some(Segment::DiceRoll {
                    op: '+',
                    count: 4,
                    size: 6,
                    filter: None
                }),
                modifiers: vec![
                    Segment::Modifier { op: '*', amount: 2 },
                    Segment::Modifier { op: '*', amount: 2 },
                ]
            },
        ]
    );
}

#[test]
fn test_group_modifiers_to_dicerolls_no_diceroll() {
    let segments = parse_dice_segments("* 2 * 2").expect("Failed to unpack results");

    let results = group_modifiers_to_dicerolls(&segments);

    assert_eq!(
        results,
        vec![RollWithModifier {
            diceroll: None,
            modifiers: vec![
                Segment::Modifier { op: '*', amount: 2 },
                Segment::Modifier { op: '*', amount: 2 },
            ]
        },]
    );
}

#[test]
fn test_roll_dice_segments() {
    let mut rng = StdRng::seed_from_u64(2);

    let segments = parse_dice_segments("4d6").expect("Failed to unpack results");
    let rolls_with_modifiers = group_modifiers_to_dicerolls(&segments);
    let results = roll_dice_segments(&rolls_with_modifiers, &mut rng);

    assert_eq!(
        results,
        Ok(vec![Roll {
            operator: '+',
            results: vec![4, 5, 5, 1],
            total: 15
        }])
    );

    let segments = parse_dice_segments("4d6d1 + 2").expect("Failed to unpack results");
    let rolls_with_modifiers = group_modifiers_to_dicerolls(&segments);
    let results = roll_dice_segments(&rolls_with_modifiers, &mut rng);

    assert_eq!(
        results,
        Ok(vec![Roll {
            operator: '+',
            results: vec![1, 1, 5],
            total: 9
        }])
    );

    let segments = parse_dice_segments("+ 2 + 2d12kh1 / 2").expect("Failed to unpack results");
    let rolls_with_modifiers = group_modifiers_to_dicerolls(&segments);
    let results = roll_dice_segments(&rolls_with_modifiers, &mut rng);

    assert_eq!(
        results,
        Ok(vec![
            Roll {
                operator: '+',
                results: vec![],
                total: 2
            },
            Roll {
                operator: '+',
                results: vec![12],
                total: 6
            },
        ])
    );

    let segments = parse_dice_segments("1d6 * 1d6 + 2 / 2").expect("Failed to unpack results");
    let rolls_with_modifiers = group_modifiers_to_dicerolls(&segments);
    let results = roll_dice_segments(&rolls_with_modifiers, &mut rng);

    assert_eq!(
        results,
        Ok(vec![
            Roll {
                operator: '+',
                results: vec![1],
                total: 1
            },
            Roll {
                operator: '*',
                results: vec![6],
                total: 4
            },
        ])
    );
}

#[test]
fn test_roll_dice() {
    let input = "4d6d2 + 2 - 1d4";

    let mut rng = StdRng::seed_from_u64(2);

    let result = roll_dice(input, &mut rng);

    let expected = RollSet {
        total: 8i32,
        rolls: vec![
            (
                Roll {
                    operator: '+',
                    results: vec![5, 5],
                    total: 12,
                },
                vec![Segment::Modifier { op: '+', amount: 2 }],
            ),
            (
                Roll {
                    operator: '-',
                    results: vec![4],
                    total: 4,
                },
                vec![],
            ),
        ],
    };

    if let Ok(result) = result {
        assert_eq!(result, expected);
    } else {
        panic!("Expected Ok, was: {:?}", result);
    }
}
