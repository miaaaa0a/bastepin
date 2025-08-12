#[cfg(test)]
mod tests {
    use crate::*;
    use std::time::Instant;

    #[test]
    fn test_rw_storage() {
        // this should be okay even if unsafe since we are running a test
        unsafe {
            std::env::set_var("STORAGE_PATH", "./teststorage");
        }

        let test_string = String::from("haiiiiiii :3!!!");
        let encoded = encoding::encode(&test_string).unwrap();
        let hash = storage::write(&encoded).unwrap();

        let hm = storage::read_into_hashmap().unwrap();
        let result = hm.get(&hash).unwrap().unwrap();
        let string = std::str::from_utf8(&result).unwrap();
        assert_eq!(&encoded, string);
    }

    #[test]
    fn test_speed() {
        // this should be okay even if unsafe since we are running a test
        unsafe {
            std::env::set_var("STORAGE_PATH", "./teststorage");
        }

        let onemb = std::fs::read_to_string("1mb.txt").unwrap();
        const TESTSIZE: f32 = 52_428_800.0;

        // writing
        let write_now = Instant::now();
        let mut hashes = Vec::with_capacity(50);
        for _ in 0..50 {
            let encoded = encoding::encode(&onemb).unwrap();
            let hash = storage::write(&encoded).unwrap();
            hashes.push(hash);
        }
        let write_time = write_now.elapsed().as_millis();

        // reading
        let read_now = Instant::now();
        let hm = storage::read_into_hashmap().unwrap();
        for h in hashes {
            let _ = hm.get(&h).unwrap();
        }
        let read_time = read_now.elapsed().as_millis();

        let rw_speed = ((TESTSIZE / read_time as f32) + (TESTSIZE / write_time as f32)) / 2.0;

        println!(
            "WRITE TIME: {}ms\nREAD TIME: {}ms\n---\nAVG READ/WRITE SPEED: {} bytes/ms",
            write_time, read_time, rw_speed
        );
        std::fs::remove_file("./teststorage").unwrap();
    }
}
