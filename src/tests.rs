#[cfg(test)]
mod tests {
    use crate::*;
    use std::sync::{LazyLock, Mutex, MutexGuard};
    use std::time::Instant;

    static DB_PATH: &str = "./teststorage";
    static STORAGE: LazyLock<Mutex<Storage>> = LazyLock::new(|| Storage::new(DB_PATH).into());

    #[test]
    fn test_rw_storage() {
        // this should be okay even if unsafe since we are running a test
        let db = &*STORAGE.lock().unwrap();

        let test_string = String::from("haiiiiiii :3!!!");
        let encoded = encoding::encode(&test_string).unwrap();
        let hash = db.write(&encoded).unwrap();

        let result = db.get(hash).unwrap().unwrap();
        let string = std::str::from_utf8(&result).unwrap();
        assert_eq!(&encoded, string);
    }

    #[test]
    fn test_speed() {
        // this should be okay even if unsafe since we are running a test
        let db = &*STORAGE.lock().unwrap();

        let onemb = std::fs::read_to_string("1mb.txt").unwrap();
        const TESTSIZE: f32 = 52_428_800.0;

        // writing
        let write_now = Instant::now();
        let mut hashes = Vec::with_capacity(50);
        for _ in 0..50 {
            let encoded = encoding::encode(&onemb).unwrap();
            let hash = db.write(&encoded).unwrap();
            hashes.push(hash);
        }
        let write_time = write_now.elapsed().as_millis();

        // reading
        let read_now = Instant::now();
        for h in hashes {
            let _ = db.get(h).unwrap();
        }
        let read_time = read_now.elapsed().as_millis();

        let rw_speed = ((TESTSIZE / read_time as f32) + (TESTSIZE / write_time as f32)) / 2.0;

        println!(
            "WRITE TIME: {}ms\nREAD TIME: {}ms\n---\nAVG READ/WRITE SPEED: {} bytes/ms",
            write_time, read_time, rw_speed
        );
    }
}
