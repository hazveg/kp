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

fn minimum_distance(word0: &str, word1: &str) -> usize {
    let mut cache = vec![vec![0; word1.len() + 1]; word0.len() + 1];

    for j in 0..(word1.len() + 1) {
        cache[word0.len()][j] = word1.len() - j
    }
    for i in 0..(word0.len() + 1) {
        cache[i][word1.len()] = word0.len() - i
    }

    for i in ((word0.len() - 1) as i32)..-1 {
        for j in ((word1.len() - 1) as i32)..-1 {
            if word0.chars().nth(i as usize) == word1.chars().nth(j as usize) {
                cache[i as usize][j as usize] = cache[i as usize + 1][j as usize + 1];
            } else {
                cache[i as usize][j as usize] = 1 + compare(cache[i as usize + 1][j as usize], cache[i as usize][j as usize + 1], cache[i as usize + 1][j as usize + 1]);
            }
        }
    }

    cache[0][0]
}

#[cfg(test)]
mod tests {
    use super::minimum_distance;

    #[test]
    fn vector_2d() {
        let word0 = "abc";
        let word1 = "doodle";

        assert_eq!(minimum_distance(word0, word1), 0);
    }
}
