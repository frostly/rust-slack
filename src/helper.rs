use slack::SlackText;

pub fn opt_bool_to_u8(opt: &Option<bool>) -> Option<u8> {
    match opt {
        &Some(true) => Some(1u8),
        &Some(false) => Some(0u8),
        _ => None,
    }
}

pub fn opt_str_to_string(opt: &Option<&str>) -> Option<String> {
    match opt {
        &Some(x) => Some(x.to_string()),
        _ => None,
    }
}

pub fn opt_str_to_slacktext(opt: &Option<&str>) -> Option<SlackText> {
    match opt {
        &Some(x) => Some(SlackText::new(x)),
        _ => None,
    }
}
