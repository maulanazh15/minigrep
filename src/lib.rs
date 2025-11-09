pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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
Ambil Tiga.
proDuktor Lakban.";

        assert_eq!(vec!["aman, cepat, produktif."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let contents = "\
Rust:
aman, cepat, produktif.
Ambil Tiga.
Trust padaku.";
        assert_eq!(
            vec!["Rust:", "Trust padaku."],
            search_case_insensitive(query, contents)
        );
    }
}
