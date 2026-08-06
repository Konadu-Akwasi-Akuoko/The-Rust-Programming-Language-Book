fn main() {
    println!("Hello, world!");
    print_name("Akwasi Konadu");
    print_age(&23);
    print_height(&700);
}

fn print_name(name: &str) {
    println!("My name is {}", name);
    println!("Hello, how are you?");
}

fn print_age(age: &i16) {
    println!("I am {} years old", age);
}

fn print_height(height: &i16) {
    println!("I am {} feet tall", height);
}
