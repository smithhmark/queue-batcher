
fn message() -> String {
    String::from("Hello world")
}

fn main() {
    let msg = message();
    println!("{}", msg);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_startswith_hello() {
        let expected = String::from("Hello");
        assert_eq!(expected, message()[..expected.len()]);
        assert!(message().starts_with(&expected));
    }

    #[test]
    fn message_endswith_world() {
        let expected = String::from("world");
        let msg = message();
        assert_eq!(expected, msg[msg.len()-expected.len()..]);
        assert!(msg.ends_with(&expected));
    }
}
