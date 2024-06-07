pub mod bindings;

use bindings::apply_patch;

use crate::bindings::HStreamOutput;

fn main() {
    let mut out_new_data = vec![0; 100]; // Initialize with appropriate size
    let old_data = vec![1, 2, 3];
    let serialized_diff = vec![/* serialized diff data */];

    let success = apply_patch(&mut out_new_data, &old_data, &serialized_diff);

    if success {
        println!("Patch applied successfully");
    } else {
        println!("Patch failed");
    }
}
