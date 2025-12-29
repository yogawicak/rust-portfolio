use std::collections::HashMap;

#[test]
fn test_maps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("Scores: {:?}", scores);

    let team_name = String::from("Blue");
    if let Some(score) = scores.get(&team_name) {
        println!("{}: {}", team_name, score);
    }
}
