#[cfg(test)]
mod tests {
    use derive_pattern::Pattern;

    #[derive(Pattern)]
    struct TestStruct {
        x: i32,
        y: i32,
    }

    #[test]
    fn it_works() {
        let test = TestStruct { x: 5, y: 10 };
        match test {
            test_struct_pattern!() => {
                assert!(x == 5);
                assert!(y == 5);
            }
            _ => unreachable!("pattern should have matched"),
        }
    }
}
