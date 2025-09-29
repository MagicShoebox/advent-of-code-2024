use ndarray::{Array, Array2, Dimension, IntoDimension, NdIndex, ShapeError};
use std::iter;

pub trait Array2Ext {
    fn from_string<A, F>(input: &str, f: F) -> Result<Array2<A>, ShapeError>
    where
        F: Fn(char) -> A,
    {
        let mut flat_vec = Vec::new();
        let mut rows = 0;
        let mut cols = 0;
        for (r, row) in input.lines().enumerate() {
            for (c, x) in row.chars().enumerate() {
                flat_vec.push(f(x));
                cols = c;
            }
            rows = r;
        }
        Array2::from_shape_vec((rows + 1, cols + 1), flat_vec)
    }
}

impl<A> Array2Ext for Array2<A> {}

pub trait ArrayExt<D, I>
where
    I: NdIndex<D>,
{
    type Output;
    fn neighbors(&self, ix: I) -> impl Iterator<Item = Self::Output>;
}

impl<A, D, I> ArrayExt<D, I> for Array<A, D>
where
    D: Dimension,
    I: NdIndex<D> + IntoDimension,
{
    type Output = <<I as IntoDimension>::Dim as Dimension>::Pattern;
    fn neighbors(&self, ix: I) -> impl Iterator<Item = Self::Output> {
        let mut n = 0;
        let ix = ix.into_dimension();
        iter::from_fn(move || loop {
            let i = n / 2;
            if i >= self.ndim() {
                return None;
            }
            if n % 2 == 0 {
                if ix[i] > 0 {
                    n += 1;
                    let mut nghbr = ix.clone();
                    nghbr[i] -= 1;
                    return Some(nghbr.into_pattern());
                }
            } else {
                if ix[i] + 1 < self.shape()[i] {
                    n += 1;
                    let mut nghbr = ix.clone();
                    nghbr[i] += 1;
                    return Some(nghbr.into_pattern());
                }
            }
            n += 1;
        })
    }
}
