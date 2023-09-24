use crate::{Determinant, Errors};
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

/// # Matrix
/// * `items` - Items of the matrix in row by row order
/// * `order` - Order of the matrix
///
/// ## Examples
/// ```
/// use math_matrix::Matrix;
/// let matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], (3, 2));
/// let invalid_matrix = Matrix::new(vec![1.0, 2.0, 3.0], (3, 2));
///
/// assert_eq!(matrix.is_ok(), true);
/// assert_eq!(matrix.unwrap().order, (3, 2));
/// assert_eq!(invalid_matrix.is_ok(), false);
/// ```
///
/// ### Matrixes from functions
/// Using functions to describe the matrix
/// ```
/// use math_matrix::Matrix;
/// // Function generated matrix
/// // i^2 + 3j - 7
/// let function_generated = Matrix::generate(|i, j| (i * i + 3 * j) as f64 - 7.0, (5, 5));
/// // -3  0   3   6   9
/// // 0   3   6   9   12
/// // 5   8   11  14  17
/// // 12  15  18  21  24
/// // 21  24  27  30  33
///
/// assert_eq!(function_generated[(1, 1)], -3.0);
/// assert_eq!(function_generated[(2, 1)], 0.0);
/// assert_eq!(function_generated[(3, 3)], 11.0);
/// assert_eq!(function_generated[(4, 3)], 18.0);
/// ```
///
/// ### Built in matrices
/// * `Row matrix` - A matrix with only 1 row
/// * `Column matrix` - A matrix with only 1 column
/// * `Null matrix` - A matrix with all zeros
/// * `Square matrix` - A matrix with equal number of rows and columns. Returns [`Result`], [`Ok`] if items can be arranged in a square, [`Err`] otheriwse
/// * `Diagonal matrix` - A matrix with items only along the diagonal
/// * `Scalar matrix` - A diagonal matrix with only 1 value
/// * `Identity matrix` - A scalar matrix with the value of 1
///
/// ```
/// use math_matrix::Matrix;
/// // Row matrix
/// let row_matrix = Matrix::row_matrix(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
/// assert_eq!(row_matrix[(1, 5)], 5.0);
/// assert_eq!(row_matrix.get(2, 5).is_err(), true);
///
/// // Column matrix
/// let column_matrix = Matrix::column_matrix(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
/// assert_eq!(column_matrix[(3, 1)], 3.0);
/// assert_eq!(column_matrix.get(3, 5).is_err(), true);
///
/// // Null matrix
/// let null_matrix = Matrix::null_matrix((10, 10));
/// assert_eq!(null_matrix[(5, 5)], 0.0);
/// assert_eq!(null_matrix[(10, 10)], 0.0);
/// assert_eq!(null_matrix[(9, 6)], 0.0);
///
/// // Square matrix
/// let square_matrix = Matrix::square_matrix(vec![1.0, 2.0, 3.0, 4.0]);
/// let invalid_square_matrix = Matrix::square_matrix(vec![1.0, 2.0, 3.0]);
///
/// assert_eq!(square_matrix.unwrap().order, (2, 2));
/// assert_eq!(invalid_square_matrix.is_ok(), false);
///
/// // Diagonal matrix
/// let diagonal_matrix = Matrix::diagonal_matrix(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
///
/// assert_eq!(diagonal_matrix.order, (8, 8));
/// assert_eq!(diagonal_matrix[(4, 5)], 0.0);
/// assert_eq!(diagonal_matrix[(5, 5)], 5.0);
/// assert_eq!(diagonal_matrix[(7, 8)], 0.0);
///
/// /// Scalar matrix
/// let scalar_matrix = Matrix::scalar_matrix(5.0, 6);
///
/// assert_eq!(scalar_matrix.order, (6, 6));
/// assert_eq!(scalar_matrix[(3, 4)], 0.0);
/// assert_eq!(scalar_matrix[(5, 5)], 5.0);
/// assert_eq!(scalar_matrix[(3, 3)], 5.0);
///
/// /// Identity matrix
/// let identity_matrix = Matrix::identity_matrix(5);
///
/// assert_eq!(identity_matrix.order, (5, 5));
/// assert_eq!(identity_matrix[(3, 4)], 0.0);
/// assert_eq!(identity_matrix[(5, 5)], 1.0);
/// assert_eq!(identity_matrix[(3, 3)], 1.0);
/// ```
/// ### Traces
/// Traces are the diagonal items of a square matrix<br>
/// Returns [`Result`], [`Ok`] if the matrix is square, [`Err`] otherwise
/// ```
/// use math_matrix::Matrix;
/// // Traces
/// let random_matrix = Matrix::new(vec![6.0, 4.0, 87.0, 3.0, 6.0, 89.0, 6.0, 8.0, 4.0, 2.0, 45.0, 2.0, 5.0, 7.0, 9.0, 9.0], (4, 4)).unwrap();
/// let trace = random_matrix.trace();
/// // trace: [ 6, 89, 45, 9 ]
///
/// let no_trace_matrix = Matrix::new(vec![6.0, 4.0, 87.0, 3.0, 6.0, 89.0, 6.0, 8.0, 4.0, 2.0, 45.0, 2.0, 5.0, 7.0, 9.0], (5, 3)).unwrap();
/// let invalid_trace = no_trace_matrix.trace();
///
/// assert_eq!(invalid_trace.is_ok(), false);
/// assert_eq!(trace.is_ok(), true);
/// let trace = trace.unwrap();
/// assert_eq!(trace[0], 6.0);
/// assert_eq!(trace[2], 45.0);
/// ```
/// ### Getting and Setting items
/// * `get_row` - Returns a [`Result`], [`Ok`] if index is within the bounds, contains the **nth row**, [`Err`] otherise
/// * `get_column` - Returns a [`Result`], [`Ok`] if index is within the bounds, contains the **nth column**, [`Err`] otherwise
/// * `get` - Returns a [`Result`], [`Ok`] if indexes is within the bounds, [`Err`] otherwise
/// * `set` - Returns a [`Result`], [`Ok`] if the value was updated, [`Err`] otherwise
/// ```
/// use math_matrix::Matrix;
/// let mut matrix = Matrix::new(vec![6.0, 4.0, 87.0, 3.0, 6.0, 89.0, 6.0, 8.0, 4.0, 2.0, 45.0, 2.0, 5.0, 7.0, 9.0], (5, 3)).unwrap();
/// // 6   4   87
/// // 3   6   89
/// // 6   8   4
/// // 2   45  2
/// // 5   7   9
///
/// assert_eq!(matrix.get_row(1).unwrap(), vec![6.0, 4.0, 87.0]);
/// assert_eq!(matrix.get_column(1).unwrap(), vec![6.0, 3.0, 6.0, 2.0, 5.0]);
/// assert_eq!(matrix.get_column(2).unwrap(), vec![4.0, 6.0, 8.0, 45.0, 7.0]);
/// assert_eq!(matrix.get(3, 2).unwrap(), 8.0);
/// assert_eq!(matrix.get(5, 1).unwrap(), 5.0);
///
/// matrix.set(5, 1, 99.0);
///
/// assert_eq!(matrix.get(5, 1).unwrap(), 99.0);
/// ```
#[derive(Clone, PartialEq)]
pub struct Matrix {
    items: Vec<f64>,
    pub order: (u32, u32),
}

