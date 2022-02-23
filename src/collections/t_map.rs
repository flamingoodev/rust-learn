use std::collections::HashMap;

pub fn t_map() {
    let mut scores: HashMap<String, u8> = HashMap::new();
    scores.insert("Math".to_string(), 60);
    scores.insert("English".to_string(), 99);
    scores.insert(String::from("Chinese"), 119);
    let option = scores.get("Math");
    match option {
        None => println!("Math does not exist."),
        Some(score) => println!("Math score is {}", score),
    }
    if let Some(score) = option {
        println!("Math score is {}", score);
    } else {
        println!("Math does not exist.");
    }
    // other create map
    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores: Vec<u8> = vec![10, 30];
    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scores size is {}", scores.len());
    let team_yellow = String::from("Yellow");
    let team_yellow_init_score: u8 = 60;
    scores.insert(&team_yellow, &team_yellow_init_score);
    println!("scores size is {}", scores.len());
    println!("team yellow score is {}", team_yellow_init_score);
    // insert will move ownership
    let mut mut_scores: HashMap<String, u8> = HashMap::new();
    let yellow = String::from("Yellow");
    let yellow_init_score: u8 = 60;
    mut_scores.insert(yellow, yellow_init_score);
    println!("mut_scores len is {}", mut_scores.len());
    // error: borrow of moved value: `yellow`
    // println!("yellow is {}", yellow);
    let key_yellow = String::from("Yellow");
    let score = mut_scores.get(&key_yellow);
    match score {
        None => println!("yellow does not exist."),
        Some(item) => println!("yellow team score is {}", item),
    }
    // if not use reference,the for loop will be move ownership
    for (k, v) in &mut_scores {
        println!("{}: {}", k, v);
    }
    println!("{:?}", mut_scores);
    // when you insert the same key to map, value will be override.
    mut_scores.insert(String::from("Yellow"), 100);
    println!("{:#?}", mut_scores);
    // if key does not exist, a default value will be inserted.
    mut_scores.entry(String::from("Blue")).or_insert(90);
    let score = mut_scores.entry(String::from("Red")).or_default();
    // need resolve reference
    *score += 1;
    println!("{:#?}", mut_scores);
}
