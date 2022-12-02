use rlib_io::reader::Reader;
use rlib_io::writer::{Writable, Writer};
use rlib_tensor::Tensor;

fn make_reader(s: &str) -> Reader {
    Reader::new(Box::new(std::io::Cursor::new(
        s.as_bytes().iter().cloned().collect::<Vec<u8>>(),
    )))
}

fn write<T: Writable>(t: T) -> String {
    let mut v = Vec::new();
    let mut writer = Writer::new(Box::new(&mut v));
    writer.write(&t);
    drop(writer);
    std::str::from_utf8(&v).unwrap().to_string()
}

#[test]
fn constuctors() {
    type T = Tensor<i32, 2>;
    let mut t1 = T::new([2, 4], 42);
    for i in 0..t1.dim(0) {
        for j in 0..t1.dim(1) {
            t1[[i, j]] = (i * 10 + j) as i32;
        }
    }

    let t2 = T::from_vec([2, 4], vec![0, 1, 2, 3, 10, 11, 12, 13]);
    let t3 = T::from_slice([2, 4], &[0, 1, 2, 3, 10, 11, 12, 13]);
    let mut r = make_reader("0 1 2 3 10 11 12 13");
    let t4 = T::read([2, 4], &mut r);

    assert_eq!(t1, t2);
    assert_eq!(t1, t3);
    assert_eq!(t1, t4);
}

#[test]
#[should_panic]
fn out_of_bounds() {
    let mut t = Tensor::<i32, 3>::new([3, 4, 5], 42);
    t[[0, 0, 5]] = 41;
}

#[test]
#[should_panic]
fn zero_dim() {
    let _ = Tensor::<i32, 1>::new([0], 0);
}

#[test]
fn output() {
    let t: Tensor<i32, 3> = Tensor::from_vec([2, 2, 3], vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    assert_eq!(write(t), "0 1 2\n3 4 5\n\n6 7 8\n9 10 11");
}
