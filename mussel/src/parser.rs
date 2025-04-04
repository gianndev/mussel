pub enum Atom {
    String(String),
}

// This parser will expect a double quote, then it will take everything until the next double quote. 
// Then it will map it to an 'Atom'.
fn parse_string(input: &str) -> IResult<&str, Atom> {
    let parser = tag("\"");
}