# Sorting Algorithm Library in Rust
The Sorting Library crate provides a collection of sorting algorithms implemented in Rust, along with a trait for defining sortable objects. This crate can be used to perform sorting operations on various data types, including integers, strings, custom structs, and more.
- Quick Sort
- Selection Sort
- Merge Sort
- Insertion Sort

## Installation
1. Clone the repository:
    ```bash
    git clone https://github.com/ornur/sorting-alg-rust
    ```
    ![gif](https://github.com/ornur/sorting-alg-rust/blob/main/images/1.gif)

2. To use this library in your Rust project, add it as a dependency in your `Cargo.toml` file::
    ```toml
    [dependencies]
    sorting_library = { path = "path/to/sorting-alg-rust" }
    ```
    Provide the correct path.

    ![gif](https://github.com/ornur/sorting-alg-rust/blob/main/images/2.gif)

## Usage
### Importing the Sorting Library
Import the sorting functions and the Sortable trait into your Rust code:
```rust
use sorting_library::sorting::{quick_sort, selection_sort}; // Import sorting functions
use sorting_library::Sortable; // Import the Sortable trait
```

<img src="https://github.com/ornur/sorting-alg-rust/blob/main/images/3.gif" width="300" alt="gif info">

### Using the Sorting Functions
You can now use the sorting functions provided by the library to sort various types of data. For example:

```rust
fn main() {
    let mut numbers = vec![5, 2, 9, 1, 5, 6];
    println!("Original numbers: {:?}", numbers);
    quick_sort(&mut numbers);
    println!("Sorted numbers: {:?}", numbers);

    let mut words = vec!["banana", "apple", "pear", "orange"];
    println!("Original words: {:?}", words);
    selection_sort(&mut words);
    println!("Sorted words: {:?}", words);
}
```

<img src="https://github.com/ornur/sorting-alg-rust/blob/main/images/4.gif" width="300" alt="gif info">

### Implementing Sortable for Custom Types
You can implement the Sortable trait for custom types to enable sorting operations on them. Here's an example:
```rust
// Import the Sortable trait
use sorting_library::Sortable;

// Define a struct that implements the Sortable trait
struct MyStruct {
    value: i32,
}

// Implement the Sortable trait for MyStruct
impl Sortable for MyStruct {
    fn compare(&self, other: &Self) -> bool {
        self.value < other.value
    }
}

fn main() {
    let a = MyStruct { value: 5 };
    let b = MyStruct { value: 10 };

    // Call the compare method from the Sortable trait
    if a.compare(&b) {
        println!("a is less than b");
    } else {
        println!("a is greater than or equal to b");
    }
}

```
![gif](https://github.com/ornur/sorting-alg-rust/blob/main/images/5.gif)
### Running Tests
To run the tests included in the sorting library, use the following command:
```bash
cargo test
```
![gif](https://github.com/ornur/sorting-alg-rust/blob/main/images/6.gif)
## Licence
- https://github.com/flakusha/sorting_rs/blob/master/LICENSE
- https://www.geeksforgeeks.org/sorting-algorithms/
