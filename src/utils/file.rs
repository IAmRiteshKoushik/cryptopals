use std::{
    fs::{File, metadata, read_to_string},
    io::{self, BufRead, BufReader, Read},
    path::Path,
};

// -- ACTUALLY USEFUL FUNCTIONS
pub fn read_lines_from_file(path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line_result in reader.lines() {
        // Longer version
        // match line_result {
        //     Ok(line) => lines.push(line),
        //     Err(e) => return Err(e),
        // }

        // -- Smart alternative
        let line = line_result?;
        lines.push(line);
    }

    Ok(lines)
}

// -- EXPERIMENTAL FUNCTIONS
// These are a bunch of functions that I wrote for checking out how things
// work in Rust. They are written only for experimentation purposes and do not
// really contribute to the solutions of cryptopals
fn simple_file_reader() -> io::Result<()> {
    let file_contents = read_to_string("info.txt")?;
    println!("info.txt content =\n{file_contents}");
    Ok(())
}

fn read_file_after_verifying_metadata() -> io::Result<()> {
    let file_path = "info.txt";
    match metadata(file_path) {
        Ok(metadata) => {
            if metadata.is_file() {
                let file_contents = read_to_string(file_path)?;
                println!("info.txt content =\n{file_contents}");
            } else {
                println!("Path exists, but it's not a file.");
            }
        }
        Err(_) => {
            println!("File does not exist.");
        }
    }
    Ok(())
}

// Error handling for not found
fn read_file_with_error_handling() -> io::Result<()> {
    let mut file = match File::open("info.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                println!("File not found");
                return Ok(());
            }
            _ => return Err(error),
        },
    };
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    println!("File contents: {:?}", contents);
    Ok(())
}

// Using file-buffers for faster reading
fn read_file_wit_buffer() -> io::Result<()> {
    // -- Shorthand
    // let file = File::open("info.txt")?;
    let file = match File::open("info.txt") {
        Ok(val) => val,
        Err(err) => return Err(err), // this error must either satisfy or must
                                     // be convertible to the function's error
    };
    let reader = BufReader::new(file);

    // -- Shorthand
    // println!("{}", line?);
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => println!("{}", line),
            Err(err) => return Err(err),
        }
    }
    Ok(())
}

// Reading the file as a vector (has more utility)
// And converting it into a string
fn read_file_as_vector() -> io::Result<()> {
    let mut file = File::open("assets/s1c3.txt")?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    match String::from_utf8(contents) {
        Ok(text) => println!("File contents as string: {:?}", text),
        Err(e) => println!("Failed to parse contents as UTF-8 string: {:?}", e),
    }

    Ok(())
}
