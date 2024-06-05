use borsh::BorshSchema;

#[derive(BorshSchema)]
enum A {
    Bacon(Vec<i64>),
    Eggs,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_question_mark() {
        let mut defs = Default::default();
        A::add_definitions_recursively(&mut defs);
        println!("{:#?}", defs);
    }
}
