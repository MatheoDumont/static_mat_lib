use num::traits::Float;

use std::{
    ops::{Div, DivAssign, Index, Mul, MulAssign},
};
/**
 * Static Matrix Library
 * Raisons:
 *  Je n'ai pas besoins de type dynamique, d'avoir des matrix<type, n, n> avec n inconnu.
 *
 *
 * Avantages:
 *  Pour l'implementation, le compilateur aide beaucoup pour détecter les erreurs de tailles.
 *  
 *  
 * Désavantages:
 *  Complique l'implémentation,
 *  Le binary sera bien plus lourd, car pour chaque type déclaré, il écrira toutes les fonctions avec ROW, COL spécialisés.
 *  Aucunes actions où la taille est inconnue à la compilation ne peuvent être accomplies.
 *  Je ne peut pas modifier la taille des données stockées, je suis obligé d'en créer un nouveau, par exemple pour transposer.
 */

// type Mat3 = Matrix<Real, 3, 3>;
// type Mat4 = Matrix<Real, 4, 4>;

pub struct Matrix<Type: Float, const ROW: usize, const COL: usize> {
    values: [[Type; COL]; ROW],
}

impl<Type: Float, const ROW: usize, const COL: usize> Matrix<Type, ROW, COL> {
    pub fn size(&self) -> (usize, usize) {
        (ROW, COL)
    }

    pub fn transposed(&self) -> Matrix<Type, COL, ROW> {
        let mut m = Matrix::<Type, COL, ROW>::zero();
        for r in 0..ROW {
            for c in 0..COL {
                m.values[c][r] = self.values[r][c];
            }
        }
        m
    }

    pub fn zero() -> Matrix<Type, ROW, COL> {
        Matrix {
            values: [[Type::zero(); COL]; ROW],
        }
    }

    pub fn values(v: Type) -> Matrix<Type, ROW, COL> {
        Matrix {
            values: [[v; COL]; ROW],
        }
    }
    /**
     * from_1d_array returns a column vector
     */
    pub fn from_1d_array(arr: [Type; COL]) -> Matrix<Type, COL, 1> {
        Matrix { values: [arr] }.transposed()
    }

    pub fn from_2d_array(arr: [[Type; COL]; ROW]) -> Matrix<Type, ROW, COL> {
        Matrix { values: arr }
    }

    // pub fn at_ref(&self, row: usize, col: usize) -> Type {
    //     self.values[row][col]
    // }

    // pub fn at_mut(&mut self, row: usize, col: usize) -> &mut Type {
    //     &mut self.values[row][col]
    // }

    pub fn col(&self, col: usize) -> Matrix<Type, ROW, 1> {
        let mut c = Matrix::<Type, ROW, 1>::zero();
        for row in 0..ROW {
            c.values[row][0] = self.values[row][col];
        }
        c
    }

    pub fn set_col(&mut self, idx: usize, col: Matrix<Type, ROW, 1>) {
        for i in 0..ROW {
            self.values[i][idx] = col.values[i][0];
        }
    }

    pub fn row(&self, row: usize) -> Matrix<Type, 1, COL> {
        let mut r = Matrix::<Type, 1, COL>::zero();
        for col in 0..COL {
            r.values[0][col] = self.values[row][col];
        }
        r
    }

    pub fn set_row(&mut self, idx: usize, row: Matrix<Type, 1, COL>) {
        self.values[idx] = row.values[0];
    }
}

// impl<Type,Idx, const ROW: usize, const COL: usize > Index<Idx> for Matrix<Type, ROW, COL>
// where
// Type:Float,
// Idx: std::slice::SliceIndex<Matrix<Type, ROW, COL>>,
//  {
//     type Output = Idx::Output;

//     fn index(&self, index: Idx) -> &Self::Output {
//         &self.values[index]
//     }
//}

// impl<Type: Float, const ROW: usize> Matrix<Type, ROW, 1> {
//     pub fn at_ref(&self, row: usize) -> Type {
//         self.values[row][0]
//     }

