use std::io::Read;
pub mod topology;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn get_mobile_nodes() -> ()/*std::vec::Vec<>*/ {
    let mut res = reqwest::blocking::get("http://127.0.0.1:8081/v1/nes/location/allMobile").unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

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
