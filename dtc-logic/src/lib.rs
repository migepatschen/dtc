use itertools::Itertools;

pub fn encode(key_1: &str, key_2: &str, text: &str) -> String {
    let prepared_text = prepare_input(text);
    let transposed_text = transpose(key_1, &prepared_text);
    return transposed_text;
}

fn transpose(key: &str, text: &str) -> String {
    let mut columns = Column::from_str(key);
    let mut indices = (0..columns.len()).cycle();

    for c in text.chars() {
        if let Some(index) = indices.next() {
            columns[index].add(c)
        }
    }

    text.to_owned()
}

fn prepare_input(input: &str) -> String {
    let mut text = String::with_capacity(input.len());
    for c in input.chars() {
        if c.is_whitespace() || c.is_ascii_punctuation() {
            continue;
        }
        let uppercase_c = c.to_uppercase().to_string();
        text += &uppercase_c;
    }
    text
}

#[derive(Debug, Clone)]
struct Column {
    letter: char,
    index: usize,
    value: String,
}

impl Column {
    fn from_str(key: &str) -> Vec<Self> {
        let sorted_key: String = key.chars().sorted().collect();
        let mut columns: Vec<Self> = Vec::with_capacity(key.len());
        for c in key.chars() {
            let index = sorted_key.find(c).expect("the letter should be in the key");
            columns.push(Column {
                letter: c,
                index,
                value: String::new(),
            });
        }
        columns
    }

    fn add(&mut self, c: char) {
        self.value.push(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let encoded_text = encode("Apfel", "Kirsche", "Beispielklartext");
        assert_eq!("SLEEKXILRBIEATTP".to_owned(), encoded_text);
    }

    #[test]
    fn preparation() {
        let prepared_input = prepare_input("input text with ä,ß.ö-ü;Ü:Ä#Ö áñ");
        //let prepared_input = prepare_input("h ä,ß.ö-ü;Ü:Ä#Ö ");
        assert_eq!("INPUTTEXTWITHÄSSÖÜÜÄÖÁÑ".to_owned(), prepared_input);
    }
}
