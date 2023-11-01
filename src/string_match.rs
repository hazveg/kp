fn compare(val0: usize, val1: usize, val2: usize) -> usize {
    let first = if val0 > val1 {
        val1
    } else {
        val0
    };

    let second = if val1 > val2 {
        val2
    } else {
        val1
    };

    if first > second {
        second
    } else {
        first
    }
}

// Levenshtein Distance
//
// Copied from here, refer to this in case not understood:
// https://www.youtube.com/watch?v=XYi2-LPrwm4
//
// I don't understand it anyway
pub fn minimum_distance(word0: &str, word1: &str) -> usize {
    let mut cache = vec![vec![0; word1.len() + 1]; word0.len() + 1];

    for j in 0..(word1.len() + 1) {
        cache[word0.len()][j] = word1.len() - j
    }
    for i in 0..(word0.len() + 1) {
        cache[i][word1.len()] = word0.len() - i
    }

    for i in (0..word0.len()).rev() {
        for j in (0..word1.len()).rev() {
            if word0.chars().nth(i) == word1.chars().nth(j) {
                cache[i][j] = cache[i + 1][j + 1];
            } else {
                cache[i][j] = 1 + compare(cache[i + 1][j], cache[i][j + 1], cache[i + 1][j + 1]);
            }
        }
    }

    cache[0][0]
}

#[cfg(test)]
mod tests {
    use crate::string_match::compare;

    use super::minimum_distance;

    #[test]
    fn vector_2d() {
        let word0 = "abc";
        let word1 = "doodle";

        assert_eq!(minimum_distance(word0, word1), 6);
    }
    
    #[test]
    fn dfae() {
        let val0 = 0;
        let val1 = 3;
        let val2 = 4;

        assert_eq!(val0, compare(val0, val1, val2));
    }
}
