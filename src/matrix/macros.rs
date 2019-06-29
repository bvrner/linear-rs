//implementation of all basic ops to the matrix structs
macro_rules! impl_mat_ops {
    //basic ops
    ($MatN:ident, $field:ident, $dimension:expr, $indextype:ty) => {
        use std::ops::*;

        impl Add<$MatN> for $MatN {
            type Output = Self;

            fn add(self, other: $MatN) -> Self {
                let mut ret = $MatN::default();

                for i in 0..$dimension {
                    for j in 0..$dimension {
                        ret.$field[i][j] = self.$field[i][j] + other.$field[i][j];
                    }
                }
                ret
            }
        }

        impl AddAssign<$MatN> for $MatN {
            fn add_assign(&mut self, other: $MatN) {
                for i in 0..$dimension {
                    for j in 0..$dimension {
                        self.$field[i][j] += other.$field[i][j];
                    }
                }
            }
        }

        impl Sub<$MatN> for $MatN {
            type Output = Self;

            fn sub(self, other: $MatN) -> Self {
                let mut ret = $MatN::default();

                for i in 0..$dimension {
                    for j in 0..$dimension {
                        ret.$field[i][j] = self.$field[i][j] - other.$field[i][j];
                    }
                }
                ret
            }
        }

        impl SubAssign<$MatN> for $MatN {
            fn sub_assign(&mut self, other: $MatN) {
                for i in 0..$dimension {
                    for j in 0..$dimension {
                        self.$field[i][j] -= other.$field[i][j];
                    }
                }
            }
        }

        impl Mul<f32> for $MatN {
            type Output = Self;

            fn mul(self, other: f32) -> Self {
                let mut ret = $MatN::default();

                for i in 0..$dimension {
                    for j in 0..$dimension {
                        ret.$field[i][j] = self.$field[i][j] * other;
                    }
                }
                ret
            }
        }

        impl Mul<$MatN> for $MatN {
            type Output = Self;

            fn mul(self, rhs: $MatN) -> Self {
                let mut ret = $MatN::zero();

                for i in 0..$dimension {
                    for j in 0..$dimension {
                        for k in 0..$dimension {
                            ret[i][j] += self[k][j] * rhs[i][k];
                        }
                    }
                }
                ret
            }
        }

        impl Index<usize> for $MatN {
            type Output = $indextype;

            fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
                &self.$field[index]
            }
        }

        impl IndexMut<usize> for $MatN {
            fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut $indextype {
                &mut self.$field[index]
            }
        }
    };

    //Matrix x Vector multiplication
    ($MatN:ident, $VecN:ident, $dimension:expr) => {
        use std::ops::Mul;

        impl Mul<$VecN> for $MatN {
            type Output = $VecN;

            fn mul(self, other: $VecN) -> $VecN {
                let mut ret = $VecN::default();

                for i in 0..$dimension {
                    for j in 0..$dimension {
                        ret[j] += other[i] * self[i][j];
                    }
                }
                ret
            }
        }
    };
}
