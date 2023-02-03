macro_rules! fake_embed_to_embed {
    ($a:expr) => {
        |e: &mut CreateEmbed| {
            if $a.timestamp.is_some() {
                e.timestamp(serenity::model::timestamp::Timestamp::from_unix_timestamp($a.timestamp.unwrap() as i64).unwrap());
            }

            if $a.footer.is_some() {
                let text = $a.footer.unwrap();
                e.footer(|footer| {
                    footer.text(text)
                });
            }

            if $a.fields.is_some() {
                e.fields($a.fields.unwrap());
            }

            if $a.color.is_some() {
                e.colour($a.color.unwrap());
            }

            if $a.description.is_some() {
                e.description($a.description.unwrap());
            }

            if $a.title.is_some() {
                e.title($a.title.unwrap());
            }

            if $a.url.is_some() {
                e.url($a.url.unwrap().clone());
            }

            if $a.author.is_some() {
                let name = $a.author.unwrap();
                e.author(|author| {
                    author.name(&name)
                        .url(format!("https://reddit.com/u/{}", name))
                });
            }

            if $a.thumbnail.is_some() {
                e.thumbnail($a.thumbnail.unwrap());
            }

            if $a.image.is_some() {
                let url = $a.image.clone().unwrap();
                if !(url.contains("imgur") && url.contains(".gif")) && !(url.contains("redgifs")) {
                    e.image(url);
                }
            }

            return e;
        }
    }
}

macro_rules! fake_embed_to_buttons {
    ($a:expr) => {
        |c| {
            c.create_action_row(|a| {
                for button in $a.clone().buttons.unwrap() {
                    a.create_button(|b| {
                        b.label(button.label)
                            .style(button.style)
                            .disabled(button.disabled);

                        if button.url.is_some() {
                            b.url(button.url.unwrap());
                        }

                        if button.custom_id.is_some() {
                            b.custom_id(button.custom_id.unwrap());
                        }

                        return b;
                    });
                }

                return a;
            })
        }
    };
}

macro_rules! fake_embed_to_message {
    ($a:expr) => {
        |message| {
            let mut do_buttons = true;
            if $a.url.is_some() {
                let url = $a.image.clone().unwrap();
                if (url.contains("imgur") && url.contains(".gif") ) || url.contains("redgifs") {
                    do_buttons = false;
                }
            }
    
            if ($a.buttons.is_some()) && do_buttons {
                message.components(fake_embed_to_buttons!($a));
            }
    
            message.embed(fake_embed_to_embed!($a))
        }
    };
}