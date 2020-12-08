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
    println!("  Coded by L14ms1 for pa$$ion");
    println!("  version : 0.1.11b");
    println!("  For educational purposes only !");
    println!("---------------------------------------------------------------------------------------------------------");
    println!(" Brute-force a password from a website form : [EXPERIMENTAL]");
    println!("    $1 - input[type]: text && password && submit        $2 - input[type]: email && password && submit");
    println!("    $3 - input[type]: text && password && <button>      $4 - input[type]: email && password && <button>");
    println!("    $5 - input[type]: text && password && button*       $6 - input[type]: email && password && button*");
    println!("    $7 - Personnalize                                   *personnalize");
    println!("---------------------------------------------------------------------------------------------------------");
    println!(" Brute-force the password of a mailbox : [BETA]");
    println!("    $8 - An Outlook & Hotmail email address             $9 - An iCloud email address");
    println!("   $10 - An Yahoo email address:                       $11 - Personnalize (imap address)");
    println!(" Doesn't work for gmail because OAUTH2 ;)");
    println!("---------------------------------------------------------------------------------------------------------");
    if verify_prerequisites() == false {
        println!("[!] [WARNING] Please put selenium.jar and chromedriver.exe in C:/webdrivers/");
    }
    if verify_update(String::from("0.1.11b"), String::from("https://raw.githubusercontent.com/L14ms111/assbreak/main/VERSION")) == true {
        let version: String = download_string(String::from("https://raw.githubusercontent.com/L14ms111/assbreak/main/VERSION"));
        println!("[!] [WARNING] A new version is available : {:?}, the software update is available on this link : https://github.com/L14ms111/assbreak/releases", filter(version).replace(" ", ""));
    }
}