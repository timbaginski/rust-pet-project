 mod musicians;


fn main() {
    let patrick_musicians = vec![
        musicians::Musician::Band(String::from("Rush"), 9), 
        musicians::Musician::SoloAct(String::from("Tame Impala"))
        ];
    
    let search_queries = [
        String::from("Rush"), 
        String::from("Tame Impala"),
        String::from("The Band")
    ];

    for query in search_queries.iter() {
        match musicians::search_artist(&patrick_musicians, query.to_string()) {
            Some(musician) => {
                match musician {
                    musicians::Musician::SoloAct(name) => {
                        println!("Solo act name is {}", name)
                    }
                    musicians::Musician::Band(name, members) => {
                        println!("Band is named {}, and has {} members", name, members)
                    }
                }
            }
            None => println!("Musician not found"), 
        }
    } 
}
