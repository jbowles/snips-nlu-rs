use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};
use std::iter::FromIterator;

use crate::errors::*;

pub trait Gazetteer: Send + Sync {
    fn contains(&self, value: &str) -> bool;
}

pub struct HashSetGazetteer {
    values: HashSet<String>,
}

impl HashSetGazetteer {
    pub fn from_reader<R: Read>(reader: R) -> Result<Self> {
        let reader = BufReader::new(reader);
        let mut values = HashSet::<String>::new();
        for line in reader.lines() {
            let word = line?;
            if !word.is_empty() {
                values.insert(word);
            }
        }
        Ok(Self { values })
    }
}

impl FromIterator<String> for HashSetGazetteer {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Self {
            values: HashSet::from_iter(iter),
        }
    }
}

impl Gazetteer for HashSetGazetteer {
    fn contains(&self, value: &str) -> bool {
        self.values.contains(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{Gazetteer, HashSetGazetteer};

    #[test]
    fn test_hashset_gazetteer() {
        // Given
        let gazetteer: &[u8] = r#"
dog
cat
bear
crocodile"#
            .as_ref();

        // When
        let gazetteer = HashSetGazetteer::from_reader(gazetteer);

        // Then
        assert!(gazetteer.is_ok());
        let gazetteer = gazetteer.unwrap();
        assert!(gazetteer.contains("dog"));
        assert!(gazetteer.contains("crocodile"));
        assert!(!gazetteer.contains("bird"));
    }
}
