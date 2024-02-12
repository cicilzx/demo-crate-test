mod url_process;

use rand::Rng;


pub fn add(left: usize, right: usize) -> usize {
    let re = left + right;
    re
}

pub fn get_url_info(url_str: &str) {
    url_process::process_url(url_str)
}


pub fn get_random() {
    let mut rng = rand::thread_rng(); // 获取一个随机数生成器

    // 生成一个随机的u32整数
    let random_int: u32 = rng.gen();
    println!("Random u32: {}", random_int);

    // 生成一个0到1之间的随机浮点数
    let random_float: f64 = rng.gen();
    println!("Random f64: {}", random_float);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

}
