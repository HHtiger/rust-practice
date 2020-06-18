use std::ffi::CString;
use std::os::raw::{c_float, c_int};

#[repr(C)]
#[derive(Debug)]
pub struct CStudent {
    pub num: c_int,
    pub total: c_int,
    pub name: [u8; 20],
    pub scores: [c_float; 3],
}

impl CStudent {
    // constructor with parameters
    pub fn new(num: i32, total: i32, name: String, scores: Vec<f32>) -> CStudent {
        let c_string = CString::new(name).expect("CString::new failed");
        let bytes = c_string.as_bytes_with_nul();
        
        let mut c_name: [u8; 20] = [0u8; 20];
        c_name[..bytes.len()].copy_from_slice(bytes);

        let mut c_scores: [f32; 3] = [0f32; 3];
        for (index, score) in scores.iter().enumerate() {
            if index > 2 {
                break;
            } else {
                c_scores[index] = *score as c_float;
            }
        }

        CStudent {
            num,
            total,
            name: c_name,
            scores: c_scores,
        }
    }
}

// Default constructor
impl Default for CStudent {
    fn default() -> Self {
        CStudent {
            num: 0 as c_int,
            total: 0 as c_int,
            name: [0u8; 20],
            scores: [0.0 as c_float; 3],            
        }
    }
}

#[link(name = "cfoo")]
extern "C" {
    fn print_data(p_stu: *mut CStudent);
    fn fill_data(p_stu: *mut CStudent);
}


fn main() {
    // Initialization of allocated memory
    let new_stu: CStudent = Default::default();
    println!("rust side print new_stu: {:?}", new_stu);
    let box_new_stu = Box::new(new_stu);
    let p_stu = Box::into_raw(box_new_stu);

    unsafe {
        fill_data(p_stu);
        print_data(p_stu);
        println!("rust side print Bob: {:?}", Box::from_raw(p_stu));
        
    }
}