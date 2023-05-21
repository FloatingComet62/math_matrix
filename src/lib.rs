use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

/// # Errors
/// * `InappropriateNumberOfItems` - Inappropriate number of items
/// * `TraceExistsOnlyForSquareMatrices` - Traces exists only for square matrices
/// * `IncorrectOrdersForOperation` - Incorret orders of matrices for algebric operations
/// * `IndexOutOfRange` - Index out of range
pub enum Errors {
    InappropriateNumberOfItems,
    TraceExistsOnlyForSquareMatrices,
    IncorrectOrdersForOperation,
    IndexOutOfRange,
}
impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Errors::InappropriateNumberOfItems => "Inappropriate number of items",
            Errors::TraceExistsOnlyForSquareMatrices => "Traces exists only for square matrices",
            Errors::IncorrectOrdersForOperation => {
                "Incorrect orders of matrices for algebric operations"
            }
            Errors::IndexOutOfRange => "Index out of range",
        })
    }
}
impl Debug for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

/// # Matrix
/// * `items` - Items of the matrix in row by row order
/// * `order` - Order of the matrix
///
/// ## Examples
/// ```
/// use math_matrix::Matrix;
/// let matrix = Matrix::new(vec![1, 2, 3, 4, 5, 6], (3, 2));
/// let invalid_matrix = Matrix::new(vec![1, 2, 3], (3, 2));
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
/// let function_generated = Matrix::generate(|i, j| (i * i + 3 * j) as i32 - 7, (5, 5));
/// // -3  0   3   6   9
/// // 0   3   6   9   12
/// // 5   8   11  14  17
/// // 12  15  18  21  24
/// // 21  24  27  30  33
///
/// assert_eq!(function_generated[(1, 1)], -3);
/// assert_eq!(function_generated[(2, 1)], 0);
/// assert_eq!(function_generated[(3, 3)], 11);
/// assert_eq!(function_generated[(4, 3)], 18);
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
/// let row_matrix = Matrix::row_matrix(vec![1, 2, 3, 4, 5, 6, 7]);
/// assert_eq!(row_matrix[(1, 5)], 5);
/// assert_eq!(row_matrix.get(2, 5).is_err(), true);
///
/// // Column matrix
/// let column_matrix = Matrix::column_matrix(vec![1, 2, 3, 4, 5, 6]);
/// assert_eq!(column_matrix[(3, 1)], 3);
/// assert_eq!(column_matrix.get(3, 5).is_err(), true);
///
/// // Null matrix
/// let null_matrix = Matrix::null_matrix((10, 10));
/// assert_eq!(null_matrix[(5, 5)], 0);
/// assert_eq!(null_matrix[(10, 10)], 0);
/// assert_eq!(null_matrix[(9, 6)], 0);
///
/// // Square matrix
/// let square_matrix = Matrix::square_matrix(vec![1, 2, 3, 4]);
/// let invalid_square_matrix = Matrix::square_matrix(vec![1, 2, 3]);
///
/// assert_eq!(square_matrix.unwrap().order, (2, 2));
/// assert_eq!(invalid_square_matrix.is_ok(), false);
///
/// // Diagonal matrix
/// let diagonal_matrix = Matrix::diagonal_matrix(vec![1, 2, 3, 4, 5, 6, 7, 8]);
///
/// assert_eq!(diagonal_matrix.order, (8, 8));
/// assert_eq!(diagonal_matrix[(4, 5)], 0);
/// assert_eq!(diagonal_matrix[(5, 5)], 5);
/// assert_eq!(diagonal_matrix[(7, 8)], 0);
///
/// /// Scalar matrix
/// let scalar_matrix = Matrix::scalar_matrix(5, 6);
///
/// assert_eq!(scalar_matrix.order, (6, 6));
/// assert_eq!(scalar_matrix[(3, 4)], 0);
/// assert_eq!(scalar_matrix[(5, 5)], 5);
/// assert_eq!(scalar_matrix[(3, 3)], 5);
///
/// /// Identity matrix
/// let identity_matrix = Matrix::identity_matrix(5);
///
/// assert_eq!(identity_matrix.order, (5, 5));
/// assert_eq!(identity_matrix[(3, 4)], 0);
/// assert_eq!(identity_matrix[(5, 5)], 1);
/// assert_eq!(identity_matrix[(3, 3)], 1);
/// ```
/// ### Traces
/// Traces are the diagonal items of a square matrix<br>
/// Returns [`Result`], [`Ok`] if the matrix is square, [`Err`] otherwise
/// ```
/// use math_matrix::Matrix;
/// // Traces
/// let random_matrix = Matrix::new(vec![6, 4, 87, 3, 6, 89, 6, 8, 4, 2, 45, 2, 5, 7, 9, 9], (4, 4)).unwrap();
/// let trace = random_matrix.trace();
/// // trace: [ 6, 89, 45, 9 ]
///
/// let no_trace_matrix = Matrix::new(vec![6, 4, 87, 3, 6, 89, 6, 8, 4, 2, 45, 2, 5, 7, 9], (5, 3)).unwrap();
/// let invalid_trace = no_trace_matrix.trace();
///
/// assert_eq!(invalid_trace.is_ok(), false);
/// assert_eq!(trace.is_ok(), true);
/// let trace = trace.unwrap();
/// assert_eq!(trace[0], 6);
/// assert_eq!(trace[2], 45);
/// ```
/// ### Getting and Setting items
/// * `get_row` - Returns a [`Result`], [`Ok`] if index is within the bounds, contains the **nth row**, [`Err`] otherise
/// * `get_column` - Returns a [`Result`], [`Ok`] if index is within the bounds, contains the **nth column**, [`Err`] otherwise
/// * `get` - Returns a [`Result`], [`Ok`] if indexes is within the bounds, [`Err`] otherwise
/// * `set` - Returns a [`Result`], [`Ok`] if the value was updated, [`Err`] otherwise
/// ```
/// use math_matrix::Matrix;
/// let mut matrix = Matrix::new(vec![6, 4, 87, 3, 6, 89, 6, 8, 4, 2, 45, 2, 5, 7, 9], (5, 3)).unwrap();
/// // 6   4   87
/// // 3   6   89
/// // 6   8   4
/// // 2   45  2
/// // 5   7   9
///
/// assert_eq!(matrix.get_row(1).unwrap(), vec![6, 4, 87]);
/// assert_eq!(matrix.get_column(1).unwrap(), vec![6, 3, 6, 2, 5]);
/// assert_eq!(matrix.get_column(2).unwrap(), vec![4, 6, 8, 45, 7]);
/// assert_eq!(matrix.get(3, 2).unwrap(), 8);
/// assert_eq!(matrix.get(5, 1).unwrap(), 5);
///
/// matrix.set(5, 1, 99);
///
/// assert_eq!(matrix.get(5, 1).unwrap(), 99);
/// ```
#[derive(Clone)]
pub struct Matrix {
    items: Vec<i32>,
    pub order: (u32, u32),
}

