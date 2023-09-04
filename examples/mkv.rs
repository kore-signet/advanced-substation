use ssa::{models::events::EventLine, LineStreamParser};

fn main() {
    let parser: LineStreamParser<9, EventLine<'_>> = LineStreamParser::new(
        "ReadOrder, Layer, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
    )
    .unwrap();

    dbg!(parser.parse_line(
        "",
        "1,,Wolf main,Cher,0000,0000,0000,,Et les enregistrements de ses, ondes delta ?"
    ));
}
