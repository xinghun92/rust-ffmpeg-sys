#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[macro_use]
mod avutil;
pub use avutil::*;

#[cfg(features = "runnable")]
extern "C" {
    pub fn run(
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_run() {
        use std::str::FromStr;
        use std::os::raw::c_char;

        let args = vec!["ffmpeg", "-i", "/Users/wangsijie/Downloads/New_IMG_0431.MOV", "-vframes", "1", "-f", "image2", "test_image.jpg"];
        let argc = args.len() as ::std::os::raw::c_int;

        let mut c_args = Vec::new();
        let mut c_args_hold = Vec::new();
        for arg in args {
            let mut c_chars = arg.to_string().into_bytes();
            c_chars.push('\0' as u8);

            println!("push: {:?}", String::from_utf8_lossy(&c_chars));
            let mut c_chars: Vec<_> = c_chars.into_iter().map(|b| b as c_char).collect();
            c_args.push(c_chars.as_mut_ptr());
            c_args_hold.push(c_chars);
        }

        unsafe {
            ::run(argc, c_args.as_mut_ptr());
        }
    }

    #[test]
    fn test_init() {
        unsafe {
            ::avfilter_register_all();
        }
    }
}