use std::{
    option::Option,
    ops::AddAssign,
};
use num::Num;

pub fn sum_continuous_numbers<N>(numbers: Vec<Option<N>>) -> Vec<N>
where N: Num + AddAssign + Copy {
   return numbers.iter().fold(Vec::new(), |mut acc, itm| {
        if acc.len() == 0 {
            acc.push(N::zero());
        }
        match itm {
            None => {
                acc.push(N::zero());
                acc
            },
            Some(val) => {
                if let Some(itm) = acc.last_mut() {
                    *itm += *val;
                }
                acc
            },
        }
   });
}

pub fn find_duplicate_chars(strings: &Vec<String>) -> String { 
    let mut duplicates = String::from("");
    if let Some((first, rest)) = strings.split_first() {
        for char in first.chars() {
            let mut count_found = 0;
            for itm in rest {
                if let Some(_) = itm.find(char) {
                    count_found += 1;
                }
            }
            if count_found == rest.len() {
                if let None = duplicates.find(char) {
                    duplicates += &char.to_string();
                }
            }
        }
    }
    duplicates
}
