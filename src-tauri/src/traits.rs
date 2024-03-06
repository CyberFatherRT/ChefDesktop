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
    fn capitalize(&self) -> String;
}

impl StringTrait for String {
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
