use super::func::*;

pub fn header() {
    print!("
:::'###:::::'######:::'######::'########::'########::'########::::'###::::'##:::'##:
::'## ##:::'##... ##:'##... ##: ##.... ##: ##.... ##: ##.....::::'## ##::: ##::'##::
:'##:. ##:: ##:::..:: ##:::..:: ##:::: ##: ##:::: ##: ##::::::::'##:. ##:: ##:'##:::
'##:::. ##:. ######::. ######:: ########:: ########:: ######:::'##:::. ##: #####::::
:#########::..... ##::..... ##: ##.... ##: ##.. ##::: ##...:::: #########: ##. ##:::
:##.... ##:'##::: ##:'##::: ##: ##:::: ##: ##::. ##:: ##::::::: ##.... ##: ##:. ##::
:##:::: ##:. ######::. ######:: ########:: ##:::. ##: ########: ##:::: ##: ##::. ##:
..:::::..:::......::::......:::........:::..:::::..::........::..:::::..::..::::..::\n");
    println!("---------------------------------------------------------------------------------------------------------");
    println!("  Coded by L14ms1");
    println!("  version : 0.1.8b");
    println!("  For educational purposes only !");
    println!("---------------------------------------------------------------------------------------------------------");
    println!(" Brute-force a password from a website form : ");
    println!("    1 - input[type]: text && password && submit        2 - input[type]: email && password && submit");
    println!("    3 - input[type]: text && password && <button>      4 - input[type]: email && password && <button>");
    println!("    5 - Personnalize");
    println!("---------------------------------------------------------------------------------------------------------");
    println!(" Brute-force the password of a mailbox : ");
    println!("    6 - An Outlook & Hotmail email address             7 - An iCloud email address");
    println!("    8 - An Yahoo email address:                        9 - Personnalize (imap address)");
    println!(" Doesn't work for gmail because OAUTH2 ;)");
    println!("---------------------------------------------------------------------------------------------------------");
    if verify_prerequisites() == false {
        println!("[!] [WARNING] Please put selenium.jar and chromedriver.exe in C:/webdrivers/");
    }
    // only ssl certificate valid :)
    if verify_update(String::from("0.1.8b"), String::from("https://raw.githubusercontent.com/L14ms111/assbreak/main/VERSION")) == true {
        let version: String = download_string(String::from("https://raw.githubusercontent.com/L14ms111/assbreak/main/VERSION"));
        println!("[!] [WARNING] A new version is available : {:?}, the software update is available on this link : https://github.com/L14ms111/assbreak/releases", filter(version).replace(" ", ""));
    }
}