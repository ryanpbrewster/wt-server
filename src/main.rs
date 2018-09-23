extern crate wt_rs;

use wt_rs::wt_raw;

fn main() {
    println!("Hello, World!");
    let c = unsafe {
        let mut conn: wt_raw::WT_CONNECTION = std::mem::zeroed();
        let mut tmp: *mut wt_raw::__wt_connection = &mut conn as *mut wt_raw::__wt_connection;
        let mut handler: wt_raw::__wt_event_handler = std::mem::zeroed();
        wt_raw::wiredtiger_open(
            "data".as_ptr() as *const i8,
            &mut handler,
            "create\0".as_ptr() as *const i8,
            &mut tmp as *mut *mut wt_raw::__wt_connection,
        )
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
