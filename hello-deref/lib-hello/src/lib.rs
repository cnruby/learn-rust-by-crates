pub mod mod_string {
    pub fn get_string(slice: &str) -> String {
        // convert &str to String
        let y: String = String::from(slice);
        println!("{} from get_string()", y);
        y
    }

    pub fn get_slice_str(slice: &str) -> &str {
        // Deref coercion
        slice
    }
}

pub mod mod_array {
    pub fn get_array(slice: &[u8]) -> [u8; 3] {
        // convert slice to array
        [slice[0], slice[1], slice[2]]
    }

    pub fn get_slice_array(slice: &[u8]) -> &[u8] {
        // Deref coercion
        for &item in slice.iter() {
            println!("{} from get_slice_array(:?)", item);
        }
        slice
    }
}

pub mod mod_vec {
    pub fn get_vec(slice: &[u8]) -> Vec<u8> {
        // convert slice to vec!
        vec![slice[0], slice[1], slice[2]]
    }

    pub fn get_slice_vec(slice: &[u8]) -> &[u8] {
        // Deref coercion
        for &item in slice.iter() {
            println!("{} from get_slice_vec(:?)", item);
        }
        slice
    }
}

pub mod mod_box {
    pub fn get_box(x: u8) -> Box<u8> {
        // convert u8 to Box
        Box::new(x)
    }

    pub fn get_ref_box(x: &u8) -> &u8 {
        // Deref coercion
        x
    }

    pub fn get_u8(x: &u8) -> u8 {
        // Deref coercion
        *x
    }
}
