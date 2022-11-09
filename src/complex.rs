
pub mod complex {
    use std::fmt;
    use std::ops::{Add, Sub, Mul, Div, Neg};

    /**
     * Struct of complex number with any underlying type
     */
    #[derive(Debug, Clone)]
    pub struct Complex <N> {
        pub re : N,
        pub im : N,
    }

    impl <N> Complex<N> {
        /**
         * Creates new complex number 
         */
        pub fn new (re: N, im : N) -> Self {
            Complex { re, im }
        }  
    }

    /**
     * Implementation of + operator
     */
    impl <N: Add<N, Output = N>> Add<Complex<N>> for Complex<N> {
        type Output = Complex<N>;

        fn add(self, rhs: Complex<N>) -> Self::Output {
            Complex {re : self.re + rhs.re, im : self.im + rhs.im}
        }
    }

    /**
     * Implementation of unary - operator 
     */
    impl <N: Neg<Output = N>> Neg for Complex<N> {
        type Output = Complex<N>;

        fn neg(self) -> Self::Output {
            Complex {re : -self.re, im : -self.im}
        }
    }

    /**
     * Implementation of binary - operator 
     */
    impl <N: Sub<N, Output = N>> Sub<Complex<N>> for Complex<N> {
        type Output = Complex<N>;

        fn sub(self, rhs: Complex<N>) -> Self::Output {
            Complex {re : self.re - rhs.re, im : self.im - rhs.im}
        }
    }

    /**
     * Implementation of * operator 
     */
    impl <N: Mul<N, Output = N> + Add<N, Output = N> + Sub<N, Output = N> + Clone> Mul<Complex<N>> for Complex<N> {
        type Output = Complex<N>;

        fn mul(self, rhs: Complex<N>) -> Self::Output {
            let re = self.re.clone() * rhs.re.clone() - self.im.clone() * rhs.im.clone();
            let im = self.re * rhs.im + self.im * rhs.re;
            Complex {re, im}
        }
    }

    /**
     * Implementation of * operator for base number
     */
    impl <N: Mul<N, Output = N> + Clone> Mul<N> for Complex<N> {
        type Output = Complex<N>;

        fn mul(self, rhs: N) -> Self::Output {
            Complex {re : self.re * rhs.clone(), im : self.im * rhs}
        }
    }

    /**
     * Implementation of conjucture of complex number
     */
    impl <N : Neg<Output = N>> Complex<N>{
        pub fn conjugate(self) -> Self{
            Complex { re: self.re, im: -self.im }
        }
    }

    /**
     * Implementation of squared module of complex number
     */
    impl <N :  Mul<N, Output = N> + Add<N, Output = N> + Clone> Complex<N>{
        pub fn module_sq(self) -> N{
            self.re.clone() * self.re + self.im.clone() * self.im
        }
    }

    /**
     * Implementation of / operator
     */
    impl <N> Div<Complex<N>> for Complex<N> where
        N : Mul<N, Output = N> + Div<N, Output = N> + Add<N, Output = N> + Sub<N, Output = N> + Neg<Output = N> + Clone{
        type Output = Complex<N>;

        fn div(self, rhs: Complex<N>) -> Self::Output {
            let nominator = self * rhs.clone().conjugate();
            let denominator = rhs.module_sq();
            Complex {re : nominator.re / denominator.clone(), im : nominator.im / denominator}
            //Complex { re : denominator.clone(), im : denominator}
        }
    }

    /**
     * Implementation of / operator for base number
     */
    impl <N: Div<N, Output = N> + Clone> Div<N> for Complex<N> {
        type Output = Complex<N>;

        fn div(self, rhs: N) -> Self::Output {
            Complex {re : self.re / rhs.clone(), im : self.im / rhs}
        }
    }

    /**
     * Implementation of displaying complex numbers
     */
    impl <N: fmt::Display> fmt::Display for Complex<N> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({})+({})i", self.re, self.im)
        }
    }
}

