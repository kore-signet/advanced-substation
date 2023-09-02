use ssa::{
    models::{
        events::{self, EventLine},
        script_info::ScriptInfo,
        style::{self, Style},
    },
    LineItem, SSAParser,
};

fn main() {
    let data = std::fs::read_to_string(std::env::args().skip(1).next().unwrap()).unwrap();
    let mut parser = SSAParser::new(&data);

    let parsed = parser
        .section()
        .unwrap()
        .as_key_value::<ScriptInfo<'_>>()
        .unwrap();
    println!("{:#?}", parsed);

    let style_section = loop {
        let mut section = parser.section().unwrap();
        if Style::validate_section_name(section.title) {
            break section;
        } else {
            section.for_each(|_| ());
            continue;
        }
    };

    let style_parser = style_section
        .as_stream_section::<{ style::MAX_FIELDS }, Style<'_>>()
        .unwrap();

    for style in style_parser {
        println!("{:#?}", style);
    }

    let event_parser = parser
        .section()
        .unwrap()
        .as_stream_section::<{ events::MAX_FIELDS }, EventLine<'_>>()
        .unwrap();

    for event in event_parser {
        println!("{:#?}", event);
    }
}
