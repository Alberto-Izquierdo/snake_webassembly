use stdweb::unstable::TryInto;

pub fn get_random_color() -> String {
    let letters = "01234567890ABCDEF";
    let len = letters.len() as u32;
    let mut color: String = String::from("#");
    for _ in 0..6 {
        let random_value: usize = js! {return Math.floor(Math.random() * @{len})}
            .try_into()
            .unwrap();
        color.push(letters.chars().nth(random_value).unwrap());
    }
    color
}
