fn main() {
    let search_term = "picture";
    let quote = "\
Every face, every shop, bedroom window, public-house and
dark square is a picture feverishly turned--in search of what?
It is the same with books.
dark square is a feverishly turned--in search of what?
It is the same with books.
dark square is a feverishly turned--in search of what?
It is the same with books.
dark square is a feverishly turned--in search of what?
It is the same with books.
It is the same with books.
What do we seek through millions of pages?";
    let mut matches: Vec<usize> = vec![];
    let context = 1;

    for (i, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            println!("{}: {}", i+1, line);
            matches.push(i+1);
        }
    }
    println!("\n\nContext");

    for (i, line) in quote.lines().enumerate() {
        for found_line_num in &matches {
            if *found_line_num  > i.wrapping_sub(context) {
                println!("{}", line);
            } 
            else if *found_line_num < i + context {
                println!("{}", line);
            }
        }
    }

    //let mut integer = 34_i32;
    //println!("{}", integer);
    //
    //let array = [0; 30];
    //println!("{:?}", array);
}
