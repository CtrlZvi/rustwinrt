// For reference on linking to C++ functions, see https://github.com/rust-lang/rust/issues/5853#issuecomment-44365672

#[cfg(target_family = "windows")]
#[allow(non_snake_case)]
mod Platform {
    pub mod Details {
        #[link(name = "vccorlib")]
        #[cfg(target_arch="x86_64")]
        extern "C" {
            // This generates invalid program output causing a crash when this function is called. See https://github.com/rust-lang/rust/issues/23216 for details.
            #[link_name = "?GetCmdArguments@Details@Platform@@YAPEAPEA_WPEAH@Z"]
            pub fn GetCmdArguments(argc : *mut ::libc::c_int) -> *const *const ::libc::wchar_t;
        }

        #[link(name = "vccorlib")]
        #[cfg(target_arch = "x86")]
        extern "C" {
            // This generates invalid program output causing a crash when this function is called. See https://github.com/rust-lang/rust/issues/23216 for details.
            #[link_name = "?GetCmdArguments@Details@Platform@@YAPAPA_WPAH@Z"]
            pub fn GetCmdArguments(argc : *mut ::libc::c_int) -> *const *const ::libc::wchar_t;
        }
    }
}

pub fn get_command_arguments(argc : ::libc::c_int) -> Vec<String> {
    let mut argc = argc;
    let argv = unsafe { Platform::Details::GetCmdArguments(&mut argc) };
    let mut arguments : Vec<String> = Vec::with_capacity(argc as usize);
    for arg in unsafe { ::std::slice::from_raw_parts(argv, argc as usize) }.into_iter() {
        let argument : &[::libc::wchar_t] = unsafe { ::std::slice::from_raw_parts(*arg, ::libc::wcslen(*arg) as usize) };
        arguments.push(String::from_utf16(unsafe { ::std::mem::transmute(argument) } ).unwrap())
    }
    arguments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_arguments() {
        let arguments = get_command_arguments(0);
        assert!(arguments.len() == 1);
        for argv in arguments {
            println!("{}", argv)
        }
    }
}