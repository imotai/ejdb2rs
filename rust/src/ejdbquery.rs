extern crate ejdb_sys;


pub trait EJDBSerializable<T> {
    fn from_jbl(jbl: ejdb_sys::JBL) -> Result<T, ejdb_sys::iwrc> ;

    fn to_jbl(&self) -> Result<ejdb_sys::JBL, ejdb_sys::iwrc>;
}

pub struct EJDBQuery {
    pub(crate) q: ejdb_sys::JQL,
    query: String,
    collection: String,
}

pub trait SetPlaceholder<T> {
    fn set_placeholder(&self, placeholder: &str, index: i32, val: T) -> ejdb_sys::iwrc;
}

impl EJDBQuery {
    pub fn new(collection: &str, query: &str) -> EJDBQuery {
        EJDBQuery {
            q: std::ptr::null_mut(),
            query: String::from(query),
            collection: String::from(collection),
        }
    }

    pub fn init(&mut self) -> ejdb_sys::iwrc {
        let collection_str = std::ffi::CString::new(self.collection.as_str()).unwrap();
        let query_str = std::ffi::CString::new(self.query.as_str()).unwrap();
        let rc = unsafe {
            ejdb_sys::jql_create(&mut self.q, collection_str.as_ptr(), query_str.as_ptr())
        };

        if rc != 0 {
            unsafe { ejdb_sys::jql_destroy(&mut self.q) };
            self.q = std::ptr::null_mut();
        }
        return rc;
    }

    pub fn limit(&self) -> Result<i64, ejdb_sys::iwrc> {
        let mut out: i64 = 0;
        let rc = unsafe { ejdb_sys::jql_get_limit(self.q, &mut out) };

        if rc != 0 {
            return Err(rc);
        }

        return Ok(out);
    }

    pub fn set_placeholder_json(&self, placeholder: &str, index: i32, val: &str) -> ejdb_sys::iwrc {
        let mut jbl: ejdb_sys::JBL = std::ptr::null_mut();
        let value_str = std::ffi::CString::new(val).unwrap();

        let rc = unsafe { ejdb_sys::jbl_from_json(&mut jbl, value_str.as_ptr()) };
        if rc != 0 {
            println!("can't convert str to json {}", rc);
            unsafe { ejdb_sys::jbl_destroy(&mut jbl) };
            return rc;
        }
        let placeholder_str = std::ffi::CString::new(placeholder).unwrap();

        let rc2 =
            unsafe { ejdb_sys::jql_set_json_jbl(self.q, placeholder_str.as_ptr(), index, jbl) };
        if rc2 != 0 {
            println!("failed to set placeholder {} to json {}", placeholder, rc);
        }
        unsafe { ejdb_sys::jbl_destroy(&mut jbl) };
        rc2
    }

    pub fn setRegexp(&self, placeholder: &str, index: i32, regexp: &str) -> ejdb_sys::iwrc {
        let placeholder_str = std::ffi::CString::new(placeholder).unwrap();
        let regexp_str = std::ffi::CString::new(regexp).unwrap();

        let rc = unsafe {
            ejdb_sys::jql_set_regexp(self.q, placeholder_str.as_ptr(), index, regexp_str.as_ptr())
        };

        if rc != 0 {
            println!("failed to set placeholder {} to regexp {}", placeholder, rc);
        }
        rc
    }

    pub fn setNull(&self, placeholder: &str, index: i32) -> ejdb_sys::iwrc {
        let placeholder_str = std::ffi::CString::new(placeholder).unwrap();

        let rc = unsafe { ejdb_sys::jql_set_null(self.q, placeholder_str.as_ptr(), index) };
        if rc != 0 {
            println!("failed to set placeholder {} to null {}", placeholder, rc);
        }
        rc
    }
}

impl SetPlaceholder<i64> for EJDBQuery {
    fn set_placeholder(&self, placeholder: &str, index: i32, val: i64) -> ejdb_sys::iwrc {
        let placeholder_str = std::ffi::CString::new(placeholder).unwrap();

        let rc = unsafe { ejdb_sys::jql_set_i64(self.q, placeholder_str.as_ptr(), index, val) };

        if rc != 0 {
            println!("failed to set placeholder {}", placeholder);
        }
        rc
    }
}

impl SetPlaceholder<f64> for EJDBQuery {
    fn set_placeholder(&self, placeholder: &str, index: i32, val: f64) -> ejdb_sys::iwrc {
        let placeholder_str = std::ffi::CString::new(placeholder).unwrap();

        let rc = unsafe { ejdb_sys::jql_set_f64(self.q, placeholder_str.as_ptr(), index, val) };

        if rc != 0 {
            println!("failed to set placeholder {}", placeholder);
        }
        rc
    }
}

impl SetPlaceholder<bool> for EJDBQuery {
    fn set_placeholder(&self, placeholder: &str, index: i32, val: bool) -> ejdb_sys::iwrc {
        let placeholder_str = std::ffi::CString::new(placeholder).unwrap();

        let rc = unsafe { ejdb_sys::jql_set_bool(self.q, placeholder_str.as_ptr(), index, val) };

        if rc != 0 {
            println!("failed to set placeholder {}", placeholder);
        }
        rc
    }
}

impl SetPlaceholder<&str> for EJDBQuery {
    fn set_placeholder(&self, placeholder: &str, index: i32, val: &str) -> ejdb_sys::iwrc {
        let placeholder_str = std::ffi::CString::new(placeholder).unwrap();
        let value_str = std::ffi::CString::new(val).unwrap();

        let rc = unsafe {
            ejdb_sys::jql_set_str(self.q, placeholder_str.as_ptr(), index, value_str.as_ptr())
        };

        if rc != 0 {
            println!("failed to set placeholder {}", placeholder);
        }
        rc
    }
}
