use input_recorder::replay;

fn main() {
    let s = include_str!("../../helloworld.json");
    let mut slice = serde_json::from_str(s).unwrap();
    replay(slice);
}
