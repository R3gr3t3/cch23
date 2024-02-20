use axum::extract::Path;

// pub async fn task1_handler(Path((num1, num2)): Path<(i32, i32)>) -> String {
//     format!("{}", (num1 ^ num2).pow(3))
// }

pub async fn task2_handler(Path(path): Path<String>) -> String {
    let nums: Vec<&str> = path.split("/").collect();
    let mut result = 0;

    for num in nums {
        result ^= num.parse::<i32>().unwrap();
    }

    format!("{}", result.pow(3))
}