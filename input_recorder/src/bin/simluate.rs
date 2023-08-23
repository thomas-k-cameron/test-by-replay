use input_recorder::replay;

fn main() {
    let s = include_str!("../../helloworld.json");
    let slice = serde_json::from_str(s).unwrap();
    replay(slice);
}
