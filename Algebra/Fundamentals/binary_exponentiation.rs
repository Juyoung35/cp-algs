/*
a^n을 O(logn)으로 계산하는 트릭
거듭제곱 외에도 associativity를 가진 요소들이면 적용 가능
mod 포함 곱셈, 행렬의 곱셈에 이용
a^n={1 if n == 0
    {(a^(n/2))^2 if n > 0 and n even
    {(a^((n-1)/2))^2 if n > 0 and n odd
*/
fn binpow_usize_recursive(a: usize, b: usize) -> usize {
    if b == 0 { return 1 }
    let mut res = binpow(a, b >> 1);
    if b % 2 == 0 {
        res * res
    } else {
        res * res * a
    }
}

fn binpow_usize(mut a: usize, mut b: usize) -> usize {
    let mut res = 1;
    while b > 0 {
        if b % 2 == 1 {
            res *= a;
        }
        a *= a;
        b >>= 1;
    }
    res
}

// Effective computation of large exponents modulo a number
// https://cp-algorithms.com/algebra/module-inverse.html
// Problem: compute (x^n) mod m
fn sol_1(mut a: usize, mut b: usize, m: usize) -> usize {
    let mut res = 1;
    a %= m;
    while b > 0 {
        if b % 2 == 1 {
            res = res * a % m;
        }
        a = a * a % m;
        b >>= 1;
    }
    res
}

// Effective computation of Fibonacci numbers
// Problem: compute n-th Fibonacii number
// (1 1)^n   (F_(n+1) F_n    )
// (1 1)   = (F_n     F_(n-1))
fn sol_2(n:usize) -> usize {
    struct Matrix<T, const N: usize, const M: usize>([[T; M]; N])
    impl Mul for Matrix<T, const N: usize, const M: usize> {
        fn mul(self, rhs: Self) -> Self::Output {
            for i in 0..self.0.len() {
                for j in 0..rhs.0[0].len() {
                    
                }
            }
        }
    }
}
