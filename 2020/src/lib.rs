use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
/**
 * [[a0, a1, a2],
 *  [a3, a4, a5],
 *  [a6, a7, a8]]
 * det = a0*a4*a8 + a1*a5*a6 + a2*a3*a7
 *     - a2*a4*a6 - a1*a3*a8 - a0*a5*a7
 * define get_determinant() with 9 arguments
 * try all combinations
 */
pub fn q1() {
    let input = "./q1_in.txt";
    let output = "./q1_out.txt";
    if let Ok(lines) = read_lines(input) {
        let mut res = "".to_string();
        for line in lines {
            if let Ok(samples) = line {
                let mut iter = samples.split_whitespace();
                let x = iter.next().unwrap().parse().unwrap();
                let y = iter.next().unwrap().parse().unwrap();
                let d = iter.next().unwrap().parse().unwrap();
                res.push_str(&count_d(x, y, d).to_string());
                res.push_str("\n");
            }
        }
        fs::write(output, res).expect("Failed to write a file");
    } else {
        println!("There isn't a file name {}", input);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn count_d(x: i32, y: i32, d: i32) -> i32 {
    let candidates = vec![x, y];
    let mat_list = get_all_combinations(9, &candidates);
    let mut cnt = 0;
    for mat in mat_list.into_iter() {
        if get_determinant(&mat) == d {
            cnt += 1;
        }
    }
    cnt
}

fn get_determinant(mat: &[i32]) -> i32 {
    mat[0] * mat[4] * mat[8] + mat[1] * mat[5] * mat[6] + mat[2] * mat[3] * mat[7]
        - mat[2] * mat[4] * mat[6]
        - mat[1] * mat[3] * mat[8]
        - mat[0] * mat[5] * mat[7]
}

fn get_all_combinations<T: Clone>(len: usize, elements: &[T]) -> Vec<Vec<T>> {
    if len == 0 {
        return vec![vec![]];
    }
    let pre_comb = get_all_combinations(len - 1, elements);
    let mut res = vec![];
    for element in elements {
        for prev in &pre_comb {
            let mut inserted = prev.clone();
            inserted.push(element.to_owned());
            res.push(inserted)
        }
    }
    res
}

#[test]
fn test_count_d() {
    assert_eq!(count_d(0, 1, 2), 3);
    assert_eq!(count_d(0, 1, 99), 0);
}

#[test]
fn test_get_all_combinations() {
    let achieved = get_all_combinations(2, &[1, 3]);
    let expected = vec![vec![1, 1], vec![3, 1], vec![1, 3], vec![3, 3]];
    assert_eq!(achieved, expected);
}

#[test]
fn test_get_determinant() {
    let mat1 = vec![1, 3, 2, 5, 4, 6, 8, 9, 7];
    let mat2 = vec![1, 0, 2, -1, -1, 1, 2, 1, 2];
    assert_eq!(get_determinant(&mat1), 39);
    assert_eq!(get_determinant(&mat2), -1);
}
