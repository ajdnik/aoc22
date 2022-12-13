use std::cmp::Ordering;
use crate::utils::file;
use log::{debug, info};

fn convert_to_sublist(list: &Vec<file::SignalParts<u32>>, start_index: usize) -> (Vec<file::SignalParts<u32>>, usize) {
    let mut start_count = 0;
    let mut index = start_index;
    let mut new_list = Vec::new();
    while list.len() > index {
        match list[index] {
            file::SignalParts::Start => {
                start_count += 1;
                new_list.push(list[index]);
            },
            file::SignalParts::End => {
                start_count -= 1;
                new_list.push(list[index]);
                if start_count == 0 {
                    index += 1;
                    break;
                }
            },
            _ => {
                new_list.push(list[index]);
            }
        }
        index += 1;
    }
    (new_list, index)
}

fn is_in_right_order(left: Vec<file::SignalParts<u32>>, right: Vec<file::SignalParts<u32>>) -> Ordering {
    let mut left_idx = 1;
    let mut right_idx = 1;
    while left.len() > left_idx && right.len() > right_idx {
        if let (file::SignalParts::Number(left_val), file::SignalParts::Number(right_val)) = (left[left_idx], right[right_idx]) {
            if left_val > right_val {
                return Ordering::Greater;
            } else if left_val < right_val {
                return Ordering::Less;
            }
            left_idx += 1;
            right_idx += 1;
        } else if file::SignalParts::Next == left[left_idx] && file::SignalParts::Next == right[right_idx] {
            left_idx += 1;
            right_idx += 1;
        } else if file::SignalParts::Start == left[left_idx] && file::SignalParts::Start == right[right_idx] {
            let (new_left, new_left_idx) = convert_to_sublist(&left, left_idx);
            let (new_right, new_right_idx) = convert_to_sublist(&right, right_idx);
            let order = is_in_right_order(new_left, new_right);
            if Ordering::Equal != order {
                return order;
            }
            left_idx = new_left_idx;
            right_idx = new_right_idx;
        } else if file::SignalParts::Start == left[left_idx] {
            let (new_left, new_left_idx) = convert_to_sublist(&left, left_idx);
            if file::SignalParts::End == right[right_idx] {
                return Ordering::Greater;
            }
            let new_right = vec![file::SignalParts::Start, right[right_idx], file::SignalParts::End];
            let order = is_in_right_order(new_left, new_right);
            if Ordering::Equal != order {
                return order;
            }
            left_idx = new_left_idx;
            right_idx += 1;
        } else if file::SignalParts::Start == right[right_idx] {
            let (new_right, new_right_idx) = convert_to_sublist(&right, right_idx);
            if file::SignalParts::End == left[left_idx] {
                return Ordering::Less;
            }
            let new_left = vec![file::SignalParts::Start, left[left_idx], file::SignalParts::End];
            let order = is_in_right_order(new_left, new_right);
            if Ordering::Equal != order {
                return order;
            }
            left_idx += 1;
            right_idx = new_right_idx;
        } else {
            break;
        }
    }
    if left.len() - left_idx < right.len() - right_idx {
        return Ordering::Less;
    } else if left.len() - left_idx > right.len() - right_idx {
        return Ordering::Greater;
    }
    Ordering::Equal
}

pub fn task1(path: &str) {
    if let Ok(input) = file::read_lines(path) {
        let signals = file::to_signals::<u32>(input);
        debug!("Found {} signals", signals.len());
        let mut idx = 1;
        let mut sum = 0;
        for i in (0..signals.len()).step_by(2) {
            if Ordering::Greater != is_in_right_order(signals[i].clone(), signals[i+1].clone()) {
                sum += idx;
            }
            idx += 1;
        }
        debug!("Checked {} pairs of signals", idx - 1);
        info!("Sum of indices of ordered signals is {}", sum);
    }
}

pub fn task2(path: &str) {
    if let Ok(input) = file::read_lines(path) {
        let mut signals = file::to_signals::<u32>(input);
        debug!("Found {} signals", signals.len());
        signals.push(vec![file::SignalParts::Start, file::SignalParts::Start, file::SignalParts::Number(2), file::SignalParts::End, file::SignalParts::End]);
        signals.push(vec![file::SignalParts::Start, file::SignalParts::Start, file::SignalParts::Number(6), file::SignalParts::End, file::SignalParts::End]);
        signals.sort_by(|left, right| is_in_right_order(left.clone(), right.clone()));
        let divider_2 = signals.iter().position(|signal| {
            signal.len() == 5 &&
            signal[0] == file::SignalParts::Start &&
            signal[1] == file::SignalParts::Start &&
            signal[2] == file::SignalParts::Number(2) &&
            signal[3] == file::SignalParts::End &&
            signal[4] == file::SignalParts::End
        });
        let divider_6 = signals.iter().position(|signal| {
            signal.len() == 5 &&
            signal[0] == file::SignalParts::Start &&
            signal[1] == file::SignalParts::Start &&
            signal[2] == file::SignalParts::Number(6) &&
            signal[3] == file::SignalParts::End &&
            signal[4] == file::SignalParts::End
        });
        info!("Decode key is {}", (divider_2.unwrap() + 1) * (divider_6.unwrap() + 1));
    } 
}
