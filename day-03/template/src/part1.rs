#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    let input = include_str!("input.txt");
    process(String::from(input))
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: String) -> String {
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(String::from("it works!"));
        assert_eq!(result, String::from("it works!"));
    }
}
