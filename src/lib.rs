/// All the raw bindgen output.
pub mod wt_raw {
    #![allow(dead_code)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod wt {
    use super::wt_raw;
    use std::ffi::{CStr, CString};
    use std::mem;
    use std::os::raw::c_char;
    use std::ptr;

    pub struct Connection {
        raw: *mut wt_raw::WT_CONNECTION,
    }

    pub struct Session {
        raw: *mut wt_raw::WT_SESSION,
    }

    pub struct Cursor {
        raw: *mut wt_raw::WT_CURSOR,
    }

    impl Connection {
        pub fn open(home: &str) -> Result<Connection, String> {
            let config = CString::new("create").expect("make CString");
            let home = CString::new(home).expect("make CString");
            unsafe {
                let mut raw: *mut wt_raw::WT_CONNECTION = mem::uninitialized();
                wt_raw::wiredtiger_open(
                    home.as_ptr(),
                    /* event_handler = */ ptr::null_mut(),
                    config.as_ptr(),
                    &mut raw,
                );
                Ok(Connection { raw })
            }
        }
    }

    impl Drop for Connection {
        fn drop(&mut self) {
            unsafe {
                let close = (*self.raw).close.expect("connection.close method");
                close(self.raw, /* config = */ ptr::null_mut());
            }
        }
    }

    impl Session {
        pub fn open(conn: &mut Connection) -> Result<Session, String> {
            unsafe {
                let mut raw: *mut wt_raw::WT_SESSION = mem::uninitialized();
                let open_session = (*conn.raw).open_session.expect("connection.open_session");
                open_session(
                    conn.raw,
                    /* event_handler = */ ptr::null_mut(),
                    /* config = */ ptr::null(),
                    &mut raw,
                );
                Ok(Session { raw })
            }
        }

        pub fn create_table(&mut self, table_name: &str) -> Result<(), String> {
            unsafe {
                let create = (*self.raw).create.expect("session.create");
                let name = CString::new(format!("table:{}", table_name)).expect("make CString");
                let config = CString::new("key_format=S,value_format=S").expect("make CString");
                create(self.raw, name.as_ptr(), config.as_ptr());
                Ok(())
            }
        }
    }

    impl Drop for Session {
        fn drop(&mut self) {
            unsafe {
                let close = (*self.raw).close.expect("session.close");
                close(self.raw, /* config = */ ptr::null_mut());
            }
        }
    }

    impl Cursor {
        pub fn open(session: &mut Session, table_name: &str) -> Result<Cursor, String> {
            unsafe {
                let mut raw: *mut wt_raw::WT_CURSOR = mem::uninitialized();
                let open_cursor = (*session.raw).open_cursor.expect("session.open_cursor");
                let name = CString::new(format!("table:{}", table_name)).expect("make CString");
                open_cursor(
                    session.raw,
                    name.as_ptr(),
                    /* to_dup = */ ptr::null_mut(),
                    /* config = */ ptr::null(),
                    &mut raw,
                );
                Ok(Cursor { raw })
            }
        }

        pub fn put(&mut self, key: &str, value: &str) -> Result<(), String> {
            unsafe {
                let key = CString::new(key).expect("make CString");
                let set_key = (*self.raw).set_key.expect("cursor.set_key");
                set_key(self.raw, key.as_ptr());

                let value = CString::new(value).expect("make CString");
                let set_value = (*self.raw).set_value.expect("cursor.set_value");
                set_value(self.raw, value.as_ptr());

                let insert = (*self.raw).insert.expect("cursor.insert");
                insert(self.raw);
                Ok(())
            }
        }

        pub fn reset(&mut self) -> Result<(), String> {
            unsafe {
                let reset = (*self.raw).reset.expect("cursor.reset");
                reset(self.raw);
                Ok(())
            }
        }

        pub fn advance(&mut self) -> Result<bool, String> {
            unsafe {
                let next = (*self.raw).next.expect("cursor.next");
                Ok(next(self.raw) == 0)
            }
        }

        pub fn get(&mut self) -> Result<(String, String), String> {
            unsafe {
                let get_key = (*self.raw).get_key.expect("cursor.get_key");
                let mut k: *mut i8 = mem::uninitialized();
                get_key(self.raw, &mut k);

                let get_value = (*self.raw).get_value.expect("cursor.get_value");
                let mut v: *mut i8 = mem::uninitialized();
                get_value(self.raw, &mut v);

                let kstr =
                    String::from_utf8(CStr::from_ptr(k as *const c_char).to_bytes().to_vec())
                        .expect("utf8-key");
                let vstr =
                    String::from_utf8(CStr::from_ptr(v as *const c_char).to_bytes().to_vec())
                        .expect("utf8-value");
                Ok((kstr, vstr))
            }
        }
    }

    impl Drop for Cursor {
        fn drop(&mut self) {
            unsafe {
                let close = (*self.raw).close.expect("cursor.close");
                close(self.raw);
            }
        }
    }
}
