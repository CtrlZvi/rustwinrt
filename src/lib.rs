#![feature(core, libc, os, unicode)]
extern crate libc;
extern crate unicode;

use std::mem;
use std::num::ToPrimitive;
use std::os;

#[cfg(target_family = "windows")]
#[link(name = "vccorlib120")]
#[allow(non_snake_case)]
pub mod Platform {
    pub mod Details {
        extern "system" {
            #[link_name = "?GetCmdArguments@Details@Platform@@YAPAPA_WPAH@Z"]
            pub fn GetCmdArguments(argc : *const ::libc::c_int) -> *const *const ::libc::wchar_t;
        }
    }
}

pub mod platform {
    pub mod details {
        #[unstable]
        pub fn get_command_arguments(argc : ::libc::c_int) -> *const *const ::libc::wchar_t {
            unsafe { ::Platform::Details::GetCmdArguments(&argc) }
        }
    }
}