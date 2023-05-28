use crate::Errors;

/// # Determinant
/// The determinant is a scalar value that is a function of the entries of a square matrix. It characterizes some properties of the matrix and the linear map represented by the matrix.<br>
/// eg.<br>
/// | 1  2  3 |<br>
/// | 4  5  6 | == 0<br>
/// | 7  8  9 |<br>
///
///
/// ## Examples
/// ```
/// use math_matrix::Determinant;
/// let det = Determinant::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
/// let invalid_det = Determinant::new(vec![1.0, 2.0, 3.0]);
///
/// assert_eq!(invalid_det.is_ok(), false);
/// assert_eq!(det.is_ok(), true);
/// let det = det.unwrap();
/// assert_eq!(det.value(), 0.0);
/// ```
pub struct Determinant {
    items: Vec<f64>,
    pub size: u32,
}
impl Determinant {
    pub fn new(items: Vec<f64>) -> Result<Determinant, Errors> {
        let size = (items.len() as f32).sqrt();
        if size.fract() != 0.0 {
            return Err(Errors::InappropriateNumberOfItems);
        }
        let size = size as u32;
        Ok(Determinant { items, size })
    }
    fn value_inner(&self, items: Vec<f64>) -> f64 {
        // just in case :)
        if items.is_empty() {
            return 0.0;
        }

        if items.len() == 1 {
            // 1x1 determinant
            return items[0];
        }
        if items.len() == 4 {
            // 2x2 determinant
            return items[0] * items[3] - items[1] * items[2];
        }

        // we are already calculating along the first column
        let mut value = 0.0;
        let new_size = (items.len() as f32).sqrt() as u32;
        for i in 0..new_size {
            let item = items[(i * new_size) as usize];
            let minor = self.value_inner(
                items
                    .iter()
                    .enumerate()
                    .filter(|&(j, _)| {
                        let row = j as u32 / new_size;
                        let column = j as u32 % new_size;
                        if i == row || column == 0 {
                            return false;
                        }
                        true
                    })
                    .map(|(_, x)| *x)
                    .collect(),
            );
            let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
            value += minor * item * sign;
        }
        value
    }
    /// Calculate the value of determinant
    /// ```
    /// use math_matrix::Determinant;
    /// let det = Determinant::new(vec![9.0, 8.0, 4.0, 8.0, 3.0, 2.0, 4.0, 3.0, 2.0]).unwrap();
    ///
    /// assert_eq!(det.value(), -16.0);
    /// ```
    pub fn value(&self) -> f64 {
        self.value_inner(self.items.clone())
    }
    /// Get the cofactor of an item
    /// ```
    /// use math_matrix::Determinant;
    /// let det = Determinant::new(vec![1.0, 2.0, 3.0, 4.0]).unwrap();
    /// let det3x3 = Determinant::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]).unwrap();
    ///
    /// assert_eq!(det.cofactor(1, 1).unwrap(), 4.0);
    /// assert_eq!(det3x3.cofactor(1, 2).unwrap(), 6.0);
    /// ```
    pub fn cofactor(&self, i: u32, j: u32) -> Result<f64, Errors> {
        if i == 0 || i > self.size || j == 0 || j > self.size {
            return Err(Errors::IndexOutOfRange);
        }
        let minor = self.value_inner(
            self.items
                .iter()
                .enumerate()
                .filter(|&(x, _)| {
                    let row = x as u32 / self.size;
                    let column = x as u32 % self.size;
                    if (i - 1) == row || (j - 1) == column {
                        return false;
                    }
                    true
                })
                .map(|(_, x)| *x)
                .collect(),
        );
        let sign = if i % 2 == 0 { -1.0 } else { 1.0 };
        let sign = if j % 2 == 0 { -sign } else { sign };
        Ok(minor * sign)
    }
}

mod tests {
    #[test]
    fn value() {
        macro_rules! value_checker {
            ($items: expr, $expected_value: expr) => {
                let d = Determinant::new($items);
                assert_eq!(d.is_ok(), true);
                let d = d.expect("Impossible");
                assert_eq!(d.value(), $expected_value);
            };
        }
        use crate::Determinant;
        value_checker!(vec![1.0], 1.0);
        value_checker!(vec![1.0, 2.0, 3.0, 4.0], -2.0);
        value_checker!(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0], 0.0);
        value_checker!(
            vec![1.0, 3.0, 5.0, 9.0, 1.0, 3.0, 1.0, 7.0, 4.0, 3.0, 9.0, 7.0, 5.0, 2.0, 0.0, 9.0],
            -376.0
        );
        value_checker!(
            vec![
                9.0, 8.0, 4.0, 4.0, 78.0, 8.0, 3.0, 2.0, 56.0, 45.0, 43.0, 13.0, 23.0, 42.0, 99.0,
                1.0, 35.0, 4.0, 77.0, 108.0, 25.0, 1.0, 87.0, 199.0, 78.0,
            ],
            -283039494.0
        );
    }
}
