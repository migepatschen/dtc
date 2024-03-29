use column::Column;
use itertools::Itertools;

mod column;

pub fn encode(key_1: &str, key_2: &str, text: &str) -> String {
    let prepared_key_1 = prepare_input(key_1);
    let prepared_key_2 = prepare_input(key_2);
    let prepared_text = prepare_input(text);
    let mut transposed_text = transpose(&prepared_key_1, &prepared_text);
    transposed_text = transpose(&prepared_key_2, &transposed_text);
    transposed_text
}

fn transpose(key: &str, text: &str) -> String {
    let mut columns = Column::from_str(key);
    let mut indices = (0..columns.len()).cycle();

    for c in text.chars() {
        if let Some(index) = indices.next() {
            columns[index].add(c)
        }
    }

    let mut transposed_text = String::new();
    for col in columns.iter().sorted() {
        transposed_text.push_str(col.value());
    }

    transposed_text
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::*;

    #[rstest]
    #[case("Apfel", "Kirsche", "Beispielklartext", "SLEEKXILRBIEATTP")]
    #[case("Hans", "Dampf", "Hallo Welt!", "WEALTHLOL")]
    #[case(
        "Lorem",
        "ipsum",
        "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam",
        "EEMETMIIGLSOESNRIOPSSUALSDNDTRTEOCIUMTOEICSALDIMDPRRTA"
    )]
    fn encoding(
        #[case] key1: &str,
        #[case] key2: &str,
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let encoded_text = encode(key1, key2, input);
        assert_eq!(expected.to_owned(), encoded_text);
    }

    #[rstest]
    fn preparation() {
        let prepared_input = prepare_input("input text with ä,ß.ö-ü;Ü:Ä#Ö áñ");
        //let prepared_input = prepare_input("h ä,ß.ö-ü;Ü:Ä#Ö ");
        assert_eq!("INPUTTEXTWITHÄSSÖÜÜÄÖÁÑ".to_owned(), prepared_input);
    }

    #[rstest]
    #[case("APFEL", "AEFLP")]
    #[case("KIRSCHE", "CEHIKRS")]
    fn key_sorting(#[case] key: &str, #[case] expected: &str) {
        let sorted_key: String = key.chars().sorted().collect();
        assert_eq!(expected.to_owned(), sorted_key);
    }
}
