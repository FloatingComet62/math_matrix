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
/// let det = Determinant::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
/// let invalid_det = Determinant::new(vec![1, 2, 3]);
///
/// assert_eq!(invalid_det.is_ok(), false);
/// assert_eq!(det.is_ok(), true);
/// let det = det.unwrap();
/// assert_eq!(det.value(), 0);
/// ```
pub struct Determinant {
    items: Vec<i32>,
    pub size: u32,
}
impl Determinant {
    pub fn new(items: Vec<i32>) -> Result<Determinant, Errors> {
        let size = (items.len() as f32).sqrt();
        if size.fract() != 0.0 {
            return Err(Errors::InappropriateNumberOfItems);
        }
        let size = size as u32;
        Ok(Determinant { items, size })
    }
    fn value_inner(&self, items: Vec<i32>) -> i32 {
        // just in case :)
        if items.is_empty() {
            return 0;
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
        let mut value = 0;
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
            if i % 2 == 0 {
                value += minor * item;
            } else {
                value -= minor * item;
            }
        }
        value
    }
    /// Calculate the value of determinant
    /// ## Examples
    /// ```
    /// use math_matrix::Determinant;
    /// let det = Determinant::new(vec![9, 8, 4, 8, 3, 2, 4, 3, 2]).unwrap();
    ///
    /// assert_eq!(det.value(), -16);
    /// ```
    pub fn value(&self) -> i32 {
        self.value_inner(self.items.clone())
    }
}

mod tests {
    #[test]
    fn value() {
        macro_rules! value_checker {
            ($items: expr, $expected_value: expr) => {
                let d = Determinant::new($items);
                assert_eq!(d.is_ok(), true);
                let d = d.unwrap();
                assert_eq!(d.value(), $expected_value);
            };
        }
        use crate::Determinant;
        value_checker!(vec![1], 1);
        value_checker!(vec![1, 2, 3, 4], -2);
        value_checker!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 0);
        value_checker!(vec![1, 3, 5, 9, 1, 3, 1, 7, 4, 3, 9, 7, 5, 2, 0, 9], -376);
        value_checker!(
            vec![
                9, 8, 4, 4, 78, 8, 3, 2, 56, 45, 43, 13, 23, 42, 99, 1, 35, 4, 77, 108, 25, 1, 87,
                199, 78,
            ],
            -283039494
        );
    }
}
