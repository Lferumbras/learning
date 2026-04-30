// COnceitos de mut e shadowing



fn main() {
    //sem o mut daria erro ao tentar mudar coisas em height
    //mut faz com que seja possível alterar a variavel
    let mut height = 190;

    height = height - 20;

    let result = if height > 180 {
        "Tall"
    } else if height > 170 {
        "Average"
    } else {
        "Short"
    };

    println!("Result: {}", result);


    let health = if height < 180 {"good"} else {"unknown"}; 
    println!("Health: {}", health);      
    
    //Shadowing (atribuir valor a variavel com o let)
    let health = if height < 180 {true} else {false};
    println!("Health: {}", health);


    //RESUMO
    //COM MUT
    let mut height = 190;
    height = height - 20;

    //Com shadowing
    let height = 190;
    let height = height - 20;
}
