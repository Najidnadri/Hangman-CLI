use std::io;



fn into_dash(s: &String) -> String {
    s
    .chars()
    .map(|x| match x {
        ' ' => return x,
        _ => return '-',
    })
    .collect()
}

fn show_char(char: String, string: String) -> String {
    string
    .chars()
    .map(|x| if char.contains(x) {
        return x
    } else {
        return '-'
    })
    .collect()

}


fn main() {
    println!("lets play hangman game!");
    println!("the words consist only lowerletter case");
    println!("please input only one char at a time");
    // print the man itself
    let string = String::from("hello darkness my old friend");
    let mut answered = String::new();
    let mut showed_string = into_dash(&string);
    println!("{}", showed_string);

    loop {
        //read input 
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "quit".to_string() {
            println!("bye bye");
            break;
        }
        println!("You typed: {}", input.trim());

        //check input
        let a = input.trim();
        if string.contains(&a) {
            if answered.contains(&a){
                println!("it is a repetition!");
                println!("remove hand");
            } else {
                //push into char
                answered.push_str(a);
                showed_string = show_char(answered.clone(), string.clone());
                println!("correct!");
                println!("{}", showed_string);
            }
        } else {
            println!("that char is not in the string!");
            println!("remove hands");
        }
    }
}
