#[macro_export]
macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut m = std::collections::HashMap::new();
        $(m.insert($k, $v);)*
        m
    }};
}

#[macro_export]
macro_rules! regex_check {
    ($regex:expr => $string:expr) => {{
        let regex = regex::Regex::new($regex).unwrap();
        regex.is_match($string)
    }};
}

#[macro_export]
macro_rules! create_me_daddy {
    () => {
        #[derive(Deserialize)]
        struct DeserializeMeDaddy {
            input: String,
            params: Params,
        }
    };
}

#[macro_export]
macro_rules! lang_me_daddy {
    () => {
        #[derive(Deserialize)]
        pub struct DeserializeMeDaddy {
            input: String,
            lang: String,
            params: Params,
        }
    };
}

#[macro_export]
macro_rules! create_info_struct {
    ($struct_name:ident, $name:ident, $doc:ident, $description_en:ident, $description_ru:ident, $info_url:ident) => {
        #[derive(Serialize)]
        pub struct $struct_name {
            name: &'static str,
            documentation: &'static str,
            description_en: &'static str,
            description_ru: &'static str,
            info_url: Option<&'static str>,
        }

        impl $struct_name {
            pub fn info() -> String {
                let structure = Self {
                    name: $name,
                    documentation: $doc,
                    description_en: $description_en,
                    description_ru: $description_ru,
                    info_url: $info_url,
                };
                serde_json::to_string(&structure).unwrap()
            }
        }
    };
}

#[macro_export]
macro_rules! run_op {
    ($op:ident, $struct:ident) => {
        pub fn $op(op: $struct, input: &str, request: &str) -> Result<String, String> {
            op.do_black_magic(input, request)
                .map_err(|err| err.to_string())
        }
    };
}
