use ssa::{
    models::{
        events::{self, EventLine, EventLineParser},
        script_info::ScriptInfo,
        style::{self, Style, StyleParser},
    },
    LineItemParser, SSAParser,
};

fn main() {
    let data = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut parser = SSAParser::new(&data);

    let parsed = parser
        .section()
        .unwrap()
        .as_key_value::<ScriptInfo<'_>>()
        .unwrap();
    println!("{:#?}", parsed);

    let style_section = loop {
        let section = parser.section().unwrap();
        if StyleParser::validate_section_name(section.title) {
            break section;
        } else {
            section.for_each(|_| ());
            continue;
        }
    };

    let style_parser = style_section
        .as_stream_section::<{ style::MAX_FIELDS }, StyleParser>()
        .unwrap();

    for style in style_parser {
        println!("{:#?}", style);
    }

    let event_parser = parser
        .section()
        .unwrap()
        .as_stream_section::<{ events::MAX_FIELDS }, EventLineParser>()
        .unwrap();

    for event in event_parser {
        println!("{:#?}", event);
    }
}
