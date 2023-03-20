# Test in Rust
```Rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let hello_world = "Hello, world!";
        assert_eq!(hello_world, "Hello, world!");
    }
}
```
and Produces
```
running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
I don't know the testing but guessing from the code `#[cfg(test)]` link's to ( --> ) cargo *I think.* Which in result you can see it run the mod test.
and then run's all the function inside the mod test which is `it_works()`.

    Not sure, if *assert_eq()* is the best way to create this test.
