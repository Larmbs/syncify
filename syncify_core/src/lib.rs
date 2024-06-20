
mod decode;
mod encode;
mod convert;
mod traits;

mod diff; 

pub use decode::decode;
pub use encode::encode;

pub use convert::{struct_from_bytes, struct_to_bytes};
pub use diff::{calc_diff, calc_next_with_prev};
pub use traits::Syncable;

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{
        Serialize,
        Deserialize
    };

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Object {
        p1: String,
        p2: u16,
        p3: Option<i128>,
        p4: String,
        p5: Option<()>,
        p6: Vec<u8>,
    }

    #[test]
    fn test1() {
        let prev_obj = Object {
            p1: String::from("Hello World"),
            p2: 345,
            p3: None,
            p4: String::from("Another string is here and is large and needs to be compressed by a lot as there is no point in keeping it so large"),
            p5: None,
            p6: vec![1, 3, 4, 6, 7]

        };
        let new_obj = Object {
            p1: String::from("Hello World"),
            p2: 345,
            p3: None,
            p4: String::from("Another string is here and is large and needs to be compressed by a lot as there is no point in keeping it so large"),
            p5: Some(()),
            p6: vec![1, 3, 4, 6, 7]
        };

        // Sending part
        let prev_obj_bytes = convert::struct_to_bytes(&prev_obj).unwrap();
        let new_obj_bytes = convert::struct_to_bytes(&new_obj).unwrap();

        let diff = diff::calc_diff(&prev_obj_bytes, &new_obj_bytes);
        println!("Before: len({})", diff.len());

        let compressed = encode(&diff);
        println!("After: len({})", compressed.len());


        // Receiving
        let decompressed = decode(&compressed);
        let rx_new_bytes = diff::calc_next_with_prev(&prev_obj_bytes, &decompressed);
        let rx_new_obj: Object = convert::struct_from_bytes(&rx_new_bytes).unwrap();

        assert_eq!(new_obj, rx_new_obj);
        println!("{:?}", rx_new_obj);
    }
}


