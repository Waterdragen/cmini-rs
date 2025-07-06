#[cfg(test)]
mod tests {
    use glob::glob;

    #[test]
    fn test_glob_corpora() {
        let pattern = "corpora/*";

        for entry in glob(pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => println!("{:?}", path),
                Err(e) => println!("Error: {:?}", e),
            }
        }
    }
}