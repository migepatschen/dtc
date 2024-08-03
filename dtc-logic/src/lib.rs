use column::{Column, DtcColumnSort};
use unicode_segmentation::UnicodeSegmentation;

mod column;

pub fn encode(key_1: &str, key_2: &str, text: &str) -> String {
    let prepared_key_1 = prepare_input(key_1);
    let prepared_key_2 = prepare_input(key_2);
    let prepared_text = prepare_input(text);
    let mut transposed_text = transpose(&prepared_key_1, &prepared_text);
    transposed_text = transpose(&prepared_key_2, &transposed_text);
    group_output(&transposed_text)
}

pub fn decode(key_1: &str, key_2: &str, text: &str) -> String {
    let prepared_key_1 = prepare_input(key_1);
    let prepared_key_2 = prepare_input(key_2);
    let prepared_text = prepare_input(text);
    let mut transposed_text = reverse_transpose(&prepared_key_2, &prepared_text);
    transposed_text = reverse_transpose(&prepared_key_1, &transposed_text);
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

    columns.sort_by_index();
    let mut transposed_text = String::new();
    for col in columns {
        transposed_text.push_str(col.value());
    }

    transposed_text
}

fn reverse_transpose(key: &str, text: &str) -> String {
    let mut columns = Column::from_str(key);
    columns.sort_by_index();

    let (max_chars, max_chars_columns) = {
        let quotient = text.len() / key.len();
        let remainder = text.len() % key.len();
        if remainder == 0 {
            (quotient, 0)
        } else {
            (quotient + 1, remainder)
        }
    };

    let mut chars = text.graphemes(true);
    for (i, col) in columns.iter_mut().enumerate() {
        let max = if i < max_chars_columns {
            max_chars
        } else {
            max_chars - 1
        };

        for _ in 0..max {
            if let Some(c) = chars.next() {
                col.add_str(c);
            }
        }
    }

    columns.sort_by_original_index();
    let mut matrix: Vec<&str> = Vec::new();
    for col in &columns {
        matrix.push(col.value());
    }
    let mut transposed_text = String::new();

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

fn group_output(output: &str) -> String {
    let mut counter: usize = 0;
    let mut grouped_output = String::with_capacity(output.len() + 10);
    for c in output.chars() {
        grouped_output.push(c);
        if counter < 4 {
            counter += 1;
        } else {
            grouped_output.push(' ');
            counter = 0;
        }
    }
    grouped_output
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use pretty_assertions::assert_eq;
    use rstest::*;

    #[rstest]
    #[case("Apfel", "Kirsche", "Beispielklartext", "SLEEK XILRB IEATT P")]
    #[case("Hans", "Dampf", "Hallo Welt!", "WEALT HLOL")]
    #[case(
        "Lorem",
        "ipsum",
        "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam",
        "EEMET MIIGL SOESN RIOPS SUALS DNDTR TEOCI UMTOE ICSAL DIMDP RRTA"
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
    #[case("Apfel", "Kirsche", "SLEEKXILRBIEATTP", "BEISPIELKLARTEXT")]
    #[case("Hans", "Dampf", "WEALTHLOL", "HALLOWELT")]
    #[case(
        "Lorem",
        "ipsum",
        "EEMETMIIGLSOESNRIOPSSUALSDNDTRTEOCIUMTOEICSALDIMDPRRTA",
        "LOREMIPSUMDOLORSITAMETCONSETETURSADIPSCINGELITRSEDDIAM"
    )]
    #[case(
        "NOTEBOOK",
        "DECKEL",
        "NRSGS ESAIE OZRAB INADI ILURT NDEHX USRHE VIEEP AEHEE GTLZF TLIAN MEL",
        "HALLODASHIERISTEINLANGERBEISPIELTEXTUMDASVERFAHRENZUZEIGEN"
    )]
    fn decoding(
        #[case] key1: &str,
        #[case] key2: &str,
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let decoded_text = decode(key1, key2, input);
        assert_eq!(expected.to_owned(), decoded_text);
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

    #[rstest]
    #[case("APFEL", "BEISPIELKLARTEXT", "BEISPIELKLARTEXT")]
    fn decode_single(#[case] key: &str, #[case] text: &str, #[case] expected: &str) {
        let prepared_key = prepare_input(key);
        let prepared_text = prepare_input(text);
        let encoded_text = transpose(&prepared_key, &prepared_text);
        let decoded_text = reverse_transpose(&prepared_key, &encoded_text);
        assert_eq!(expected.to_owned(), decoded_text);
    }
}
