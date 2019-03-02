

fn main() {

    // 1. Берем алфавит
    let alphabet_ru = vec!['а','б','в','г','д','е','ё','ж','з','и','й','к','л','м','н',
                           'о','п','р','с','т','у','ф','х','ц','ч','ш','щ','ъ','ы','ь',
                           'э','ю','я'];
    // 2. Берем текст, который шифруем
    let text_to_encrypt: Vec<char> = "Я супер хакер".chars().collect();

    // 3. Указываем цифру сдвига
    let shift_digit = 10;

    // 4. Шифруем
    let mut encrypted_text: Vec<char> = Vec::new();

    for (i, ch) in text_to_encrypt.iter().enumerate() {
        let new_char = alphabet_ru.get(i + shift_digit).unwrap();
        encrypted_text.push(*new_char);
        print!("{} ", ch);
    }

    println!("\nsource text: {:?}\nencrypted text: {:?}\n", text_to_encrypt, encrypted_text);

    // 5. Дешифруем
    // 6. Взламываем методом перебора по цифре сдвига

}
