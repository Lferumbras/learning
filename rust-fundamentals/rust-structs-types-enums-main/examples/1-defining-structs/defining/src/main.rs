
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    email: String,
    phone: String
}

fn get_full_name(people : &Person) -> String{
    format!("{} {}", people.first_name, people.last_name)
}

fn get_email(people : &Person) -> String{
    people.email.to_string()
}

fn main() {
    let people = Person {
        first_name: "Joselito".to_string(),
        last_name: "Doe".to_string(),
        age: 25,
        email: "doe_john@hotmail.com".to_string(),
        phone: "+55(62)994574978".to_string()
    };

    println!("{:?}", people);

    let full_name = get_full_name(&people);
    let email = get_email(&people);

    println!("{:?}, {:?}", full_name, email);
}