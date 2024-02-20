use std::collections::HashSet;

pub trait CharTrait {
    fn to_lower_case(&self) -> Self;
    fn to_upper_case(&self) -> Self;
}

impl CharTrait for char {
    fn to_lower_case(&self) -> char {
        match self {
            'A'..='Z' => self.to_ascii_lowercase(),
            'А'..='Я' | 'Ё' => self.to_uppercase().next().unwrap(),
            _ => *self,
        }
    }

    fn to_upper_case(&self) -> char {
        match self {
            'a'..='z' => self.to_ascii_uppercase(),
            'а'..='я' | 'ё' => self.to_uppercase().next().unwrap(),
            _ => *self,
        }
    }
}

pub trait StringTrait {
    fn _replace_by_alphabet(&self, alphabet: &str) -> String;

    fn _regex_replace_all(&self, regex: &str, replacement: &str) -> Result<String, String>;
    fn _regex_replace(&self, regex: &str, replacement: &str) -> Result<String, String>;

    fn capitalize(&self) -> String;
}

impl StringTrait for String {
    fn _replace_by_alphabet(&self, alphabet: &str) -> String {
        let alphabet: HashSet<char> = HashSet::from_iter(alphabet.chars());

        self.chars()
            .filter(|c| alphabet.contains(c))
            .collect::<String>()
    }

    fn _regex_replace_all(&self, regex_str: &str, replacement: &str) -> Result<String, String> {
        let re = regex::Regex::new(regex_str).map_err(|_| String::from("wrong regex"))?;
        let output: String = re.replace_all(regex_str, replacement).to_string();
        Ok(output)
    }

    fn _regex_replace(&self, regex_str: &str, replacement: &str) -> Result<String, String> {
        let re = regex::Regex::new(regex_str).map_err(|_| String::from("wrong regex"))?;
        let output: String = re.replace(regex_str, replacement).to_string();
        Ok(output)
    }

    fn capitalize(&self) -> String {
        let mut c = self.chars();
        match c.next() {
            None => String::new(),
            Some(f) => format!("{}{}", f.to_upper_case(), c.as_str()),
        }
    }
}

pub trait IntegerTrait {
    fn gcd(&self, other: &Self) -> Self;
}

impl IntegerTrait for i16 {
    fn gcd(&self, other: &Self) -> i16 {
        if *self == 0 {
            return *other;
        }
        if *other == 0 {
            return *self;
        }
        let mut a = *self;
        let mut b = *other;
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}
