#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    let input = include_str!("input.txt");
    process(input).to_string()
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    input.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process("0");
        assert_eq!(result, 0);
    }
}
