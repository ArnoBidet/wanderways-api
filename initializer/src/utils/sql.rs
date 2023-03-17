/// # Arguments
///
/// * `value_list` - A vector of strings string containing pre-formated data values. E.g : `("a") ("b")`
pub fn gen_insert(table_name: &str, schema: &str, value_line_list: Vec<String>) -> String {
    if value_line_list.len() < 1 {
        return String::from("");
    }
    format!(
        "INSERT INTO {} {} VALUES\n{};\n",
        table_name,
        schema,
        value_line_list.join(",\n").sql_escape()
    )
}

pub fn gen_value_line(values_list: Vec<&str>) -> String {
    format!(
        "({})",
        values_list
            .iter()
            .map(|val| {
                return if !val.eq(&"null") {
                    format!("\"{}\"", val)
                } else {
                    String::from(val.clone())
                };
            })
            .collect::<Vec<String>>()
            .join(", ")
    )
}

pub fn string_or_null(value: &Option<String>) -> String {
    match value {
        Some(val) => val.clone(),
        None => String::from("null"),
    }
}
trait SqlEscape {
    fn sql_escape(&mut self) -> Self;
}

impl SqlEscape for String {
    fn sql_escape(&mut self) -> Self {
        self.replace("'", "''")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_insert_test_empty() {
        assert_eq!(gen_insert("", "", vec![]), "");
    }

    #[test]
    fn gen_insert_test_one_value() {
        assert_eq!(
            gen_insert("test", "(test)", vec![String::from("test")]),
            "INSERT INTO test (test) VALUES\ntest;\n"
        );
    }

    #[test]
    fn gen_value_line_one() {
        assert_eq!(gen_value_line(vec!["test"]), "(\"test\")");
    }

    #[test]
    fn gen_value_multiple() {
        assert_eq!(
            gen_value_line(vec!["test", "test2"]),
            "(\"test\", \"test2\")"
        );
    }

    #[test]
    fn gen_value_none() {
        assert_eq!(gen_value_line(vec![]), "()");
    }

    #[test]
    fn gen_value_null_not_surrounded() {
        assert_eq!(gen_value_line(vec!["test", "null"]), "(\"test\", null)");
    }

    #[test]
    fn sql_escape_multiple() {
        assert_eq!(
            String::from("test'test'test").sql_escape(),
            "test''test''test"
        );
    }
}
