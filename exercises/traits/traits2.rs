// traits2.rs
//
// Your task is to implement the trait
// `AppendBar' for a vector of strings.
//
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time,
// you can do this!
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

trait AppendBar {
    fn append_bar(self) -> Self;
}

//TODO: Add your code here
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push("Bar".to_string());
        self
        // if need to add in a loop
        // let mut new_vec: Vec<String> = Vec::new();
        // while let s = self.pop() {
        //     match s {
        //         Some(i) => new_vec.push(i),
        //         None => break,
        //     }
        // }
        // new_vec.push("Bar".to_string());
        // new_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