impl Matrix {
    /// # Matrix Constructor
    /// ```
    /// use math_matrix::Matrix;
    /// let matrix = Matrix::new(vec![1, 2, 3, 4, 5, 6], (3, 2));
    /// let invalid_matrix = Matrix::new(vec![1, 2, 3], (3, 2));
    ///
    /// assert_eq!(matrix.is_ok(), true);
    /// assert_eq!(matrix.unwrap().order, (3, 2));
    /// assert_eq!(invalid_matrix.is_ok(), false);
    /// ```
    pub fn new(items: Vec<i32>, order: (u32, u32)) -> Result<Matrix, Errors> {
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
    /// let function_generated = Matrix::generate(|i, j| (i * i + 3 * j) as i32 - 7, (5, 5));
    /// // -3  0   3   6   9
    /// // 0   3   6   9   12
    /// // 5   8   11  14  17
    /// // 12  15  18  21  24
    /// // 21  24  27  30  33
    ///
    /// assert_eq!(function_generated[(1, 1)], -3);
    /// assert_eq!(function_generated[(2, 1)], 0);
    /// assert_eq!(function_generated[(3, 3)], 11);
    /// assert_eq!(function_generated[(4, 3)], 18);
    /// ```
    pub fn generate<F>(f: F, order: (u32, u32)) -> Matrix
    where
        F: Fn(u32, u32) -> i32,
    {
        let mut items: Vec<i32> = vec![];
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
    /// let row_matrix = Matrix::row_matrix(vec![1, 2, 3, 4, 5, 6, 7]);
    /// assert_eq!(row_matrix[(1, 5)], 5);
    /// assert_eq!(row_matrix.get(2, 5).is_err(), true);
    /// ```
    pub fn row_matrix(items: Vec<i32>) -> Matrix {
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
    /// let column_matrix = Matrix::column_matrix(vec![1, 2, 3, 4, 5, 6]);
    /// assert_eq!(column_matrix[(3, 1)], 3);
    /// assert_eq!(column_matrix.get(3, 5).is_err(), true);
    /// ```
    pub fn column_matrix(items: Vec<i32>) -> Matrix {
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
    /// assert_eq!(null_matrix[(5, 5)], 0);
    /// assert_eq!(null_matrix[(10, 10)], 0);
    /// assert_eq!(null_matrix[(9, 6)], 0);
    /// ```
    pub fn null_matrix(order: (u32, u32)) -> Matrix {
        Matrix::generate(|_, _| 0, order)
    }
    /// # Square Matrix
    /// Returns [`Result`], [`Ok`] if the items can be arranged like a square, [`Err`] otherwise<br>
    /// eg.<br>`1  2  3  4  6`<br>`3  3  8  5  1`<br>`7  4  7  1  2`<br>`2  3  4  5  4`<br>`5  7  2  9  9`
    /// ```
    /// use math_matrix::Matrix;
    /// let square_matrix = Matrix::square_matrix(vec![1, 2, 3, 4]);
    /// let invalid_square_matrix = Matrix::square_matrix(vec![1, 2, 3]);
    ///
    /// assert_eq!(square_matrix.unwrap().order, (2, 2));
    /// assert_eq!(invalid_square_matrix.is_ok(), false);
    ///
    /// ```
    pub fn square_matrix(items: Vec<i32>) -> Result<Matrix, Errors> {
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
    /// let diagonal_matrix = Matrix::diagonal_matrix(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    ///
    /// assert_eq!(diagonal_matrix.order, (8, 8));
    /// assert_eq!(diagonal_matrix[(4, 5)], 0);
    /// assert_eq!(diagonal_matrix[(5, 5)], 5);
    /// assert_eq!(diagonal_matrix[(7, 8)], 0);
    ///
    /// ```
    pub fn diagonal_matrix(items: Vec<i32>) -> Matrix {
        Matrix::generate(
            |i, j| {
                if i != j {
                    return 0;
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
    /// let scalar_matrix = Matrix::scalar_matrix(5, 6);
    ///
    /// assert_eq!(scalar_matrix.order, (6, 6));
    /// assert_eq!(scalar_matrix[(3, 4)], 0);
    /// assert_eq!(scalar_matrix[(5, 5)], 5);
    /// assert_eq!(scalar_matrix[(3, 3)], 5);
    /// ```
    pub fn scalar_matrix(item: i32, size: u32) -> Matrix {
        Matrix::generate(
            |i, j| {
                if i != j {
                    return 0;
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
    /// assert_eq!(identity_matrix[(3, 4)], 0);
    /// assert_eq!(identity_matrix[(5, 5)], 1);
    /// assert_eq!(identity_matrix[(3, 3)], 1);
    /// ```
    pub fn identity_matrix(size: u32) -> Matrix {
        Matrix::scalar_matrix(1, size)
    }
    /// # Trace
    /// Traces are the diagonal items of a square matrix<br>
    /// Returns [`Result`], [`Ok`] if the matrix is square, [`Err`] otherwise<br>
    /// eg.<br>` `**1**`  2  3  4  6`<br>`3  `**3**`  8  5  1`<br>`7  4  `**7**`  1  2`<br>`2  3  4  `**5**`  4`<br>`5  7  2  9 `**9**` `
    /// ```
    /// use math_matrix::Matrix;
    /// let random_matrix = Matrix::new(vec![6, 4, 87, 3, 6, 89, 6, 8, 4, 2, 45, 2, 5, 7, 9, 9], (4, 4)).unwrap();
    /// let trace = random_matrix.trace();
    /// // trace: [ 6, 89, 45, 9 ]
    ///
    /// let no_trace_matrix = Matrix::new(vec![6, 4, 87, 3, 6, 89, 6, 8, 4, 2, 45, 2, 5, 7, 9], (5, 3)).unwrap();
    /// let invalid_trace = no_trace_matrix.trace();
    ///
    /// assert_eq!(invalid_trace.is_ok(), false);
    /// assert_eq!(trace.is_ok(), true);
    /// let trace = trace.unwrap();
    /// assert_eq!(trace[0], 6);
    /// assert_eq!(trace[2], 45);
    /// ```
    pub fn trace(&self) -> Result<Vec<i32>, Errors> {
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
    /// # Is the matrix horizontal?
    /// ```
    /// use math_matrix::Matrix;
    /// let matrix = Matrix::new(vec![6, 4, 87, 3, 6, 89, 6, 8, 4, 2, 45, 2, 5, 7, 9], (5, 3)).unwrap();
    /// // 6   4   87
    /// // 3   6   89
    /// // 6   8   4
    /// // 2   45  2
    /// // 5   7   9
    /// let horizontal_matrix = Matrix::null_matrix((5, 10));
    /// assert_eq!(horizontal_matrix.is_horizontal(), true);
    /// ```
    pub fn is_horizontal(&self) -> bool {
        self.order.1 > self.order.0
    }
    /// # Is the matrix vertical?
    /// ```
    /// use math_matrix::Matrix;
    /// let matrix = Matrix::new(vec![6, 4, 87, 3, 6, 89, 6, 8, 4, 2, 45, 2, 5, 7, 9], (5, 3)).unwrap();
    /// // 6   4   87
    /// // 3   6   89
    /// // 6   8   4
    /// // 2   45  2
    /// // 5   7   9
    /// let vertical_matrix = Matrix::null_matrix((10, 5));
    /// assert_eq!(vertical_matrix.is_vertical(), true);
    /// ```
    pub fn is_vertical(&self) -> bool {
        self.order.0 > self.order.1
    }
    /// # Get an item from the matrix
    /// ```
    /// use math_matrix::Matrix;
    /// let matrix = Matrix::new(vec![6, 4, 87, 3, 6, 89, 6, 8, 4, 2, 45, 2, 5, 7, 9], (5, 3)).unwrap();
    /// // 6   4   87
    /// // 3   6   89
    /// // 6   8   4
    /// // 2   45  2
    /// // 5   7   9
    /// assert_eq!(matrix.get(3, 2).unwrap(), 8);
    /// assert_eq!(matrix.get(5, 1).unwrap(), 5);
    /// ```
    pub fn get(&self, i: u32, j: u32) -> Result<i32, Errors> {
        match self.items.get(((i - 1) * self.order.1 + (j - 1)) as usize) {
            Some(item) => return Ok(item.clone()),
            None => return Err(Errors::IndexOutOfRange),
        }
    }
    /// # Get an entire row
    /// ```
    /// use math_matrix::Matrix;
    /// let matrix = Matrix::new(vec![6, 4, 87, 3, 6, 89, 6, 8, 4, 2, 45, 2, 5, 7, 9], (5, 3)).unwrap();
    /// // 6   4   87
    /// // 3   6   89
    /// // 6   8   4
    /// // 2   45  2
    /// // 5   7   9
    ///
    /// assert_eq!(matrix.get_row(1).unwrap(), vec![6, 4, 87]);
    /// ```
    pub fn get_row(&self, i: u32) -> Result<Vec<i32>, Errors> {
        if i > self.order.0 {
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
    /// let matrix = Matrix::new(vec![6, 4, 87, 3, 6, 89, 6, 8, 4, 2, 45, 2, 5, 7, 9], (5, 3)).unwrap();
    /// // 6   4   87
    /// // 3   6   89
    /// // 6   8   4
    /// // 2   45  2
    /// // 5   7   9
    ///
    /// assert_eq!(matrix.get_column(1).unwrap(), vec![6, 3, 6, 2, 5]);
    /// assert_eq!(matrix.get_column(2).unwrap(), vec![4, 6, 8, 45, 7]);
    /// ```
    pub fn get_column(&self, j: u32) -> Result<Vec<i32>, Errors> {
        if j > self.order.1 {
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
    /// let mut matrix = Matrix::new(vec![6, 4, 87, 3, 6, 89, 6, 8, 4, 2, 45, 2, 5, 7, 9], (5, 3)).unwrap();
    /// // 6   4   87
    /// // 3   6   89
    /// // 6   8   4
    /// // 2   45  2
    /// // 5   7   9
    ///
    /// assert_eq!(matrix.get(5, 1).unwrap(), 5);
    /// matrix.set(5, 1, 99);
    /// assert_eq!(matrix.get(5, 1).unwrap(), 99);
    /// ```
    pub fn set(&mut self, i: u32, j: u32, new_value: i32) -> Result<(), Errors> {
        match self
            .items
            .get_mut(((i - 1) * self.order.1 + (j - 1)) as usize)
        {
            Some(item) => {
                *item = new_value;
                return Ok(());
            }
            None => return Err(Errors::IndexOutOfRange),
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
        return Matrix::generate(
            |i, j| self.get(i, j).unwrap() + rhs.get(i, j).unwrap(),
            self.order,
        );
    }
}
impl AddAssign for Matrix {
    fn add_assign(&mut self, rhs: Self) {
        if self.order != rhs.order {
            eprintln!("Error: {}", Errors::IncorrectOrdersForOperation);
            panic!();
        }
        *self = Matrix::generate(
            |i, j| self.get(i, j).unwrap() + rhs.get(i, j).unwrap(),
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
        return Matrix::generate(
            |i, j| self.get(i, j).unwrap() - rhs.get(i, j).unwrap(),
            self.order,
        );
    }
}
impl SubAssign for Matrix {
    fn sub_assign(&mut self, rhs: Self) {
        if self.order != rhs.order {
            eprintln!("Error: {}", Errors::IncorrectOrdersForOperation);
            panic!();
        }
        *self = Matrix::generate(
            |i, j| self.get(i, j).unwrap() - rhs.get(i, j).unwrap(),
            self.order,
        );
    }
}
impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.order.1 != rhs.order.0 {
            eprintln!("Error: {}", Errors::IncorrectOrdersForOperation);
            panic!();
        }
        return Matrix::generate(
            |i, j| {
                let mut sum = 0;
                let a = self.get_row(i).unwrap();
                let b = rhs.get_column(j).unwrap();
                for r in 0..self.order.1 {
                    sum += a[r as usize] * b[r as usize]
                }
                sum
            },
            (self.order.0, rhs.order.1),
        );
    }
}
impl Mul<i32> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: i32) -> Self::Output {
        return Matrix::generate(|i, j| self.get(i, j).unwrap() * rhs, self.order);
    }
}
impl MulAssign for Matrix {
    fn mul_assign(&mut self, rhs: Self) {
        if self.order.1 != rhs.order.0 {
            eprintln!("Error: {}", Errors::IncorrectOrdersForOperation);
            panic!();
        }
        *self = Matrix::generate(
            |i, j| {
                let mut sum = 0;
                let a = self.get_row(i).unwrap();
                let b = rhs.get_column(j).unwrap();
                for r in 0..self.order.1 {
                    sum += a[r as usize] * b[r as usize]
                }
                sum
            },
            (self.order.0, rhs.order.1),
        );
    }
}
impl MulAssign<i32> for Matrix {
    fn mul_assign(&mut self, rhs: i32) {
        *self = Matrix::generate(|i, j| self.get(i, j).unwrap() * rhs, self.order);
    }
}
impl Index<(u32, u32)> for Matrix {
    type Output = i32;
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

mod tests {
    #[test]
    fn addition() {
        use crate::Matrix;
        let a = Matrix::new(vec![45, 2, 65, 899, 6, 61], (3, 2)).unwrap();
        let b = Matrix::new(vec![4, 87, 2, 99, 12, 44], (3, 2)).unwrap();

        let sum = a + b;
        assert_eq!(sum.order, (3, 2));
        assert_eq!(sum.items, vec![49, 89, 67, 998, 18, 105]);
    }

    #[test]
    fn subtraction() {
        use crate::Matrix;
        let a = Matrix::new(vec![45, 2, 65, 899, 6, 61], (3, 2)).unwrap();
        let b = Matrix::new(vec![4, 87, 2, 99, 12, 44], (3, 2)).unwrap();

        let diff = a - b;
        assert_eq!(diff.order, (3, 2));
        assert_eq!(diff.items, vec![41, -85, 63, 800, -6, 17]);
    }

    #[test]
    fn multiplication() {
        use crate::Matrix;
        let a = Matrix::new(vec![4, 87, 2, 99, 12, 44], (3, 2)).unwrap();

        let scalar_mul = a.clone() * 5;
        assert_eq!(scalar_mul.order, (3, 2));
        assert_eq!(scalar_mul.items, vec![20, 435, 10, 495, 60, 220]);

        let b = Matrix::new(vec![45, 2, 65, 899, 6, 61], (2, 3)).unwrap();

        let matrix_mul = a * b;
        assert_eq!(matrix_mul.order, (3, 3));
        assert_eq!(
            matrix_mul.items,
            vec![78393, 530, 5567, 89091, 598, 6169, 40096, 288, 3464]
        );
    }
}
