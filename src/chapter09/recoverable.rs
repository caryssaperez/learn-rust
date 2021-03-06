// We can use the Result enum to handle recoverable errors, like trying to write
// to a file that doesn't exist.

use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

// This way we manually handle errors and return early if there's an error reading
// the file.
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// This way uses ? which short circuits the function if any errors occur,
// implicitly returning early if an error is encountered. Can only be used with
// functions that return a Result.
fn read_username_from_file_with_question_mark() -> Result<String, io::Error> {
    // fs::read_to_string("hello.txt") <-- can be boiled down to just that at the end
    // of the day.
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let f = File::open("hello.txt");

    // Handle the error with a match, which isn't as sexy as using closures and unwrap_or_else.
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!(
                    "Tried to create file but there was a problem: {:?}",
                    e
                ),
            },
            other_error => panic!(
                "There was a problem opening the file: {:?}",
                other_error
            ),
        },
    };

    // Or handle by propogating error to what is calling the code.
    let result = read_username_from_file();
    print!("{:?}", result);
}
