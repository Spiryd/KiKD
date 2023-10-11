use l1::entropy;

fn main() {
    let test_files = vec!["test_cases\\test1.bin", "test_cases\\test2.bin", "test_cases\\test3.bin", "test_cases\\pan-tadeusz-czyli-ostatni-zajazd-na-litwie.txt", "test_cases\\bee_movie_test_suite\\bee_movie_script_x12.txt", "test_cases\\bee_movie_test_suite\\bee_movie_script_x12.doc", "test_cases\\bee_movie_test_suite\\bee_movie_script_x12.pdf", "test_cases\\bee_movie_test_suite\\beemovie.jpg", "test_cases\\bee_movie_test_suite\\beemovie.mp4"];
    for file_path in test_files {
        println!("{file_path}");
        let file = std::fs::read(file_path).unwrap();
        entropy(file);
    }
}