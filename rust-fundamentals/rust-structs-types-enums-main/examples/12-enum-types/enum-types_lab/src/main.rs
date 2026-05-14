/*
    Challenge Questions:

    Modify the code to add a new variant to the `WineRegions` enum, representing a wine region of your choice. Create a new `wine3` instance that 
    utilizes this new variant. Print information about `wine3` to verify your changes.

    Extend the program by implementing a function that takes a `WineRegions` value as input and returns a string representation of the region's popularity. 
    For example, for the region "Bordeaux", the function could return "Highly Popular." Invoke this function with different wine regions and print 
    the result to ensure it works correctly
*/


#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
    NewRegion
}

struct Wine {
    name: String,
    region: WineRegions,
}


fn get_popularity(w: &WineRegions) -> String {
    match w {
        WineRegions::Bordeaux => String::from("Highly Popular."),
        WineRegions::Burgundy => String::from("Medium Popular."),
        _ => String::from("unknown")
    }
}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    let wine3 = Wine {
        name: String::from("New"),
        region: WineRegions::NewRegion,
    };

    println!("Wine 3: {} from {:?}", wine1.name, wine1.region);
    println!("Wine 3: {} from {:?}", wine2.name, wine2.region);
    println!("Wine 3: {} from {:?}", wine3.name, wine3.region);


    println!(
        "The popularity of wines from region {:?} is {}",
        wine1.region,
        get_popularity(&wine1.region)
    );

    println!(
        "The popularity of wines from region {:?} is {}",
        wine2.region,
        get_popularity(&wine2.region)
    );

    println!(
        "The popularity of wines from region {:?} is {}",
        wine3.region,
        get_popularity(&wine3.region)
    );



}
