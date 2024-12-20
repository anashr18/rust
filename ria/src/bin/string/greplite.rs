fn array_related() {
    let one = [1, 2, 3];
    let two = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2 = [0; 3];
    let array = [one, two, blank1, blank2];

    for a in &array {
        print!("{:?}: ", a);
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }
        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t(SUM{:?} : {})", a, sum);
    }
}
fn main() {
    let quote = "\
    Every face, every shop, bedroom window, public-house, and
    dark square is a picture feverishly turned--in search of what?
    It is the same with books.It is the same with books.
    What do we seek through millions of pages?";
    let search_term = "picture";
    for (i, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            println!("{} {}", i + 1, line);
        }
    }
    println!("{}", quote);
    array_related();
}
