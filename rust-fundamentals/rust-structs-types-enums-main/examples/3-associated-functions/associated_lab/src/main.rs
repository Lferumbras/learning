#[derive(Debug)]
struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
    }

    fn update_uri(&mut self, url:String) {
        self.uri = url;
    }

    //Método regular 
    /*fn from_email(&self, email: String) -> User{
        let val= email.split_once('@') ;
        //Some((user, domain)) 
        
        User{
            username: String::from(val.unwrap().0),
            email: String::from(email.clone()),
            uri: String::from(format!("https://{}.com", val.unwrap().0)),
            active: true
        }
    }*/

    //Função associativa
    fn from_email(email: String) -> Self {
        let username = email
            .split_once('@')
            .map(|(user, _domain)| user)
            .unwrap_or("unknown")
            .to_string();

        Self {
            username: username.clone(),
            email,
            uri: format!("https://{}.com", username),
            active: true,
        }
    }
}



fn main() {

    let mut new_user = User::new(
        String::from("alfredodeza"),
        String::from("alfreodeza@example.com"),
        String::from("https://alfredodeza.com"),
    );
    println!("Hello, {}!", new_user.username);
    println!("Account {} status is: {}", new_user.username, new_user.active);
    new_user.deactivate();
    println!("Account {} status is: {}", new_user.username, new_user.active);


    //retorna novo usuário com base no email
    //let person = new_user.from_email("john@hotmail.com".to_string());
    //println!("{:#?}",person);
    let person = User::from_email("john@hotmail.com".to_string());
    println!("{:#?}",person);

    //Atualiza a uri da instância new_user 
    new_user.update_uri("https://change_uri.com".to_string());
    println!("New uri is {}", new_user.uri);

    
}
