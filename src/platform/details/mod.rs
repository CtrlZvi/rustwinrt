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

pub fn get_command_arguments(argc : ::libc::c_int) -> (*const *const ::libc::wchar_t, ::libc::c_int) {
    let mut argc = argc;
    ( unsafe { Platform::Details::GetCmdArguments(&mut argc) }, argc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_arguments() {
        let (argv, argc) = get_command_arguments(0);
        assert!(argc == 1);
    }
}