use anyhow::Result;


#[derive(Clone)]
pub enum Operation {
    RemovePunkt,
    TrimSpaces,
    Lowercase,
    NGrams(String, usize)
}

#[derive(Clone)]
pub struct TextProcessor {
    operations: Vec<Operation>
}

impl TextProcessor {

    pub fn new() -> Self {
        Self {
            operations: Vec::new()
        }
    }

    pub fn pipe(mut self, operation: Operation) -> Self {
        self.operations.push(operation); 
        self
    }

    pub fn process(&self, text: String) -> Result<String> {
        let mut res = text.clone();
        for operation in &self.operations {
            res = match operation {
                Operation::RemovePunkt => self.remove_punkt(res),
                Operation::TrimSpaces => self.trim_spaces(res),
                Operation::Lowercase => res.to_lowercase(),
                Operation::NGrams(sep, n) => self.get_n_grams(sep, *n, res)
            }
        }
        Ok(res)
    }

    fn get_n_grams(&self, sep: &str, n: usize, text: String) -> String {
        let words:Vec<_> = text.split_whitespace().collect();
        words
            .windows(n)
            .map(|pair| pair.join(" "))
            .collect::<Vec<_>>()
            .join(sep)
    }

    fn trim_spaces(&self, text: String) -> String {
        text
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn remove_punkt(&self, text: String) -> String {
        text.chars()
            .filter(|c| !c.is_ascii_punctuation())
            .collect()
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_punkt() {
        let text = "Dog, can ! bark.;".to_string();
        let exp = "dog can bark".to_string();
        let res = TextProcessor::new()
            .pipe(Operation::RemovePunkt)
            .pipe(Operation::TrimSpaces)
            .pipe(Operation::Lowercase)
            .process(text)
            .unwrap();
        assert_eq!(res, exp);
    }

    #[test]
    fn test_ngrams() {
        let text = "a dog can bark".to_string();
        let exp = "a dog; dog can; can bark";
        let res = TextProcessor::new()
            .pipe(Operation::NGrams("; ".to_string(), 2))
            .process(text)
            .unwrap();
        assert_eq!(res, exp);
    }
}
