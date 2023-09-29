use chrono::{DateTime, Utc};
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub query);

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum OpTree {
    Eq(Value, Value),
    Neq(Value, Value),
    Lt(Value, Value),
    Lte(Value, Value),
    Gt(Value, Value),
    Gte(Value, Value),
    Within(Value, Value),
    TopK(Value, Value),
    And(Box<OpTree>, Box<OpTree>),
    Or(Box<OpTree>, Box<OpTree>),
    Not(Box<OpTree>),
    Value(Value),
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
        test_match!(
            r#""\"I'm a string!""#,
            Value::String(r#""I'm a string!"#.into())
        );
        test_match!(
            r#""I'm a string!""#,
            Value::String(r#"I'm a string!"#.into())
        );
        test_match!("$1", Value::ResourceTag(1));
        test_match!("$33", Value::ResourceTag(33));
    }

    #[test]
    fn parses_ops() {
        let parser = query::ScopeParser::new();
        macro_rules! lit {
            ($v:expr) => {
                Value::Literal($v.into())
            };
        }
        macro_rules! int {
            ($v: expr) => {
                Value::Integer($v)
            };
        }
        macro_rules! eq {
            ($a: expr, $b: expr) => {
                OpTree::Eq($a, $b)
            };
        }
        macro_rules! and {
            ($a: expr,$b: expr) => {
                OpTree::And(Box::new($a), Box::new($b))
            };
        }

        let inputs = ["a == 1", "a==1", "a ==1", "(a==1)", "a== 1", "((a==1))"];
        let expected = eq!(lit!("a"), int!(1));
        for input in inputs {
            assert_eq!(parser.parse(&input), Ok(expected.clone()));
        }

        let inputs = ["(a == 1) && (b==2)", "((a == 1) && (b==2))"];
        let expected = and!(eq!(lit!("a"), int!(1)), eq!(lit!("b"), int!(2)));
        for input in inputs {
            dbg!(&input);
            assert_eq!(parser.parse(&input), Ok(expected.clone()));
        }

        let expected = and!(
            and!(eq!(lit!("a"), int!(1)), eq!(lit!("b"), int!(2))),
            eq!(lit!("c"), int!(3))
        );
        assert_eq!(
            parser.parse("((a == 1) && (b == 2)) && (c == 3)"),
            Ok(expected.clone())
        );


        for (s, op) in [
            ("==", OpTree::Eq(lit!("a"), int!(1))),
            ("!=", OpTree::Neq(lit!("a"), int!(1))),
            ("<", OpTree::Lt(lit!("a"), int!(1))),
            ("<=", OpTree::Lte(lit!("a"), int!(1))),
            (">", OpTree::Gt(lit!("a"), int!(1))),
            (">=", OpTree::Gte(lit!("a"), int!(1))),
            ("within", OpTree::Within(lit!("a"), int!(1))),
            ("topk", OpTree::TopK(lit!("a"), int!(1))),
        ] {
            assert_eq!(parser.parse(&format!("a {} 1",s)), Ok(op));
        }
        assert_eq!(parser.parse("!(a == 1)"), Ok(OpTree::Not(Box::new(eq!(lit!("a"), int!(1))))));
        assert_eq!(parser.parse("!a"), Ok(OpTree::Not(Box::new(OpTree::Value(lit!("a"))))));
    }
}
