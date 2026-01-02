pub fn i_to_u((x, y): (isize, isize)) -> (usize, usize) {
    (usize::try_from(x).unwrap(), usize::try_from(y).unwrap())
}

pub fn u_to_i((x, y): (usize, usize)) -> (isize, isize) {
    (isize::try_from(x).unwrap(), isize::try_from(y).unwrap())
}
