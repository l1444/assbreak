extern crate reqwest;

use std::io::*;


///
/// The structure to initialize webrequests.
///
/// Examples :
/// ```
/// let x = WebRequests::new("http://httpbin.org/get");
/// ```
///
pub struct WebRequests {
    addr: String,
    status: String,
    body: String,
}

impl WebRequests {

    ///
    /// The function that initializes webrequests :)
    ///
    /// Examples :
    ///
    /// ```
    /// let x = WebRequests::new("http://httpbin.org/get");
    // ```
    pub fn new(addr: &str) -> Self {
        WebRequests {
            addr: String::from(addr),
            status: String::new(),
            body: String::new()
        }
    }
    ///
    /// Allows you to store 2 values which are the body of a web page, and the status.
    ///
    /// Examples :
    ///
    /// ```
    /// let x = WebRequests::new("http://httpbin.org/get")
    ///         .send_requests();
    /// ```
    /// 
    pub fn send_requests(&self) -> Self {
        match reqwest::blocking::get(&self.addr) {
            Ok(mut res) => {
                let mut body = String::new();
                res.read_to_string(&mut body).unwrap();

                WebRequests {
                    addr: self.addr.to_string(),
                    body,
                    status : res.status().as_u16().to_string()
                }
            },
            Err(_) => {
                WebRequests {
                    addr: self.addr.to_string(),
                    status: "".to_string(),
                    body: "".to_string(),
                }
            }
        }
    }

    ///
    /// receive the content of the page ;)
    ///
    /// Examples :
    ///
    /// ```
    /// let x = WebRequests::new("http://httpbin.org/get")
    ///         .send_requests();
    /// let body: String = x.get_body();
    ///
    /// println!("{}", body);
    /// ```
    ///
    pub fn get_body(self) -> String {
        return self.body
    }

    ///
    /// receive the status of the page ;)
    ///
    /// Examples :
    ///
    /// ```
    /// let x = WebRequests::new("http://httpbin.org/get")
    ///         .send_requests();
    /// let status: String = x.get_status();
    ///
    /// println!("{}", status);
    /// ```
    ///
    pub fn get_status(self) -> String {
        return self.status
    }

    ///
    /// Check if there is a connection or not. 
    /// 
    /// Examples : 
    /// ```
    /// let req = WebRequests::new("https://github.com").send_requests();
    /// if req.get_status == "" {
    ///     // you're offline
    /// }
    /// 
    /// 
    pub fn check_connection(&self) -> bool {
        let req = WebRequests::new(&self.addr).send_requests();
        if req.get_status() != "" {
            true
        } else {
            false
        }
    }
}