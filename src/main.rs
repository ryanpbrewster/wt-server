extern crate wt_rs;

use std::ffi::CString;

use wt_rs::wt_raw;

fn main() {
    println!("Hello, World!");
    unsafe {
        let location = CString::new("data").expect("data");
        let action = CString::new("create").expect("create");

        let mut conn: *mut wt_raw::WT_CONNECTION = std::mem::zeroed();
        wt_raw::wiredtiger_open(
            location.as_ptr() as *const i8,
            std::ptr::null_mut(),
            action.as_ptr() as *const i8,
            &mut conn,
        );

        let mut session: *mut wt_raw::WT_SESSION = std::mem::zeroed();
        let open_session = (*conn).open_session.expect("open_session");
        open_session(
            conn,
            std::ptr::null_mut(),
            std::ptr::null(),
            &mut session,
        );

        let create = (*session).create.expect("create");
        let name = CString::new("table:access").expect("name");
        let config = CString::new("key_format=S,value_format=S").expect("config");
        create(session, name.as_ptr() as *const i8, config.as_ptr()) as *const i8;

        let mut cursor: *mut wt_raw::WT_CURSOR = std::mem::zeroed();
        let open_cursor = (*session).open_cursor.expect("open_cursor");
        open_cursor(
            session,
            name.as_ptr() as *const i8,
            std::ptr::null_mut(),
            std::ptr::null(),
            &mut cursor,
        );

        let key = CString::new("rpb").expect("key");
        let set_key = (*cursor).set_key.expect("set_key");
        set_key(cursor, key.as_ptr());

        let value = CString::new("foo bar baz").expect("value");
        let set_value = (*cursor).set_value.expect("set_value");
        set_value(cursor, value.as_ptr());

        let insert = (*cursor).insert.expect("insert");
        insert(cursor);
    };
    println!("done");
}

/*
#include <iostream>
#include "wiredtiger.h"

static const char *home;

static void
access_example(void)
{
        WT_CONNECTION *conn;
        WT_CURSOR *cursor;
        WT_SESSION *session;
        const char *key, *value;
        int ret;
        /* Open a connection to the database, creating it if necessary. */
        (wiredtiger_open(home, NULL, "create", &conn));
        /* Open a session handle for the database. */
        (conn->open_session(conn, NULL, NULL, &session));
        (session->create(
            session, "table:access", "key_format=S,value_format=S"));
        (session->open_cursor(
            session, "table:access", NULL, NULL, &cursor));

        for (int i=0; i < 100000; i++) {
          std::string k = "key-" + std::to_string(i);
          std::string v = "value-" + std::to_string(i * i);
          cursor->set_key(cursor, k.data());        /* Insert a record. */
          cursor->set_value(cursor, v.data());
          cursor->insert(cursor);
        }
        cursor->reset(cursor);     /* Restart the scan. */
        while ((ret = cursor->next(cursor)) == 0) {
                (cursor->get_key(cursor, &key));
                (cursor->get_value(cursor, &value));
                std::cout << "Got record: " << key << " = " << value << std::endl;
        }
        (conn->close(conn, NULL));   /* Close all handles. */
}
int
main(int argc, char *argv[])
{
        home = argv[1];
        printf("home = %s\n", home);
        access_example();
        return 0;
}
*/
