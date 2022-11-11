// #![allow(dead_code,unused_imports)]

fn main() {
    unsafe fn my_reshape(arr_ref: &[u32; 4])-> &[[u32; 2]; 2]{
        std::mem::transmute(arr_ref)
    }

    let arr= [1,2,3,4];
    let arr_ref= &arr;
    let reshaped_arr_ref= unsafe{ my_reshape(arr_ref) };

    assert_eq!( reshaped_arr_ref, &[[1,2],[3,4]] );
}
