//! Math helper functions, mainly to perform all sorts of useful statistical
//! calculations.

// TODO: All functions should not only have a single-threaded, but also a multithreaded alternative
// for large data sets.

pub fn probabilities(occurences: &Vec<u64>) -> Vec<f64> {
    let total_occurences: u64 = occurences.iter().sum();

    occurences.clone().into_iter().map(|occ| occ as f64 / total_occurences as f64).collect()
}

pub fn simpsons_d(probabilities: &Vec<f64>) -> f64 {
    1. - probabilities.iter().fold(0., |sum, val| { sum + val*val })
}

pub fn simpsons_d_of_one(probabilities: &Vec<f64>) -> f64 {
    assert!(probabilities.len() > 1);

    let factor = probabilities.len() as f64 / (probabilities.len() - 1) as f64;
    factor * simpsons_d(&probabilities)
}

/// Calculate Leti's D. Keep in mind, to enter the probabilities in ranked order asc.
pub fn letis_d(ranked_probabilities: &Vec<f64>) -> f64 {
    let mut total = 0.;
    let added_probabilites: Vec<f64> = ranked_probabilities.into_iter().map(|x| { total += x; total }).collect();

    #[allow(non_snake_case)]
    added_probabilites.iter().fold(0., |acc, F| { acc + F * (1. - F) })
}

pub fn letis_d_of_one(ranked_probabilities: &Vec<f64>) -> f64 {
    assert!(ranked_probabilities.len() > 1);

    let factor = 4. / (ranked_probabilities.len() - 1) as f64;
    factor * letis_d(&ranked_probabilities)
}

pub fn entropy(probabilities: &Vec<f64>) -> f64 {
    - probabilities.iter().fold(0., |acc, p| { acc + p*p.log2() })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simpsons_d() {
        let occ = vec![40, 60, 20];
        let prob = probabilities(&occ);
        let d = simpsons_d(&prob);
        assert!(d >= 0.6111110);
        assert!(d <= 0.6111112);
    }

    #[test]
    fn test_simpsons_d_of_one() {
        let occ = vec![40, 60, 20];
        let prob = probabilities(&occ);
        let d = simpsons_d_of_one(&prob);
        assert!(d >= 0.9166666);
        assert!(d <= 0.9166668);
    }

    #[test]
    fn test_letis_d() {
        let occ = vec![20, 60, 40];
        let prob = probabilities(&occ);
        let d = letis_d(&prob);

        assert!(d >= 0.3611110);
        assert!(d <= 0.3611112);
    }

    #[test]
    fn test_letis_d_of_one() {
        let occ = vec![20, 60, 40];
        let prob = probabilities(&occ);
        let d = letis_d_of_one(&prob);

        assert!(d >= 0.7222221);
        assert!(d <= 0.7222223);
    }

    #[test]
    fn test_entropy() {
        let occ = vec![64, 30];
        let prob = probabilities(&occ);
        let ent = entropy(&prob);

        assert!(ent >= 0.9034535);
        assert!(ent <= 0.9034537);
    }
}
