use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::{Seek, SeekFrom};
use regex::Regex;
use clap::{App, Arg};

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("filename")
            .help("Files we are searching")
            .takes_value(true)
            .required(true))
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let filename = args.value_of("filename").unwrap();
    let re = Regex::new(pattern).unwrap();

//    let quote = "\
//Every face, every shop, bedroom window, public-house and
//dark square is a picture feverishly turned--in search of what?
//It is the same with books.
//dark square is a feverishly turned--in search of what?
//It is the same with books.
//dark square is a feverishly turned--in search of what?
//It is the same with books.
//dark square is a feverishly turned--in search of what?
//It is the same with books.
//It is the same with books.
//What do we seek through millions of pages?";
    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];
    let context = 0;

    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(&file);

    for (i, line) in (&mut reader).lines().enumerate() {
        let new_line = line.unwrap();
        let contains_substring = re.find(&new_line);
        match contains_substring {
            Some(_) => {
                tags.push(i);

                let v = Vec::with_capacity(2*context+1);
                ctx.push(v);
            },
            None => (),
        }
    }

    if tags.is_empty() {
        return;
    }

    reader.seek(SeekFrom::Start(0));

    let reader2 = BufReader::new(&file);
    for (i, line) in reader2.lines().enumerate() {
        let unwrapped_line = line.unwrap();
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(context);
            let upper_bound = tag + context;
            
            if (i >= lower_bound) && (i <= upper_bound) {
                let local_ctx = (i, unwrapped_line.clone());
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
