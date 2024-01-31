use rand::{rngs::ThreadRng, Rng};
use std::time::{Duration, Instant};


fn main() {
    let mut rng: ThreadRng = rand::thread_rng();

    let mut grades_bubble: Vec<i32> = (0..1000).map(|_| rng.gen_range(0..=100)).collect();
    let mut grades_merge: Vec<i32> = grades_bubble.clone();
    let mut grades_selection: Vec<i32> = grades_bubble.clone();

    // Bubble sort
    let start_bubble: Instant = Instant::now();
    bubble_sort(&mut grades_bubble);
    let duration_bubble: Duration = start_bubble.elapsed();

    println!("Bubble sort: {:?} microseconds", duration_bubble.as_micros());
    println!("Lista de notas despues de Bubble Sort: {:?}", grades_bubble);

    // Merge sort
    let start_merge: Instant = Instant::now();
    merge_sort(&mut grades_merge);
    let duration_merge: Duration = start_merge.elapsed();
    println!("Merge Sort: {:?} microseconds", duration_merge.as_micros());
    println!("Lista de notas después de Merge Sort: {:?}", grades_merge);

    // Selection Sort
    let start_selection: Instant = Instant::now();
    selection_sort(&mut grades_selection);
    let duration_selection: Duration = start_selection.elapsed();

    println!("Selection Sort: {:?} microseconds", duration_selection.as_micros());
    println!("Lista de notas después de Selection Sort: {:?}", grades_selection);
    
}

fn bubble_sort(arr: &mut Vec<i32>) {
    let n: usize = arr.len();

    for i in 0..n {
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                // make the swap
                let temp: i32 = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}

fn merge_sort(arr: &mut Vec<i32>){
    let len:usize = arr.len();

    if len <= 1 {
        return;
    }
    
    let mid = len / 2;

    let mut left: Vec<i32> = arr[0..mid].to_vec();
    let mut right: Vec<i32> = arr[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);
    merge(arr, &left, &right);
    
}

fn merge(arr: &mut Vec<i32>, left: &Vec<i32>, right: &Vec<i32>){
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j]{
            arr[k] = left[i];
            i += 1;

        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len(){
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len(){
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn selection_sort(arr: &mut Vec<i32>){
    let len = arr.len();

    for i in 0..len{
        let mut min_index = i;

        for j in i + 1..len{
            if arr[j] <  arr[min_index]{
                min_index = j;
            }
        }

        arr.swap(i, min_index);
    }
}