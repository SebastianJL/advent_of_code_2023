use std::io::Write;
use std::path::PathBuf;

fn main() {
    let sample_file = PathBuf::from("day_01_01.old");
    let sample_content = std::fs::read_to_string(sample_file).unwrap();
    let mut counter = 0;
    for day in 1..=24 {
        for exercise_number in 1..=2 {
            let new_content = sample_content.replace(
                "day_01_01.txt",
                &format!("day_{:02}_{:02}.txt", day, exercise_number),
            );
            let new_file =
                PathBuf::from(format!("src/bin/day_{:02}_{:02}.rs", day, exercise_number));
            if !new_file.exists() {
                let mut new_file = std::fs::File::create(new_file).unwrap();
                new_file.write_all(new_content.as_bytes()).unwrap();
                counter += 1;
            }
        }
    }

    println!("Written {counter} new files.");
}
