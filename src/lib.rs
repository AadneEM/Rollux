use regex::Regex;

#[cfg(test)]
mod lib_test;

fn parse_dice_segments<'a>(cmd: &'a str) -> Vec<regex::Captures<'a>> {
  let regex = Regex::new(r#"(?x)
    (?P<count>\d+)? # count (optional)
    d
    (?P<size>\d+) # dice size
  "#).expect("Failed to compile regex");

  regex.captures_iter(cmd).collect()
}
