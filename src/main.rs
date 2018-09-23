extern crate wt_rs;

use std::ffi::CString;

use wt_rs::wt_raw;

fn main() {
    println!("Hello, World!");
    unsafe {
        let location = CString::new("data").unwrap();
        let action = CString::new("create").unwrap();

        let mut conn: wt_raw::WT_CONNECTION = std::mem::zeroed();
        wt_raw::wiredtiger_open(
            location.as_ptr() as *const i8,
            std::ptr::null_mut(),
            action.as_ptr() as *const i8,
            &mut (&mut conn as *mut wt_raw::WT_CONNECTION),
        );

        let mut session: wt_raw::WT_SESSION = std::mem::zeroed();
        let open_session = conn.open_session.unwrap();
        open_session(
            &mut conn,
            std::ptr::null_mut(),
            std::ptr::null(),
            &mut (&mut session as *mut wt_raw::WT_SESSION),
        );

        let create = session.create.unwrap();
        let name = CString::new("table:access").unwrap();
        let config = CString::new("key_format=S,value_format=S").unwrap();
        create(&mut session, name.as_ptr() as *const i8, config.as_ptr()) as *const i8;

        let mut cursor: wt_raw::WT_CURSOR = std::mem::zeroed();
        let open_cursor = session.open_cursor.unwrap();
        open_cursor(
            &mut session,
            name.as_ptr() as *const i8,
            std::ptr::null_mut(),
            std::ptr::null(),
            &mut (&mut cursor as *mut wt_raw::WT_CURSOR),
        );
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
