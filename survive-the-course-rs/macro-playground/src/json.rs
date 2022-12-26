use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

macro_rules! json {
    (null) => {
        $crate::json::Json::Null
    };
    ([$($arr:tt),*]) => {
        $crate::json::Json::Array(vec![$(json!($arr)),*])
    };
    ([$($arr:tt),*,]) => {
        json!([$($arr),*])
    };
    // This empty pattern makes sure the next pattern following does not
    // raise a warning of unnecessary `mut`. If there is no $key:$val,
    // there will be no `hm.insert` calls.
    ({}) => {
        $crate::json::Json::Object(::std::collections::HashMap::new())
    };
    ({$($key:tt: $val:tt),*}) => {
        {
            let mut hm = ::std::collections::HashMap::new();
            $(hm.insert(String::from($key), json!($val));)*
            $crate::json::Json::Object(hm)
        }
    };
    ({$($key:tt: $val:tt),*,}) => {
        json!({$($key: $val),*})
    };
    ($json:tt) => {
        $crate::json::Json::from($json)
    };
}

impl From<bool> for Json {
    fn from(val: bool) -> Self {
        Json::Boolean(val)
    }
}

impl From<f64> for Json {
    fn from(num: f64) -> Self {
        Json::Number(num)
    }
}

impl From<i32> for Json {
    fn from(num: i32) -> Self {
        Json::Number(num as f64)
    }
}

impl From<String> for Json {
    fn from(s: String) -> Self {
        Json::String(s)
    }
}

impl<'a> From<&'a str> for Json {
    fn from(s: &'a str) -> Self {
        Json::String(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;

    use crate::json::Json;

    #[test]
    fn test_json_null() {
        assert_eq!(json!(null), Json::Null);
    }

    #[test]
    fn test_json_number() {
        assert_eq!(json!(123), Json::Number(123.0));
    }

    #[test]
    fn test_json_array_with_and_without_trailing_comma() {
        assert_eq!(
            json!([1, 2, false, null, [2, 3],]),
            Json::Array(vec![
                Json::Number(1.0),
                Json::Number(2.0),
                Json::Boolean(false),
                Json::Null,
                Json::Array(vec![Json::Number(2.0), Json::Number(3.0)])
            ])
        );
    }

    #[test]
    fn test_json_object_with_and_without_trailing_comma() {
        assert_eq!(
            json!({
                "a": [1.0, "foo", null],
                "b": "bar",
                "c": {
                    "c1": [false, null,]
                },
                "d": {},
            }),
            Json::Object(hashmap! {
                "a".to_string() => Json::Array(vec![Json::Number(1.0), Json::String("foo".to_string()), Json::Null]),
                "b".to_string() => Json::String("bar".to_string()),
                "c".to_string() => Json::Object(hashmap!{
                    "c1".to_string() => Json::Array(vec![Json::Boolean(false), Json::Null])
                }),
                "d".to_string() => Json::Object(hashmap!{})
            })
        )
    }
}
