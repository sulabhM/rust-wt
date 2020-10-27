#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(c_variadic)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]

// Without this directive, we get warnings, even though we are using the macros.
#[allow(unused_macros)]
// Like a C null pointer
macro_rules! WTNULL {
    () => {
        std::ptr::null_mut();
    };
}

#[allow(unused_macros)]
// Convert a returned pointer (to a struct) to a reference (to a struct).
// Naming it WTREF might be confusing.
macro_rules! WTPTR {
    ($ptr:expr) => {
        /*unsafe*/ { &mut *($ptr) }
    };
}

#[allow(unused_macros)]
// Return a string for use in API.  Note that we should not
// return .as_ptr(), because then the underlying CString will
// go out of scope and the string's memory will go away.
macro_rules! WTSTRING {
    ($str:expr) => {
        CString::new($str).expect("CString::new failed");
    };
}

#[allow(unused_macros)]
macro_rules! WTFUN {
    ($v:ident, $f:expr) => {
          let tmpf = $f;
          let $v = match tmpf {
             Some(tmpf) => tmpf,
             None => panic!("unexpected null pointer"),
          };
    };
}

pub fn setup_home(home: &str) {
   /* Remove existing test directory and create a new one */
   if std::path::Path::new(home).is_dir() {
       std::fs::remove_dir_all(home).expect("Removal of test dir failed");
   }
   std::fs::create_dir(home).expect("Creation of test dir failed");
}

impl __wt_connection {
    pub unsafe fn close(&mut self,
            config: *const ::std::os::raw::c_char
        ) -> ::std::os::raw::c_int {
        return (self.close.unwrap()) (&mut *self, config);
    }
    pub unsafe fn open_session(&mut self,
            event_handler: *mut WT_EVENT_HANDLER,
            config: *const ::std::os::raw::c_char,
            sessionp: *mut *mut WT_SESSION,
        ) -> ::std::os::raw::c_int {
        return (self.open_session.unwrap()) (&mut *self, event_handler, config, sessionp);
    }
}

impl __wt_session {
    pub unsafe fn close(&mut self,
            config: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int {
        return (self.close.unwrap()) (&mut *self, config);
    }
    pub unsafe fn open_cursor(&mut self,
            uri: *const ::std::os::raw::c_char,
            to_dup: *mut WT_CURSOR,
            config: *const ::std::os::raw::c_char,
            cursorp: *mut *mut WT_CURSOR,
        ) -> ::std::os::raw::c_int {
        return (self.open_cursor.unwrap()) (&mut *self, uri, to_dup, config, cursorp);
    }
    pub unsafe fn create(&mut self,
            name: *const ::std::os::raw::c_char,
            config: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int {
        return (self.create.unwrap()) (&mut *self, name, config);
    }
}

impl __wt_cursor {
    pub unsafe extern "C" fn get_key(&mut self, mut args: ...) -> ::std::os::raw::c_int {
        use std::ffi::VaListImpl;
        let mut ap: VaListImpl;
        let mut aq: VaListImpl;
        ap = args.clone();
        aq = ap.clone();
        return (self.get_key.unwrap()) (&mut *self, aq.as_va_list());
    }
    pub unsafe extern "C" fn get_value(&mut self, mut args: ...) -> ::std::os::raw::c_int {
        use std::ffi::VaListImpl;
        let mut ap: VaListImpl;
        let mut aq: VaListImpl;
        ap = args.clone();
        aq = ap.clone();
        return (self.get_value.unwrap()) (&mut *self, aq.as_va_list());
    }
    pub unsafe extern "C" fn set_key(&mut self, mut args: ...) {
        use std::ffi::VaListImpl;
        let mut ap: VaListImpl;
        let mut aq: VaListImpl;
        ap = args.clone();
        aq = ap.clone();
        return (self.set_key.unwrap()) (&mut *self, aq.as_va_list());
    }
    pub unsafe extern "C" fn set_value(&mut self, mut args: ...) {
        use std::ffi::VaListImpl;
        let mut ap: VaListImpl;
        let mut aq: VaListImpl;
        ap = args.clone();
        aq = ap.clone();
        return (self.set_value.unwrap()) (&mut *self, aq.as_va_list());
    }
    pub unsafe fn next(&mut self) -> ::std::os::raw::c_int {
        return (self.next.unwrap()) (&mut *self);
    }
    pub unsafe fn prev(&mut self) -> ::std::os::raw::c_int {
        return (self.prev.unwrap()) (&mut *self);
    }
    pub unsafe fn reset(&mut self) -> ::std::os::raw::c_int {
        return (self.reset.unwrap()) (&mut *self);
    }
    pub unsafe fn search(&mut self) -> ::std::os::raw::c_int {
        return (self.search.unwrap()) (&mut *self);
    }
    pub unsafe fn search_near(&mut self, exactp: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int {
        return (self.search_near.unwrap()) (&mut *self, exactp);
    }
    pub unsafe fn insert(&mut self) -> ::std::os::raw::c_int {
        return (self.insert.unwrap()) (&mut *self);
    }
    pub unsafe fn update(&mut self) -> ::std::os::raw::c_int {
        return (self.update.unwrap()) (&mut *self);
    }
    pub unsafe fn remove(&mut self) -> ::std::os::raw::c_int {
        return (self.remove.unwrap()) (&mut *self);
    }
    pub unsafe fn close(&mut self) -> ::std::os::raw::c_int {
        return (self.close.unwrap()) (&mut *self);
    }
}

mod tests {
    #[test]
    fn wt_open() {
        use super::*;
        use std::ffi::CString;

        const WT_HOME: &str = "./WT_TEST";
        setup_home(WT_HOME);

        unsafe {
            let mut conn_ret : *mut WT_CONNECTION = WTNULL!();
            let mut session_ret : *mut WT_SESSION = WTNULL!();
            let mut cursor_ret : *mut WT_CURSOR = WTNULL!();

            let home_c_str = WTSTRING!(WT_HOME);
            let config = WTSTRING!("create");
            let uri = WTSTRING!("table:store");
            let uri2 = WTSTRING!("table:store2");

            wiredtiger_open(home_c_str.as_ptr(), WTNULL!(), config.as_ptr(), &mut conn_ret);
            let conn = WTPTR!(conn_ret);

            conn.open_session(WTNULL!(), WTNULL!(), &mut session_ret);
            let session = WTPTR!(session_ret);
            let create_config = WTSTRING!("key_format=S,value_format=S");
            session.create(uri.as_ptr(), create_config.as_ptr());
            session.create(uri2.as_ptr(), create_config.as_ptr());

            session.open_cursor(uri.as_ptr(), WTNULL!(), WTNULL!(), &mut cursor_ret);
            let cursor = WTPTR!(cursor_ret);

            for i in 0..100 {
                let key = WTSTRING!(format!("KEY{}", i));
                let value = WTSTRING!(format!("VALUE{}", i));
                cursor.set_key(key.as_ptr());
                cursor.set_value(value.as_ptr());
                cursor.insert();
            }
            cursor.close();
            session.close(WTNULL!());
            conn.close(WTNULL!());
        }
    }
}
