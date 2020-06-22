use regex::Regex;

#[cfg(test)]
mod lib_test;

fn parse_dice_segments<'a>(cmd: &'a str) -> Vec<regex::Captures<'a>> {
  let regex = Regex::new(r#"(?x)
    (?P<op>[+\-/*])?
    \s*
    (?:
      (?:
        (?P<count>\d+)? # count (optional)
        d
        (?P<size>\d+) # dice size
        (?:d(?P<drop>\d+))?
      ) | (?:
        (?P<mod>\d+)
      )
    )
  "#).expect("Failed to compile regex");

  regex.captures_iter(cmd).collect()
}
