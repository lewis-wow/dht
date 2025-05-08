use utils::zip;

#[tokio::main]
async fn main() {
    let names = vec!["Alice", "Bob"];
    let scores = vec![90, 85];

    let zipped = zip(&names, &scores);

    for (name, score) in zipped {
        println!("{name}: {score}");
    }
}