impl Matrix {
    /// # Matrix Constructor
    /// ```
    /// use math_matrix::Matrix;
    /// let matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], (3, 2));
    /// let invalid_matrix = Matrix::new(vec![1.0, 2.0, 3.0], (3, 2));
    ///
    /// assert_eq!(matrix.is_ok(), true);
    /// assert_eq!(matrix.unwrap().order, (3, 2));
    /// assert_eq!(invalid_matrix.is_ok(), false);
    /// ```
    pub fn new(items: Vec<f64>, order: (u32, u32)) -> Result<Matrix, Errors> {
        if items.len() as u32 != order.0 * order.1 {
            return Err(Errors::InappropriateNumberOfItems);
        }
        Ok(Matrix { items, order })
    }
    /// # Matrix generated with function
    /// Using functions to describe the matrix
    /// ```
    /// use math_matrix::Matrix;
    /// // i^2 + 3j - 7
    /// let function_generated = Matrix::generate(|i, j| (i * i + 3 * j) as f64 - 7.0, (5, 5));
    /// // -3  0   3   6   9
    /// // 0   3   6   9   12
    /// // 5   8   11  14  17
    /// // 12  15  18  21  24
    /// // 21  24  27  30  33
    ///
    /// assert_eq!(function_generated[(1, 1)], -3.0);
    /// assert_eq!(function_generated[(2, 1)], 0.0);
    /// assert_eq!(function_generated[(3, 3)], 11.0);
    /// assert_eq!(function_generated[(4, 3)], 18.0);
    /// ```
    pub fn generate<F>(f: F, order: (u32, u32)) -> Matrix
    where
        F: Fn(u32, u32) -> f64,
    {
        let mut items: Vec<f64> = vec![];
        for i in 1..=order.0 {
            for j in 1..=order.1 {
                items.push(f(i, j))
            }
        }

        Matrix { items, order }
    }
    /// # Row matrix
    /// eg. `1  2  3  4  5  6  7`
    /// ```
    /// use math_matrix::Matrix;
    /// let row_matrix = Matrix::row_matrix(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
    /// assert_eq!(row_matrix[(1, 5)], 5.0);
    /// assert_eq!(row_matrix.get(2, 5).is_err(), true);
    /// ```
    pub fn row_matrix(items: Vec<f64>) -> Matrix {
        let binding = items.len() as u32;
        Matrix {
            items,
            order: (1, binding),
        }
    }
    /// # Column matrix
    /// eg. <br>`1`<br>`2`<br>`3`<br>`4`<br>`5`<br>`6`
    /// ```
    /// use math_matrix::Matrix;
    /// let column_matrix = Matrix::column_matrix(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    /// assert_eq!(column_matrix[(3, 1)], 3.0);
    /// assert_eq!(column_matrix.get(3, 5).is_err(), true);
    /// ```
    pub fn column_matrix(items: Vec<f64>) -> Matrix {
        let binding = items.len() as u32;
        Matrix {
            items,
            order: (binding, 1),
        }
    }
    /// # Null Matrix
    /// eg.<br>`0  0  0  0  0`<br>`0  0  0  0  0`<br>`0  0  0  0  0`<br>`0  0  0  0  0`<br>`0  0  0  0  0`
    /// ```
    /// use math_matrix::Matrix;
    /// let null_matrix = Matrix::null_matrix((10, 10));
    /// assert_eq!(null_matrix[(5, 5)], 0.0);
    /// assert_eq!(null_matrix[(10, 10)], 0.0);
    /// assert_eq!(null_matrix[(9, 6)], 0.0);
    /// ```
    pub fn null_matrix(order: (u32, u32)) -> Matrix {
        Matrix::generate(|_, _| 0.0, order)
    }
    /// # Square Matrix
    /// Returns [`Result`], [`Ok`] if the items can be arranged like a square, [`Err`] otherwise<br>
    /// eg.<br>`1  2  3  4  6`<br>`3  3  8  5  1`<br>`7  4  7  1  2`<br>`2  3  4  5  4`<br>`5  7  2  9  9`
    /// ```
    /// use math_matrix::Matrix;
    /// let square_matrix = Matrix::square_matrix(vec![1.0, 2.0, 3.0, 4.0]);
    /// let invalid_square_matrix = Matrix::square_matrix(vec![1.0, 2.0, 3.0]);
    ///
    /// assert_eq!(square_matrix.unwrap().order, (2, 2));
    /// assert_eq!(invalid_square_matrix.is_ok(), false);
    ///
    /// ```
    pub fn square_matrix(items: Vec<f64>) -> Result<Matrix, Errors> {
        let size = (items.len() as f32).sqrt();
        if size.fract() != 0.0 {
            return Err(Errors::InappropriateNumberOfItems);
        }
        let size = size as u32;
        Ok(Matrix {
            items,
            order: (size, size),
        })
    }
    /// # Diagonal Matrix
    /// eg.<br>`1  0  0  0  0`<br>`0  7  0  0  0`<br>`0  0  3  0  0`<br>`0  0  0  6  0`<br>`0  0  0  0  9`
    /// ```
    /// use math_matrix::Matrix;
    /// let diagonal_matrix = Matrix::diagonal_matrix(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    ///
    /// assert_eq!(diagonal_matrix.order, (8, 8));
    /// assert_eq!(diagonal_matrix[(4, 5)], 0.0);
    /// assert_eq!(diagonal_matrix[(5, 5)], 5.0);
    /// assert_eq!(diagonal_matrix[(7, 8)], 0.0);
    ///
    /// ```
    pub fn diagonal_matrix(items: Vec<f64>) -> Matrix {
        Matrix::generate(
            |i, j| {
                if i != j {
                    return 0.0;
                }
                items[(i - 1) as usize]
            },
            (items.len() as u32, items.len() as u32),
        )
    }
    /// # Scalar Matrix
    /// eg.<br>`6  0  0  0  0`<br>`0  6  0  0  0`<br>`0  0  6  0  0`<br>`0  0  0  6  0`<br>`0  0  0  0  6`
    /// ```
    /// use math_matrix::Matrix;
    /// let scalar_matrix = Matrix::scalar_matrix(5.0, 6);
    ///
    /// assert_eq!(scalar_matrix.order, (6, 6));
    /// assert_eq!(scalar_matrix[(3, 4)], 0.0);
    /// assert_eq!(scalar_matrix[(5, 5)], 5.0);
    /// assert_eq!(scalar_matrix[(3, 3)], 5.0);
    /// ```
    pub fn scalar_matrix(item: f64, size: u32) -> Matrix {
        Matrix::generate(
            |i, j| {
                if i != j {
                    return 0.0;
                }
                item
            },
            (size, size),
        )
    }
    /// # Identity Matrix
    /// eg.<br>`1  0  0  0  0`<br>`0  1  0  0  0`<br>`0  0  1  0  0`<br>`0  0  0  1  0`<br>`0  0  0  0  1`
    /// ```
    /// use math_matrix::Matrix;
    /// let identity_matrix = Matrix::identity_matrix(5);
    ///
    /// assert_eq!(identity_matrix.order, (5, 5));
    /// assert_eq!(identity_matrix[(3, 4)], 0.0);
    /// assert_eq!(identity_matrix[(5, 5)], 1.0);
    /// assert_eq!(identity_matrix[(3, 3)], 1.0);
    /// ```
    pub fn identity_matrix(size: u32) -> Matrix {
        Matrix::scalar_matrix(1.0, size)
    }
    /// # Trace
    /// Traces are the diagonal items of a square matrix<br>
    /// Returns [`Result`], [`Ok`] if the matrix is square, [`Err`] otherwise<br>
    /// eg.<br>` `**1**`  2  3  4  6`<br>`3  `**3**`  8  5  1`<br>`7  4  `**7**`  1  2`<br>`2  3  4  `**5**`  4`<br>`5  7  2  9 `**9**` `
    /// ```
    /// use math_matrix::Matrix;
    /// let random_matrix = Matrix::new(vec![6.0, 4.0, 87.0, 3.0, 6.0, 89.0, 6.0, 8.0, 4.0, 2.0, 45.0, 2.0, 5.0, 7.0, 9.0, 9.0], (4, 4)).unwrap();
    /// let trace = random_matrix.trace();
    /// // trace: [ 6, 89, 45, 9 ]
    ///
    /// let no_trace_matrix = Matrix::new(vec![6.0, 4.0, 87.0, 3.0, 6.0, 89.0, 6.0, 8.0, 4.0, 2.0, 45.0, 2.0, 5.0, 7.0, 9.0], (5, 3)).unwrap();
    /// let invalid_trace = no_trace_matrix.trace();
    ///
    /// assert_eq!(invalid_trace.is_ok(), false);
    /// assert_eq!(trace.is_ok(), true);
    /// let trace = trace.unwrap();
    /// assert_eq!(trace[0], 6.0);
    /// assert_eq!(trace[2], 45.0);
    /// ```
    pub fn trace(&self) -> Result<Vec<f64>, Errors> {
        if self.order.0 != self.order.1 {
            return Err(Errors::TraceExistsOnlyForSquareMatrices);
        }
        Ok(self
            .items
            .clone()
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| {
                let row = i as u32 / self.order.0;
                let column = i as u32 % self.order.1;

                row == column
            })
            .map(|(_, e)| e)
            .collect())
    }
    /// # Transpose
    /// Flip the rows and columns
    /// ```
    /// use math_matrix::Matrix;
    /// let matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], (2, 3)).unwrap();
    /// // 1  2  3
    /// // 4  5  6
    /// let transpose = matrix.transpose();
    /// // 1  4
    /// // 2  5
    /// // 3  6
    /// assert!(transpose == Matrix::new(vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0], (3, 2)).unwrap());
    /// ```
    pub fn transpose(&self) -> Matrix {
        Matrix::generate(
            |i, j| self.get(j, i).expect("Impossible"),
            (self.order.1, self.order.0),
        )
    }
    /// # Determinant
    /// Convert the matrix into a determinant
    /// ```
    /// use math_matrix::Matrix;
    /// let matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0], (3, 3)).unwrap();
    /// let invalid_det_matrix = Matrix::new(vec![1.0, 2.0, 3.0], (3, 1)).unwrap();
    ///
    /// // notice the matrix can be arranged like a square
    /// let invalid2_det_matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], (1, 4)).unwrap();
    ///
    /// assert_eq!(invalid_det_matrix.to_determinant().is_ok(), false);
    /// assert_eq!(invalid2_det_matrix.to_determinant().is_ok(), false);
    ///
    /// let det = matrix.to_determinant().unwrap();
    /// assert_eq!(det.value(), 0.0);
    /// ```
    pub fn to_determinant(&self) -> Result<Determinant, Errors> {
        if self.order.0 != self.order.1 {
            return Err(Errors::IncorrectOrdersForOperation);
        }
        Determinant::new(self.items.clone())
    }
    /// # Adjoint
    /// Get the adjoint of a matrix
    /// ```
    /// use math_matrix::Matrix;
    /// let matrix = Matrix::new(vec![1.0, 0.0, -1.0, 3.0, 4.0, 5.0, 0.0, -6.0, -7.0], (3, 3)).unwrap();
    /// assert!(matrix.adjoint().unwrap() == Matrix::new(vec![2.0, 6.0, 4.0, 21.0, -7.0, -8.0, -18.0, 6.0, 4.0], (3, 3)).unwrap());
    /// ```
    pub fn adjoint(&self) -> Result<Matrix, Errors> {
        let det = self.to_determinant()?;
        Ok(
            Matrix::generate(|i, j| det.cofactor(i, j).expect("Impossible"), self.order)
                .transpose(),
        )
    }
    /// # Inverse
    /// Get the inverse of a matrix
    /// ```
    /// use math_matrix::Matrix;
    /// let matrix = Matrix::new(vec![1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 2.0, 1.0, 3.0], (3, 3)).unwrap();
    /// let inverse = Matrix::new(vec![-5.0, 3.0, 4.0, 7.0, 3.0, -8.0, 1.0, -3.0, 4.0], (3, 3)).unwrap() / 12.0;
    /// assert!(matrix.inverse().unwrap() == inverse);
    /// ```
    pub fn inverse(&self) -> Result<Matrix, Errors> {
        Ok(self.adjoint()? / self.to_determinant()?.value())
    }
    /// # Round
    /// Round of all the elements of the matrix
    /// ```
    /// use math_matrix::Matrix;
    /// let matrix = Matrix::new(vec![0.9999, 0.0000023, 0.99999], (1, 3)).unwrap();
    /// assert!(matrix.round() == Matrix::new(vec![1.0, 0.0, 1.0], (1, 3)).unwrap());
    /// ```
    pub fn round(&self) -> Matrix {
        Matrix::generate(
            |i, j| self.get(i, j).expect("Impossible").round(),
            self.order,
        )
    }
    /// # Round
    /// Round of all the elements of the matrix and update the matrix
    /// ```
    /// use math_matrix::Matrix;
    /// let mut matrix = Matrix::new(vec![0.9999, 0.0000023, 0.99999], (1, 3)).unwrap();
    /// matrix.round_mut();
    /// assert!(matrix == Matrix::new(vec![1.0, 0.0, 1.0], (1, 3)).unwrap());
    /// ```
    pub fn round_mut(&mut self) {
        *self = Matrix::generate(
            |i, j| self.get(i, j).expect("Impossible").round(),
            self.order,
        );
    }
    /// # Is the matrix horizontal?
    /// ```
    /// use math_matrix::Matrix;
    /// let matrix = Matrix::new(vec![6.0, 4.0, 87.0, 3.0, 6.0, 89.0, 6.0, 8.0, 4.0, 2.0, 45.0, 2.0, 5.0, 7.0, 9.0], (5, 3)).unwrap();
    /// // 6   4   87
    /// // 3   6   89
    /// // 6   8   4
    /// // 2   45  2
    /// // 5   7   9
    /// let horizontal_matrix = Matrix::null_matrix((5, 10));
    /// assert!(horizontal_matrix.is_horizontal());
    /// ```
    pub fn is_horizontal(&self) -> bool {
        self.order.1 > self.order.0
    }
    /// # Is the matrix vertical?
    /// ```
    /// use math_matrix::Matrix;
    /// let matrix = Matrix::new(vec![6.0, 4.0, 87.0, 3.0, 6.0, 89.0, 6.0, 8.0, 4.0, 2.0, 45.0, 2.0, 5.0, 7.0, 9.0], (5, 3)).unwrap();
    /// // 6   4   87
    /// // 3   6   89
    /// // 6   8   4
    /// // 2   45  2
    /// // 5   7   9
    /// let vertical_matrix = Matrix::null_matrix((10, 5));
    /// assert!(vertical_matrix.is_vertical());
    /// ```
    pub fn is_vertical(&self) -> bool {
        self.order.0 > self.order.1
    }
    /// # Get an item from the matrix
    /// ```
    /// use math_matrix::Matrix;
    /// let matrix = Matrix::new(vec![6.0, 4.0, 87.0, 3.0, 6.0, 89.0, 6.0, 8.0, 4.0, 2.0, 45.0, 2.0, 5.0, 7.0, 9.0], (5, 3)).unwrap();
    /// // 6   4   87
    /// // 3   6   89
    /// // 6   8   4
    /// // 2   45  2
    /// // 5   7   9
    /// assert_eq!(matrix.get(3, 2).unwrap(), 8.0);
    /// assert_eq!(matrix.get(5, 1).unwrap(), 5.0);
    /// ```
    pub fn get(&self, i: u32, j: u32) -> Result<f64, Errors> {
        match self.items.get(((i - 1) * self.order.1 + (j - 1)) as usize) {
            Some(item) => Ok(*item),
            None => Err(Errors::IndexOutOfRange),
        }
    }
    /// # Get an entire row
    /// ```
    /// use math_matrix::Matrix;
    /// let matrix = Matrix::new(vec![6.0, 4.0, 87.0, 3.0, 6.0, 89.0, 6.0, 8.0, 4.0, 2.0, 45.0, 2.0, 5.0, 7.0, 9.0], (5, 3)).unwrap();
    /// // 6   4   87
    /// // 3   6   89
    /// // 6   8   4
    /// // 2   45  2
    /// // 5   7   9
    ///
    /// assert_eq!(matrix.get_row(1).unwrap(), vec![6.0, 4.0, 87.0]);
    /// ```
    pub fn get_row(&self, i: u32) -> Result<Vec<f64>, Errors> {
        if i == 0 || i > self.order.0 {
            return Err(Errors::IndexOutOfRange);
        }
        Ok(self
            .items
            .clone()
            .into_iter()
            .enumerate()
            .filter(|&(idx, _)| {
                let row = idx as u32 / self.order.1;
                row == (i - 1)
            })
            .map(|(_, e)| e)
            .collect())
    }
    /// # Get an entire column
    /// ```
    /// use math_matrix::Matrix;
    /// let matrix = Matrix::new(vec![6.0, 4.0, 87.0, 3.0, 6.0, 89.0, 6.0, 8.0, 4.0, 2.0, 45.0, 2.0, 5.0, 7.0, 9.0], (5, 3)).unwrap();
    /// // 6   4   87
    /// // 3   6   89
    /// // 6   8   4
    /// // 2   45  2
    /// // 5   7   9
    ///
    /// assert_eq!(matrix.get_column(1).unwrap(), vec![6.0, 3.0, 6.0, 2.0, 5.0]);
    /// assert_eq!(matrix.get_column(2).unwrap(), vec![4.0, 6.0, 8.0, 45.0, 7.0]);
    /// ```
    pub fn get_column(&self, j: u32) -> Result<Vec<f64>, Errors> {
        if j == 0 || j > self.order.1 {
            return Err(Errors::IndexOutOfRange);
        }
        Ok(self
            .items
            .clone()
            .into_iter()
            .enumerate()
            .filter(|&(idx, _)| {
                let column = idx as u32 % self.order.1;
                column == (j - 1)
            })
            .map(|(_, e)| e)
            .collect())
    }
    /// # Change an item in the matrix
    /// ```
    /// use math_matrix::Matrix;
    /// let mut matrix = Matrix::new(vec![6.0, 4.0, 87.0, 3.0, 6.0, 89.0, 6.0, 8.0, 4.0, 2.0, 45.0, 2.0, 5.0, 7.0, 9.0], (5, 3)).unwrap();
    /// // 6   4   87
    /// // 3   6   89
    /// // 6   8   4
    /// // 2   45  2
    /// // 5   7   9
    ///
    /// assert_eq!(matrix.get(5, 1).unwrap(), 5.0);
    /// matrix.set(5, 1, 99.0);
    /// assert_eq!(matrix.get(5, 1).unwrap(), 99.0);
    /// ```
    pub fn set(&mut self, i: u32, j: u32, new_value: f64) -> Result<(), Errors> {
        if i == 0 || i > self.order.0 || j == 0 || j > self.order.1 {
            return Err(Errors::IndexOutOfRange);
        }
        match self
            .items
            .get_mut(((i - 1) * self.order.1 + (j - 1)) as usize)
        {
            Some(item) => {
                *item = new_value;
                Ok(())
            }
            None => Err(Errors::IndexOutOfRange),
        }
    }
}
impl Add for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Self) -> Self::Output {
        if self.order != rhs.order {
            eprintln!("Error: {}", Errors::IncorrectOrdersForOperation);
            panic!();
        }
        Matrix::generate(
            |i, j| self.get(i, j).expect("Impossible") + rhs.get(i, j).expect("Impossible"),
            self.order,
        )
    }
}
impl AddAssign for Matrix {
    fn add_assign(&mut self, rhs: Self) {
        if self.order != rhs.order {
            eprintln!("Error: {}", Errors::IncorrectOrdersForOperation);
            panic!();
        }
        *self = Matrix::generate(
            |i, j| self.get(i, j).expect("Impossible") + rhs.get(i, j).expect("Impossible"),
            self.order,
        );
    }
}
impl Sub for Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.order != rhs.order {
            eprintln!("Error: {}", Errors::IncorrectOrdersForOperation);
            panic!();
        }
        Matrix::generate(
            |i, j| self.get(i, j).expect("Impossible") - rhs.get(i, j).expect("Impossible"),
            self.order,
        )
    }
}
impl SubAssign for Matrix {
    fn sub_assign(&mut self, rhs: Self) {
        if self.order != rhs.order {
            eprintln!("Error: {}", Errors::IncorrectOrdersForOperation);
            panic!();
        }
        *self = Matrix::generate(
            |i, j| self.get(i, j).expect("Impossible") - rhs.get(i, j).expect("Impossible"),
            self.order,
        );
    }
}
#[allow(clippy::suspicious_arithmetic_impl)]
impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.order.1 != rhs.order.0 {
            eprintln!("Error: {}", Errors::IncorrectOrdersForOperation);
            panic!();
        }
        Matrix::generate(
            |i, j| {
                let mut sum = 0.0;
                let a = self.get_row(i).expect("Impossible");
                let b = rhs.get_column(j).expect("Impossible");
                for r in 0..self.order.1 {
                    sum += a[r as usize] * b[r as usize]
                }
                sum
            },
            (self.order.0, rhs.order.1),
        )
    }
}
impl Mul<f64> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f64) -> Self::Output {
        Matrix::generate(|i, j| self.get(i, j).expect("Impossible") * rhs, self.order)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl MulAssign for Matrix {
    fn mul_assign(&mut self, rhs: Self) {
        if self.order.1 != rhs.order.0 {
            eprintln!("Error: {}", Errors::IncorrectOrdersForOperation);
            panic!();
        }
        *self = Matrix::generate(
            |i, j| {
                let mut sum = 0.0;
                let a = self.get_row(i).expect("Impossible");
                let b = rhs.get_column(j).expect("Impossible");
                for r in 0..self.order.1 {
                    sum += a[r as usize] * b[r as usize]
                }
                sum
            },
            (self.order.0, rhs.order.1),
        );
    }
}
impl MulAssign<f64> for Matrix {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Matrix::generate(|i, j| self.get(i, j).expect("Impossible") * rhs, self.order);
    }
}
impl Div<f64> for Matrix {
    type Output = Matrix;
    fn div(self, rhs: f64) -> Self::Output {
        Matrix::generate(|i, j| self.get(i, j).expect("Impossible") / rhs, self.order)
    }
}
impl DivAssign<f64> for Matrix {
    fn div_assign(&mut self, rhs: f64) {
        *self = Matrix::generate(|i, j| self.get(i, j).expect("Impossible") / rhs, self.order)
    }
}
impl Index<(u32, u32)> for Matrix {
    type Output = f64;
    fn index(&self, (i, j): (u32, u32)) -> &Self::Output {
        &self.items[((i - 1) * self.order.1 + (j - 1)) as usize]
    }
}
impl IndexMut<(u32, u32)> for Matrix {
    fn index_mut(&mut self, (i, j): (u32, u32)) -> &mut Self::Output {
        &mut self.items[((i - 1) * self.order.1 + (j - 1)) as usize]
    }
}
impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut print = String::new();
        let mut largest_item_size = 0;
        for item in self.items.iter() {
            let size = item.to_string().len();
            if size > largest_item_size {
                largest_item_size = size;
            }
        }

        for (i, item) in self.items.iter().enumerate() {
            print += &format!(
                "{}{}  ",
                item,
                " ".repeat((largest_item_size - item.to_string().len()) as usize)
            );
            if (i as u32 + 1) % self.order.1 == 0 {
                print += "\n";
            }
        }
        f.write_str(&print)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn addition() {
        use crate::Matrix;
        let a = Matrix::new(vec![45.0, 2.0, 65.0, 899.0, 6.0, 61.0], (3, 2)).expect("Impossible");
        let b = Matrix::new(vec![4.0, 87.0, 2.0, 99.0, 12.0, 44.0], (3, 2)).expect("Impossible");

        let sum = a + b;
        assert_eq!(sum.order, (3, 2));
        assert_eq!(sum.items, vec![49.0, 89.0, 67.0, 998.0, 18.0, 105.0]);
    }

    #[test]
    fn subtraction() {
        use crate::Matrix;
        let a = Matrix::new(vec![45.0, 2.0, 65.0, 899.0, 6.0, 61.0], (3, 2)).expect("Impossible");
        let b = Matrix::new(vec![4.0, 87.0, 2.0, 99.0, 12.0, 44.0], (3, 2)).expect("Impossible");

        let diff = a - b;
        assert_eq!(diff.order, (3, 2));
        assert_eq!(diff.items, vec![41.0, -85.0, 63.0, 800.0, -6.0, 17.0]);
    }

    #[test]
    fn multiplication() {
        use crate::Matrix;
        let a = Matrix::new(vec![4.0, 87.0, 2.0, 99.0, 12.0, 44.0], (3, 2)).expect("Impossible");

        let scalar_mul = a.clone() * 5.0;
        assert_eq!(scalar_mul.order, (3, 2));
        assert_eq!(
            scalar_mul.items,
            vec![20.0, 435.0, 10.0, 495.0, 60.0, 220.0]
        );

        let b = Matrix::new(vec![45.0, 2.0, 65.0, 899.0, 6.0, 61.0], (2, 3)).expect("Impossible");

        let matrix_mul = a * b;
        assert_eq!(matrix_mul.order, (3, 3));
        assert_eq!(
            matrix_mul.items,
            vec![78393.0, 530.0, 5567.0, 89091.0, 598.0, 6169.0, 40096.0, 288.0, 3464.0]
        );

        let matrix =
            Matrix::new(vec![1.0, 6.0, 4.0, 2.0, 5.0, 7.0, 4.0, 2.0, 9.0], (3, 3)).unwrap();
        let inverse = matrix.inverse().unwrap();

        // Rounding because 0.999999991 is just 1
        assert!((matrix * inverse).round() == Matrix::identity_matrix(3));
    }
}
