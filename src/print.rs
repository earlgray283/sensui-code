pub trait Print<T> {
    fn print_deco(&self);
}
impl<T: std::fmt::Display> Print<T> for Vec<Vec<T>> {
    // print decolated Vec<Vec<T>>
    fn print_deco(&self) {
        let mut v = Vec::new();

        let mut max = 0;
        for i in 0..5 {
            let mut buf = String::new();
            buf.push_str("| ");
            for j in 0..5 {
                buf.push_str(&format!("{:12} ", self[i][j]));
            }
            buf.push_str("|\n");
            max = max.max(buf.len() - 3);
            v.push(buf);
        }

        print!("+");
        for _ in 0..max {
            print!("-");
        }
        println!("+");
        for s in &v {
            print!("{}", s);
        }
        print!("+");
        for _ in 0..max {
            print!("-");
        }
        println!("+");
    }
}