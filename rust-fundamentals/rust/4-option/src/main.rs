// IF let, match e conceito de option

fn find_user_age2(name: &str) -> Option<i32> {
    if name == "Thiago" {
        Some(30)
    } else {
        None
    }
}

fn read_name(name: String) {
    println!("{}", name);
}


fn find_user_age(name: &str) -> Option<Option<i32>> {
    if name == "Thiago" {
        Some(Some(30)) // usuário existe e tem idade cadastrada
    } else if name == "Maria" {
        Some(None) // usuário existe, mas idade não foi informada
    } else {
        None  // usuário não existe
    }
}

fn main2(){
    let name =  String::from("Thiago");

    read_name(name);
}

fn main() {

    //let maybe_number: Option<Option<()>> = None;

    
    let age = find_user_age("Maria");
    //let age = find_user_age("AAA");

    match age {
        Some(Some(value)) => println!("Idade: {}", value),
        Some(None) => println!("Usuário existe, mas idade não cadastrada"),
        None => println!("Usuário não encontrado"),
    }

    if let Some(Some(number)) = age {
        println!("The number is {:?}", number);
    } else {
        println!("There is no number");
    }
}


// IF let, match e conceito de option

fn read_name(name: &str) -> Option<i32> {
    if name == "Thiago" {
        Some(30)
    } else {
        None
    }
}

fn read_name_sem_ref(name: String) -> Option<i32> {
    if name == "Thiago" {
        Some(30)
    } else {
        None
    }
}

fn main() {

	//a variável name continua existindo após o read_name
    let name =  String::from("Thiago");

    read_name(&name);

    println!("{}", name);

	
	
	//a variável name é desalocada após read_name_sem_ref (deixa de existir)
    let name2 =  String::from("Thiago");

    read_name_sem_ref(name2);

    //println!("{}", name2);
}
