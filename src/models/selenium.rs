extern crate selenium_webdriver;
use selenium_webdriver::*;
use crate::models::webrequests::*;

use std::thread;
use std::time::Duration;


///
/// Is it a Trait for use Selenium.
///
pub trait Selenium {
    fn new(website: &str, username: String, input_user: String, input_password: String, button: String, show_chrome: bool) -> SeleniumConfig;
    fn start_browser(&self) -> Browser;
    fn go_brute(&self, b: Browser, password: &str) -> bool;
    fn check_selenium() -> bool;
}

///
/// This structure only allows you to use the functions available on the "Selenium" trait.
///
pub struct SeleniumConfig {
    website: String,
    username: String,
    input_user: &'static str,
    input_password: &'static str,
    button: &'static str,
    show_chrome: bool
}



impl Selenium for SeleniumConfig {

    ///
    /// Created a new instance for Selenium configuration
    ///
    /// Example :
    /// ```
    /// let mut sel = Selenium::new(true);
    /// ```
    ///
    fn new(website: &str, username: String, input_user: String, input_password: String, button: String, show_chrome: bool) -> SeleniumConfig {
        if website != "" && username != "" && input_user != "" && input_password != "" {
            SeleniumConfig {
                website: String::from(website),
                username,
                input_user: StringToStaticStr::new(input_user),
                input_password: StringToStaticStr::new(input_password),
                button: StringToStaticStr::new(button),
                show_chrome,
            }
        } else {

            // Here, when the verification fields made before have failed due to a bug, here we put the demo provided in the download page.
            SeleniumConfig {
                website: String::from("http://localhost/login_demo/login.php"),
                username: String::from("admins"),
                input_user: StringToStaticStr::new("input[name=username]".to_string()),
                input_password: StringToStaticStr::new("input[name=password]".to_string()),
                button: StringToStaticStr::new("input[type=submit]".to_string()),
                show_chrome: true,
            }
        }
    }
    ///
    /// Here, this function allows to say that we can start the browser to then be able to use the function go_brute(&self, "password") -> bool
    /// Examples :
    /// ```
    /// let sel = SeleniumConfig::new(&"".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), true);
    //  let browser = sel.start_browser();
    /// ```
    ///
    ///
    fn start_browser(&self) -> Browser {
        let mut _args = vec![];
        if &self.show_chrome == &true {
            _args = vec!["--disable-popup-blocking", "--disable-extensions"];
        } else {
            _args = vec!["--headless", "--disable-popup-blocking", "--disable-extensions"];
        }
        return Browser::start_session(BrowserName::Chrome, _args)
    }


    ///
    /// This function allows you to get to the heart of the matter, and brute-force the site in question!
    ///
    /// Examples :
    /// ```
    /// let sel = SeleniumConfig::new(&"".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), true);
    /// let browser = sel.start_browser();
    /// let bool = sel.go_brute(browser, "password");
    /// if bool == true {
    ///     println!("password is : password");
    /// } else {
    ///   println!("password isn't : password");
    /// }
    /// ```
    ///
    ///
    fn go_brute(&self, b: Browser, password: &str) -> bool {
        b.open(&self.website).unwrap();
        thread::sleep(Duration::new(1, 0));
        let input_username = b.find_element(LocatorStrategy::CSS(&self.input_user)).unwrap();
        let _ = input_username.send_keys(&self.username);
        let input_password = b.find_element(LocatorStrategy::CSS(&self.input_password)).unwrap();
        let _ = input_password.send_keys(password);
        let btn = b.find_element(LocatorStrategy::CSS(&self.button)).unwrap();
        let _ = btn.click();
        b.open(&self.website).unwrap();
        let mut link: String = b.get_link().unwrap();
        let _ = b.refresh();
        thread::sleep(Duration::new(2, 0));
        link = b.get_link().unwrap();
        if &self.website != &link {
            true
        } else {
            false
        }
    }

    ///
    /// This function only allows you to check if selenium is running.
    /// Examples : 
    /// ```
    /// if check_selenium() == true {
    ///     // selenium is run
    /// } else {
    ///     // selenium is not run
    /// }
    /// 
    fn check_selenium() -> bool {
        let req = WebRequests::new("http://localhost:4444/").send_requests();
        let status = req.get_status();
        if status == String::from("200") {
            true
        } else {
            false
        }
    }
}


///
/// This local structure only allows you to use the rust "selenium_webdriver" library without worry, since the variables must be in & 'static str
/// Examples :
/// ```
/// StringToStaticStr::new("hello".to_string());
/// ```
///
struct StringToStaticStr;

impl StringToStaticStr {
    /// Only allows to convert a String to & 'static str
    /// Examples :
    /// ```
    /// StringToStaticStr::new("hello".to_string());
    /// ```
    fn new(text: String) -> &'static str {
        return Box::leak(text.into_boxed_str())
    }

}
