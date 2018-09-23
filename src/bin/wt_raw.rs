extern crate wt_rs;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::time::SystemTime;

use wt_rs::wt_raw;

fn main() {
    println!("Hello, World!");
    unsafe {
        let location = CString::new("data").expect("data");
        let action = CString::new("create").expect("create");

        let mut conn: *mut wt_raw::WT_CONNECTION = std::mem::zeroed();
        wt_raw::wiredtiger_open(
            location.as_ptr(),
            std::ptr::null_mut(),
            action.as_ptr(),
            &mut conn,
        );

        let mut session: *mut wt_raw::WT_SESSION = std::mem::zeroed();
        let open_session = (*conn).open_session.expect("open_session");
        open_session(conn, std::ptr::null_mut(), std::ptr::null(), &mut session);

        let create = (*session).create.expect("create");
        let name = CString::new("table:access").expect("name");
        let config = CString::new("key_format=S,value_format=S").expect("config");
        create(session, name.as_ptr(), config.as_ptr());

        let mut cursor: *mut wt_raw::WT_CURSOR = std::mem::zeroed();
        let open_cursor = (*session).open_cursor.expect("open_cursor");
        open_cursor(
            session,
            name.as_ptr(),
            std::ptr::null_mut(),
            std::ptr::null(),
            &mut cursor,
        );

        let key = CString::new(format!(
            "rpb-{}",
            SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs()
        )).expect("key");
        let set_key = (*cursor).set_key.expect("set_key");
        set_key(cursor, key.as_ptr());

        let value = CString::new("foo bar baz").expect("value");
        let set_value = (*cursor).set_value.expect("set_value");
        set_value(cursor, value.as_ptr());

        let insert = (*cursor).insert.expect("insert");
        insert(cursor);

        let reset = (*cursor).reset.expect("reset"); /* Restart the scan. */
        reset(cursor);

        loop {
            let next = (*cursor).next.expect("next");
            let ret = next(cursor);
            if ret != 0 {
                break;
            }

            let get_key = (*cursor).get_key.expect("get_key");
            let mut k: *mut i8 = std::mem::zeroed();
            get_key(cursor, &mut k);

            let get_value = (*cursor).get_value.expect("get_value");
            let mut v: *mut i8 = std::mem::zeroed();
            get_value(cursor, &mut v);

            let kstr = std::str::from_utf8(CStr::from_ptr(k as *const c_char).to_bytes())
                .expect("utf8-key");
            let vstr = std::str::from_utf8(CStr::from_ptr(v as *const c_char).to_bytes())
                .expect("utf8-value");
            println!("({:?}, {:?})", kstr, vstr);
        }
        let close = (*conn).close.expect("close");
        close(conn, std::ptr::null());
    };
    println!("done");
}
