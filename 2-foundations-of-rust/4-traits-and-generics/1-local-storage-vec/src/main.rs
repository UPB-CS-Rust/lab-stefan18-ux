fn main() {
    let mut buf: [i32; 10] = [0; 10];
    let mut len = 3;
    let pos = 1;
    for i in 0..len {
        buf[i] = i as i32;
    }
    for i in 0..len {
        print!("{} ", buf[i]);
    }
    println!(" ");
    for i in (pos..len + 1).rev() {
        buf[i] = buf[i - 1];
    }
    buf[pos] = 3;
    len += 1;
    for i in 0..len {
        print!("{} ", buf[i]);
    }
}
