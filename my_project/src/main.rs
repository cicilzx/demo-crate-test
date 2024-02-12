use my_lib::add;
use my_lib::process_url;
use my_lib::get_random;

fn main() {
    let re = add(1,1);
    println!("{}", re);
    let url_str = "https://www.example.com/path/to/resource?query=param#fragment";
    process_url(url_str);
    get_random();
}
