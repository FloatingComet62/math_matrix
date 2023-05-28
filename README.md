# Matrix

- `items` - Items of the matrix in row by row order
- `order` - Order of the matrix

## Examples

```rust
use math_matrix::Matrix;
let matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], (3, 2));
let invalid_matrix = Matrix::new(vec![1.0, 2.0, 3.0], (3, 2));

assert_eq!(matrix.is_ok(), true);
assert_eq!(matrix.unwrap().order, (3, 2));
assert_eq!(invalid_matrix.is_ok(), false);
```

### Matrixes from functions

Using functions to describe the matrix

```rust
use math_matrix::Matrix;
// Function generated matrix
// i^2 + 3j - 7
let function_generated = Matrix::generate(|i, j| (i * i + 3 * j) as f64 - 7.0, (5, 5));
// -3  0   3   6   9
// 0   3   6   9   12
// 5   8   11  14  17
// 12  15  18  21  24
// 21  24  27  30  33

assert_eq!(function_generated[(1, 1)], -3.0);
assert_eq!(function_generated[(2, 1)], 0.0);
assert_eq!(function_generated[(3, 3)], 11.0);
assert_eq!(function_generated[(4, 3)], 18.0);
```

### Built in matrices

- `Row matrix` - A matrix with only 1 row
- `Column matrix` - A matrix with only 1 column
- `Null matrix` - A matrix with all zeros
- `Square matrix` - A matrix with equal number of rows and columns
- `Diagonal matrix` - A matrix with items only along the diagonal
- `Scalar matrix` - A diagonal matrix with only 1 value
- `Identity matrix` - A scalar matrix with the value of 1

```rust
use math_matrix::Matrix;
// Row matrix
let row_matrix = Matrix::row_matrix(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
assert_eq!(row_matrix[(1, 5)], 5.0);
assert_eq!(row_matrix.get(2, 5).is_err(), true);

// Column matrix
let column_matrix = Matrix::column_matrix(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
assert_eq!(column_matrix[(3, 1)], 3.0);
assert_eq!(column_matrix.get(3, 5).is_err(), true);

// Null matrix
let null_matrix = Matrix::null_matrix((10, 10));
assert_eq!(null_matrix[(5, 5)], 0.0);
assert_eq!(null_matrix[(10, 10)], 0.0);
assert_eq!(null_matrix[(9, 6)], 0.0);

// Square matrix
let square_matrix = Matrix::square_matrix(vec![1.0, 2.0, 3.0, 4.0]);
let invalid_square_matrix = Matrix::square_matrix(vec![1.0, 2.0, 3.0]);

assert_eq!(square_matrix.unwrap().order, (2, 2));
assert_eq!(invalid_square_matrix.is_ok(), false);

// Diagonal matrix
let diagonal_matrix = Matrix::diagonal_matrix(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);

assert_eq!(diagonal_matrix.order, (8, 8));
assert_eq!(diagonal_matrix[(4, 5)], 0.0);
assert_eq!(diagonal_matrix[(5, 5)], 5.0);
assert_eq!(diagonal_matrix[(7, 8)], 0.0);

/// Scalar matrix
let scalar_matrix = Matrix::scalar_matrix(5.0, 6);

assert_eq!(scalar_matrix.order, (6, 6));
assert_eq!(scalar_matrix[(3, 4)], 0.0);
assert_eq!(scalar_matrix[(5, 5)], 5.0);
assert_eq!(scalar_matrix[(3, 3)], 5.0);

/// Identity matrix
let identity_matrix = Matrix::identity_matrix(5);

assert_eq!(identity_matrix.order, (5, 5));
assert_eq!(identity_matrix[(3, 4)], 0.0);
assert_eq!(identity_matrix[(5, 5)], 1.0);
assert_eq!(identity_matrix[(3, 3)], 1.0);
```

### Traces

Traces are the diagonal items of a square matrix<br>

```rust
use math_matrix::Matrix;
// Traces
let random_matrix = Matrix::new(vec![6.0, 4.0, 87.0, 3.0, 6.0, 89.0, 6.0, 8.0, 4.0, 2.0, 45.0, 2.0, 5.0, 7.0, 9.0, 9.0], (4, 4)).unwrap();
let trace = random_matrix.trace();
// trace: [ 6, 89, 45, 9 ]

let no_trace_matrix = Matrix::new(vec![6.0, 4.0, 87.0, 3.0, 6.0, 89.0, 6.0, 8.0, 4.0, 2.0, 45.0, 2.0, 5.0, 7.0, 9.0], (5, 3)).unwrap();
let invalid_trace = no_trace_matrix.trace();

assert_eq!(invalid_trace.is_ok(), false);
assert_eq!(trace.is_ok(), true);
let trace = trace.unwrap();
assert_eq!(trace[0], 6.0);
assert_eq!(trace[2], 45.0);
```

### Getting and Setting items

- `get_row` - Returns **nth row**
- `get_column` - Returns **nth column**
- `get` - Get the item from the matrix
- `set` - Set the item from the matrix

```rust
use math_matrix::Matrix;
let mut matrix = Matrix::new(vec![6.0, 4.0, 87.0, 3.0, 6.0, 89.0, 6.0, 8.0, 4.0, 2.0, 45.0, 2.0, 5.0, 7.0, 9.0], (5, 3)).unwrap();
// 6   4   87
// 3   6   89
// 6   8   4
// 2   45  2
// 5   7   9

assert_eq!(matrix.get_row(1).unwrap(), vec![6.0, 4.0, 87.0]);
assert_eq!(matrix.get_column(1).unwrap(), vec![6.0, 3.0, 6.0, 2.0, 5.0]);
assert_eq!(matrix.get_column(2).unwrap(), vec![4.0, 6.0, 8.0, 45.0, 7.0]);
assert_eq!(matrix.get(3, 2).unwrap(), 8.0);
assert_eq!(matrix.get(5, 1).unwrap(), 5.0);

matrix.set(5, 1, 99.0);

assert_eq!(matrix.get(5, 1).unwrap(), 99.0);
```
