use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::{
    bytes::complete::take_till,
    bytes::complete::take_while,
    character::complete::{alpha1, char, digit1},
    character::complete::{multispace0, multispace1},
    combinator::{all_consuming, map, opt},
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug)]
pub struct UsdProperty {
    pub name: String,
    pub kind: String,
    pub value: String,
}
#[derive(Debug)]
pub struct UsdNode {
    pub name: String,
    pub kind: String,
    pub properties: Vec<UsdProperty>,
    pub children: Vec<UsdPart>,
}

#[derive(Debug)]
pub struct UsdComment(pub String);

#[derive(Debug)]
pub enum UsdPart {
    Node(UsdNode),
    Comment(UsdComment),
}
#[derive(Debug)]
pub struct Usd {
    pub parts: Vec<UsdPart>,
}

pub fn quoted_string(input: &str) -> IResult<&str, &str> {
    let (input, _) = char('"')(input)?;
    let (input, content) = take_till(|c: char| c == '"')(input)?;
    let (input, _) = char('"')(input)?;
    Ok((input, content))
}

pub fn parse_usd_node(input: &str) -> IResult<&str, UsdPart> {
    let (input, _) = multispace0(input)?;
    // look for string "def"
    let (input, _) = tag("def")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, kind) = alpha1(input)?;
    let (input, _) = multispace1(input)?;
    let (input, name) = quoted_string(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('{')(input)?;
    let (input, children) = opt(many0(parse_usd_node))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('}')(input)?;
    Ok((
        input,
        UsdPart::Node(UsdNode {
            name: String::from(name),
            kind: String::from(kind),
            properties: vec![],
            children: children.unwrap_or(vec![]),
        }),
    ))
}

fn parse_usd_comment(input: &str) -> IResult<&str, UsdPart> {
    let (input, _) = multispace0(input)?;
    let (input, _) = char('#')(input)?;
    let (input, comment) = take_while(|c: char| c != '\n' && c != '\r')(input)?;
    Ok((input, UsdPart::Comment(UsdComment(comment.to_string()))))
}

pub fn parse_usd_node_part(input: &str) -> IResult<&str, UsdPart> {
    let (input, _) = multispace0(input)?;
    alt((parse_usd_node, parse_usd_comment))(input)
}

pub fn parse_usd(input: &str) -> IResult<&str, Usd> {
    map(opt(many0(parse_usd_node_part)), |nodes| Usd {
        parts: nodes.unwrap_or(vec![]),
    })(input)
}

impl Usd {
    pub fn parse(buffer: &[u8]) -> Result<Usd, &'static str> {
        if let Ok(buffer_str) = std::str::from_utf8(buffer) {
            let r = parse_usd(buffer_str);
            match r {
                Ok((_, usd)) => {
                    return Ok(usd);
                }
                Err(e) => {
                    println!("{:?}", e);
                    return Err("Parse error");
                }
            }
        } else {
            return Err("Invalid UTF-8");
        }
    }
}
