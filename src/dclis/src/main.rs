/*
* Copyright 2020 Mike Chambers
* https://github.com/mikechambers/dcli
* 
* Permission is hereby granted, free of charge, to any person obtaining a copy of 
* this software and associated documentation files (the "Software"), to deal in 
* the Software without restriction, including without limitation the rights to
* use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
* of the Software, and to permit persons to whom the Software is furnished to do 
* so, subject to the following conditions:
* 
* The above copyright notice and this permission notice shall be included in all 
* copies or substantial portions of the Software.
* 
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR 
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
* FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
* COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
* IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
* CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

mod memberidsearch;

use dcli::platform::Platform;
use memberidsearch::MemberIdSearch;

use exitfailure::ExitFailure;

use structopt::StructOpt;

#[derive(StructOpt)]
/// Command line tool for retrieving primary Destiny 2 member ids.
///
/// Retrieves the primary Destiny 2 membershipId and platform for specified username or
/// steam 64 id and platform. That may a membershipId on a platform different
/// that the one specified, depending on the cross save status of the account. It
/// will return the primary membershipId that all data will be associate with.
struct Opt {
    /// Platform for specified id
    ///
    /// Platform for specified id. Valid values are:
    /// xbox, playstation, stadia or steam
    #[structopt(short = "p", long = "platform", required = true)]
    platform: Platform,

    #[structopt(short = "i", long = "id", required = true)]
    /// User name or steam 64 id
    ///
    /// User name or steam 64 id in the format 00000000000000000 (17 digit ID)
    id: String,

    ///Compact output in the form of membership_id:platform_id
    #[structopt(short = "c", long = "compact")]
    compact: bool,

    ///Print out the url used for the API call
    #[structopt(short = "u", long = "url")]
    url: bool,
}

fn is_valid_steam_id(steam_id: &str) -> bool {
    //make sure it can be parsed into a u64
    let parses = match steam_id.parse::<u64>() {
        Ok(_e) => true,
        Err(_e) => false,
    };

    parses && steam_id.chars().count() == 17
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();

    if opt.platform == Platform::Steam && !is_valid_steam_id(&opt.id) {
        println!("Invalid steam 64 id.");
        std::process::exit(1);
    }

    if !opt.compact {
        println!(
            "Searching for {id} on {platform}",
            id = opt.id,
            platform = opt.platform,
        );
    }

    let member_search = MemberIdSearch::new(opt.url);

    let membership = member_search
        .retrieve_member_id(&opt.id, opt.platform)
        .await;

    let membership = match membership {
        Some(e) => match e {
            Ok(e) => e,
            Err(e) => {
                println!("{}", e);
                //TODO: can we just return here?
                std::process::exit(1);
            }
        },
        None => {
            //TODO: add more info on what we searched for here
            println!("Member not found");
            std::process::exit(0);
        }
    };

    //TODO: compare original input to what was returned to make sure we got an exact
    //match

    if opt.compact {
        println!(
            "{membership_id}:{platform_id}",
            membership_id = membership.id,
            platform_id = membership.platform.to_id()
        );
    } else {
        println!(
            "Membership Id : {membership_id}\nPlatform : {platform} ({platform_id})",
            membership_id = membership.id,
            platform = membership.platform,
            platform_id = membership.platform.to_id()
        );
    };

    Ok(())
}