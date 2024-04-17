// Define a trait for sortable objects
pub trait Sortable {
    fn compare(&self, other: &Self) -> bool;
}

// Define the sorting algorithms
pub mod sorting {
    use crate::Sortable;

    pub fn quick_sort<T: Sortable>(input: &mut [T]) {
        if input.len() <= 1 {
                return;
            }
            let pivot_index = partition(input);
            let (left, right) = input.split_at_mut(pivot_index);
            quick_sort(left);
            quick_sort(&mut right[1..]);
    }
    fn partition<T: Sortable>(input: &mut [T]) -> usize {
        let pivot_index = input.len() - 1;
        let mut i = 0;
        for j in 0..pivot_index {
            if input[j].compare(&input[pivot_index]) {
                input.swap(i, j);
                i += 1;
            }
        }
        input.swap(i, pivot_index);
        i
    }
    
    pub fn selection_sort<T: Sortable>(input: &mut [T]) {
        // Check if the length of the input is less than 2, return if true
        if input.len() < 2 {
            return;
        }

        // Iterate over the input array
        for i in 0..input.len() {
            // Find the index of the minimum element in the unsorted portion
            let swap_val = {
                // Initialize the minimum value and its index
                let mut min = &input[i];
                let mut index_min = i;
                
                // Iterate over the unsorted portion of the array
                for j in i + 1..input.len() {
                    // Update the minimum value and its index if a smaller element is found
                    if input[j].compare(min) {
                        min = &input[j];
                        index_min = j;
                    }
                }
                index_min // Return the index of the minimum element
            };

            // Swap the current element with the minimum element if they are different
            if i != swap_val {
                input.swap(i, swap_val);
            }
        }
    }

    pub fn insert_sort<T: Sortable>(input: &mut [T]) {
        if input.len() < 2 {return;}
    
        for i in 1..input.len() {
            let mut j = i;
            while j > 0 && input[j - 1].compare(&input[j]) {
                input.swap(j - 1, j);
                j -= 1;
            }
        }
    }

    pub fn merge_sort<T: Sortable + Copy>(input: &mut [T]) {
        if input.len() < 2 {return;}
    
        let len = input.len();
        let mid = len / 2;
        merge_sort(&mut input[..mid]);
        merge_sort(&mut input[mid..]);
    
        let mut tmp = Vec::with_capacity(len);
        let mut i = 0;
        let mut j = mid;
    
        while i < mid && j < len {
            if input[i].compare(&input[j]) {
                tmp.push(input[i]);
                i += 1;
            } else {
                tmp.push(input[j]);
                j += 1;
            }
        }
        if i < mid {
            tmp.extend_from_slice(&input[i..mid]);
        } else if j < len {
            tmp.extend_from_slice(&input[j..len]);
        }
    
        input.copy_from_slice(&tmp[..]);
    }
}

// Implement sorting functions for any type that implements Sortable
impl<T: PartialOrd> Sortable for T {
    fn compare(&self, other: &Self) -> bool {
        *self < *other
    }
}


// Tests for the sorting algorithms
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut numbers = vec![5, 2, 9, 1, 5, 6];
        sorting::quick_sort(&mut numbers);
        assert_eq!(numbers, vec![1, 2, 5, 5, 6, 9]);
    }

    #[test]
    fn test_selection_sort() {
        let mut words = vec!["banana", "apple","pear", "orange"];
        sorting::selection_sort(&mut words);
        assert_eq!(words, vec!["apple", "banana", "orange", "pear"]);
    }

    #[test]
    fn test_insert_sort(){
        // Test sorting custom structs using Insertion Sort
        #[derive(Debug, PartialEq)]
        struct Person {
            name: String,
            age: u32,
        }
        impl Sortable for Person {
            fn compare(&self, other: &Self) -> bool {
                self.age > other.age
            }
        }  
        let mut person = vec![
            Person { name: "Alice".to_string(), age: 25 },
            Person { name: "Bob".to_string(), age: 30 },
            Person { name: "Charlie".to_string(), age: 20 },
        ];
        sorting::insert_sort(&mut person);
        assert_eq!(person, vec![
            Person { name: "Charlie".to_string(), age: 20 },
            Person { name: "Alice".to_string(), age: 25 },
            Person { name: "Bob".to_string(), age: 30 },
        ]);
    }

    #[test]
    fn test_merge_sort(){
        let mut floats = vec![3.5, 1.2, 2.8, 0.5, 5.9];
        sorting::merge_sort(&mut floats);
        assert_eq!(floats, vec![0.5, 1.2, 2.8, 3.5, 5.9]);
    }
}