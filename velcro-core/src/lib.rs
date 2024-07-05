#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::from_over_into)]


pub mod math;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work_city_hash32() {
        let bytes = "bors".as_bytes();
        let hash32v: u32 = math::cityhash::city_hash32(bytes, bytes.len());
        println!("city hash 32 test: {}", hash32v);
    }

    #[test]
    fn it_work_city_hash64() {
        let bytes = "bors12233fdfdfdfd".as_bytes();
        let hash64v: u64 = math::cityhash::city_hash64(bytes, bytes.len());
        println!("city hash 64 test: {}", hash64v);
    }
}