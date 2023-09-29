use chrono::{DateTime, Utc};
use lalrpop_util::lalrpop_mod;
use query::ScopeParser;


lalrpop_mod!(pub query);

#[derive(Debug, PartialEq)]
pub enum Value {
    Double(f64),
    Integer(i64),
    String(String),
    Bool(bool),
    DateTime(DateTime<Utc>),
    UUID(uuid::Uuid),
    ResourceTag(u32),
    Literal(String),
}

#[derive(Debug, PartialEq)]
pub enum OpTree {
    Eq(Value, Value),
    Neq(Value, Value),
    Lt(Value, Value),
    Lte(Value, Value),
    Gt(Value, Value),
    Gte(Value, Value),
    Within(Value, Value),
    TopK(Value, Value),
    ScopeOp(ScopeOpCode, Vec<OpTree>),
    Not(Box<OpTree>),
    Value(Value),
}

#[derive(Debug, PartialEq)]
pub enum ScopeOpCode {
    And,
    Or,
}

#[inline]
fn preprocess_input(input: &str) -> String {
    format!("({})", input.replace("&", "&amp;")
        .replace("\"", "&quot;"))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_value() {
        let parser = query::ValueParser::new();

        macro_rules! test_match {
            ($val: expr, $expected: expr) => {
                let input = format!("{}", $val);
                let expected = Ok($expected);
                assert_eq!(parser.parse(&input), expected);
            };
        }

        test_match!("0x0000A100", Value::Integer(0x0000A100));
        test_match!("1", Value::Integer(1));
        test_match!("1.0", Value::Double(1.0));
        test_match!("true", Value::Bool(true));
        test_match!("false", Value::Bool(false));
        test_match!(
            "'2020-01-01T00:00:00Z'",
            Value::DateTime(
                DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z")
                    .unwrap()
                    .into()
            )
        );
        test_match!(
            "'00000000-0000-0000-0000-000000000000'",
            Value::UUID(uuid::Uuid::nil())
        );
        
        let uuid = uuid::Uuid::new_v4();
        test_match!(format!("'{:?}'", uuid.clone()), Value::UUID(uuid));
        test_match!(r#"""I'm a string!"""#, Value::String(r#""I'm a string!""#.into()));
        test_match!(r#""I'm a string!""#, Value::String(r#"I'm a string!"#.into()));
        test_match!("$1", Value::ResourceTag(1));
        test_match!("$33", Value::ResourceTag(33));
    }



    #[test]
    fn parses_ops() {
        let parser = query::ScopeParser::new();

        let input = preprocess_input("a == 1");
        let expected = OpTree::Eq(
            Value::Literal("a".into()),
            Value::Integer(1),
        );
        assert_eq!(parser.parse(&input), Ok(expected));
    }
}
