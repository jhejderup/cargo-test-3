

fn quick_maths() -> i32 {
 2 + 2 - 1
}

fn hi() -> i32{
 4
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
