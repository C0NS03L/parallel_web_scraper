use rayon::prelude::*;

pub fn process_data(data: Vec<String>) -> Vec<String> {
    data.par_iter()
        .map(|item| item.to_uppercase())
        .filter(|item| !item.is_empty())
        .collect()
}
