#[test]
fn test_init_uninit() {
    use crate::{aws_common_library_clean_up, aws_common_library_init, aws_default_allocator};

    unsafe {
        aws_common_library_init(aws_default_allocator());
        aws_common_library_clean_up();
    }
}
