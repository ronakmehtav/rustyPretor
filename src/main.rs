fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let hello_world = "Hello, world!";
        assert_eq!(hello_world, "Hello, world!");
    }
}