//     pub fn at_mut(&mut self, col: usize) -> &mut Type {
//         &mut self.values[0][col]
//     }
// }

// ==================================== Operateurs Matrix Scalar ==================================== //

impl<Type, const ROW: usize, const COL: usize> Mul<Type> for &'static mut Matrix<Type, ROW, COL>
where
    Type: Float,
{
    type Output = &'static mut Matrix<Type, ROW, COL>;

    fn mul(self, rhs: Type) -> Self::Output {
        for r in 0..ROW {
            for c in 0..COL {
                self.values[r][c] = self.values[r][c] * rhs;
            }
        }

        self
    }
}

// ==================================== Operateurs Matrix Matrix ==================================== //

// impl<
//         Type,
//         const LeftMatRow: usize,
//         const LeftMatCol: usize,
//         const RightMatRow: usize,
//         const RightMatCol: usize,
//     > Mul<Matrix<Type, RightMatRow, RightMatCol>> for Matrix<Type, LeftMatRow, LeftMatCol>
// where
//     Type: Float,
// {
//     type Output = Matrix<Type, LeftMatRow, RightMatCol>;

//     fn mul(self, rhs: Matrix<Type, RightMatRow, RightMatCol>) -> Self::Output {
//         assert!(LeftMatCol == RightMatRow);
//         let m = Matrix::zero();
//         // for row_left_col_right in 0..RightMatCol {
//         //     let r = Type::zero();
//         //     for row_right_col_left in 0..LeftMatCol {
//         //         r = r + self.values[row_left_col_right][row_right_col_left]
//         //             * rhs.values[row_right_col_left][row_left_col_right]
//         //     }
//         //     m.values
//         // }

//         /*
//         On ne peut pas ecrire:
//         ```rust
//             let left_row_i: Matrix<Type, 1, RightMatRow>;
//             let right_col_i: Matrix<Type, RightMatRow, 1>;
//         ```
//         meme s'il a ete verifié qu'ils sont égaux (LeftMatCol==RightMatRow) car le compilateur rale.
//         */
//         let left_row_i: Matrix<Type, 1, LeftMatCol>;
//         let right_col_i: Matrix<Type, RightMatRow, 1>;

//         for i in 0..RightMatCol {
//             left_row_i = self.row(i);
//             right_col_i = rhs.col(i);
//             let r = 0;
//             for j in 0..RightMatRow {
//                 r = r +
//             }
//             for j in 0..RightMatRow {
//                 m.values[i][j] =
//             }
//         }

//         m
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_test() {
        let mat_colonne = Matrix::<Real, 5, 1>::zero();

        for i in 0..5 {
            assert_eq!(mat_colonne.values[i][0], 0 as Real);
        }

        let mat_ligne = Matrix::<Real, 1, 5>::zero();

        for i in 0..5 {
            assert_eq!(mat_ligne.values[0][i], 0 as Real);
        }
    }
    #[test]
    fn matrix_col_and_row_fn() {
        let mut mat_colonne = Matrix::<Real, 5, 5>::zero();
        for i in 0..5 {
            for j in 0..5 {
                mat_colonne.values[i][0] = (i * 5 + j) as Real;
            }
        }

        let c = mat_colonne.col(0);
        for i in 0..5 {
            assert_eq!(c.values[i][0], mat_colonne.values[i][0]);
        }

        let r = mat_colonne.row(1);
        for i in 0..5 {
            assert_eq!(r.values[0][i], mat_colonne.values[1][i]);
        }
    }
    #[test]
    fn matrix_set_col_row_fn() {
        let mut mat = Matrix::<Real, 5, 5>::zero();
        for i in 0..5 {
            for j in 0..5 {
                mat.values[i][0] = (i * 5 + j) as Real;
            }
        }
        let col = [121, 122, 123, 124, 125].map(|e| e as Real);
        let c = Matrix::<Real, 1, 5>::from_1d_array(col);
        mat.set_col(0, c);

        for i in 0..5 {
            assert_eq!(mat.values[i][0], col[i]);
        }
    }
}
