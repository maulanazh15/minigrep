pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn one_result() {
        let query = "dukt";
        let contents = "\
Rust:
aman, cepat, produktif.
Ambil Tiga.";

        assert_eq!(vec!["aman, cepat, produktif."], search(query, contents));
    }
}
