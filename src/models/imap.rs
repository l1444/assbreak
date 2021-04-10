extern crate imap;
extern crate native_tls;

///
/// The default port to be able to send a request to an imap server, and it also uses TLS technology.
///
static PORT: usize = 993;

///
/// It allows to define the functions available on ImapConfig !!!
///
pub trait Imap {
    fn new(imap: &str, email: &str, password: &str) -> ImapConfig;
    fn connect_account(&self) -> bool;
}

///
/// Here, it is a structure allowing users to use the functions present :)
///
pub struct ImapConfig {
    imap: String,
    email: String,
    password: String,
}


///
/// We have an impl which currently stores 2 public functions, so 2 functions that can be used in the project :)
///
impl ImapConfig {
    ///
    /// This function is used to initialize and be able to use the imap technology.
    ///
    /// Example :
    /// ```
    /// let imap = Imap::new("imap-mail.outlook.com", "examples@outlook.fr", "12345");
    /// ```
    /// -> return *ImapConfig*
    pub fn new(imap: &str, email: &str, password: &str) -> ImapConfig {
        ImapConfig {
            imap: String::from(imap),
            email: String::from(email),
            password: String::from(password),
        }
    }


    ///
    /// Allows you to connect with an email address (except systems using two-factor authentication) !
    ///
    /// Example :
    /// ```
    ///let imap = ImapConfig::new("imap-mail.outlook.com", "example@outlook.fr", "yeeaaaah is password");
    ///if imap.connect_account() == true{
    ///     println!("yes");
    ///} else {
    ///     print!("no");
    ///}
    /// ```
    ///
    pub fn connect_account(&self) -> bool {
        if &self.imap != "" && &self.email != "" && &self.password != "" {
            let tls = native_tls::TlsConnector::builder().build().unwrap();
            let client = imap::connect((&self.imap as &str), &self.imap as &str, &tls).unwrap();

            match client.login(&self.email, &self.password) {
                Ok(_) => true,
                Err(_) => false
            }
        } else {
            false
        }
    }
}
