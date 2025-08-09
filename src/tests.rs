#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_rw_storage() {
        let test_string = String::from("haiiiiiii :3!!!");
        let encoded = encoding::encode(&test_string).unwrap();
        let hash = storage::write(&encoded).unwrap();

        let hm = storage::read_into_hashmap().unwrap();
        let result = hm.get(&hash).unwrap();
        assert_eq!(encoded, *result);
    }
}