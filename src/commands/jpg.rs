use serenity::builder;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
// use::serenity::model::prelude::interaction::application_command::CommandDataOptionValue;

// fn option_value_to_i64(value: &CommandDataOptionValue) -> Option<i64> {
//     match value {
//         CommandDataOptionValue::Integer(int_val) => Some(*int_val),
//         _ => None,
//     }
// }

use serenity::json::Value;

fn option_value_to_string(value: &Option<Value>) -> String {
    match value {
        Some(Value::String(some_value)) => some_value.as_str().to_string(),
        _ => "Error".to_string(),
    }
}

pub fn run(options: &[CommandDataOption]) -> String {
    // let index00 = options
    //     .get(0)
    //     .expect("Missing int option")
    //     .resolved
    //     .as_ref()
    //     .expect("Expected attachment object");
    for option in options {
        println!(
            "option name: {}, option value: {}",
            option.name,
            option_value_to_string(&option.value)
        );
        if option.name == "扣分熊熊" || option.name == "近平" {
            return option_value_to_string(&option.value);
        } else {
            println!("nono jpg");
            return "./assets/images/jpg.jpg".to_string();
        }
    }
    // let number = options.get(1).expect("Missing number option").resolved;
    // let option_value = CommandDataOptionValue::Integer(64);
    // let wrapper = CommandDataOptionValueWrapper::Integer(option_value.try_into().unwrap());
    // let int_value: i64 = wrapper.try_into().unwrap();
    // if let Some(int_value) = option_value_to_i64(&index00) {
    //     println!("The integer value is: {}", int_value);
    //     return int_value.to_string();
    // } else {
    //     println!("The input value is not an integer");
    //     return "The input value is not an integer".to_string();
    // }
    return "./assets/images/jpg.jpg".to_string();
}

fn bear_string(s: &str) -> String {
    let location = "./assets/images/";
    let together = format!("{}{}", location, s);
    together
}

pub fn register(
    command: &mut builder::CreateApplicationCommand,
) -> &mut builder::CreateApplicationCommand {
    command
        .name("jpg")
        .description("jpgs")
        .create_option(|option| {
            option
                .name("扣分熊熊")
                .description("Bear group")
                .kind(CommandOptionType::String)
                .add_string_choice("和別的女生講話", bear_string("talking_to_other_girl.jpg"))
                .add_string_choice("今天很乖", bear_string("good.jpg"))
                .add_string_choice("扣錯了", bear_string("sorry.jpg"))
                .add_string_choice("就知道欺負我", bear_string("you_bully_me.jpg"))
                .add_string_choice("不給澀澀", bear_string("no_hs_hs.jpg"))
                .add_string_choice("不回消息", bear_string("you_dont_answer_me.jpg"))
                .add_string_choice("冷落我", bear_string("you_bad_bad.jpg"))
        })
        .create_option(|option| {
            option
                .name("近平")
                .description("subcommand group")
                .kind(CommandOptionType::String)
                .add_string_choice("近平不喜歡你這樣", "./assets/images/jinping_dont_like_this.jpg")
        })
}
