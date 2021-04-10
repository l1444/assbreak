use discord_rpc_client::Client;

///
/// Here, there is the public key to be able to use the DiscordRPC on assbreak.
///
static DISCORD_CLIENT: u64 = 791452694388670465;

///
/// This allows you to define the functions available on DiscordInfos ;)
///
pub trait DiscordRPC {
    fn new(title: &str, description: &str, l_image: &str, s_image: &str) -> DiscordInfos;
    fn start_rpc(&self);
}

///
/// Here, it is a structure allowing users to use the functions present :)
///
pub struct DiscordInfos {
    title: String,
    description: String,
    l_image: String,
    s_image: String
}

///
/// We have an impl which currently stores 2 public functions, so 2 functions that can be used in the project :)
///
impl DiscordRPC for DiscordInfos {

    ///
    ///
    /// This function allows to make an instance of the rich presence before starting it.
    /// Examples :
    /// ```
    ///  let d_rpc = DiscordInfos::new("", "", "", "");
    /// ```
    ///
    ///
    fn new(title: &str, description: &str, l_image: &str, s_image: &str) -> DiscordInfos {
        DiscordInfos {
            title: String::from(title),
            description: String::from(description),
            l_image: String::from(l_image),
            s_image: String::from(s_image)
        }
    }


    ///
    ///
    /// This function allows you to start the rich presence to be able to display it on discord.
    /// Examples :
    /// ```
    ///  let d_rpc = DiscordInfos::new("", "", "", "");
    //   d_rpc.start_rpc();
    /// ```
    ///
    ///
    fn start_rpc(&self) {
        let mut discord = Client::new(DISCORD_CLIENT).unwrap();
        discord.start();
        discord.set_activity(|a| a
            .state(&self.description)
            .assets(|ass| ass
                .large_image(&self.l_image)
                .large_text(&self.title)
                .small_image(&self.s_image)));
    }
}
