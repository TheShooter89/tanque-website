pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[tokio::main]
async fn main() {
    let count = add(2, 3);

    for i in 1..=count {
        println!("{}: hello world", i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
