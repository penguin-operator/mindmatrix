use mindmatrix::matrix::Matrix;

#[test]
fn matrix_multiplication() {
    let a = Matrix::from(&[
        &[0.0, 3.0, 5.0],
        &[5.0, 5.0, 2.0],
    ]);

    let b = Matrix::from(&[
        &[3.0, 4.0],
        &[3.0, -2.0],
        &[4.0, -2.0],
    ]);

    assert_eq!(a.multiply(&b), Matrix::from(&[
        &[29.0, -16.0],
        &[38.0, 6.0],
    ]));
}
