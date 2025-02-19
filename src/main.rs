use extendable_vm::jex::build_jex_machine;
use extendable_vm::jex::constant_parsers::JEX_CONSTANT_PARSERS;
use extendable_vm::machine::parsing::code_parser::CodeParser;
use extendable_vm::machine::parsing::constant_parser::ConstantParserTable;
use extendable_vm::machine::parsing::raw_bytes::RawBytes;

fn main() {
    let path = std::env::args().nth(1).expect("Filepath not given");
    // read file
    let bytes = RawBytes::from_file(&path).expect("File cannot be opened");
    // build parser
    let const_parser_table = ConstantParserTable::with_parsers(&JEX_CONSTANT_PARSERS);
    let parser = CodeParser::new(&const_parser_table);
    // parse file
    let code = parser.parse(&bytes).unwrap_or_else(|e| panic!("{}", e));
    println!("{:?}", code);
    // build machine
    let mut machine = build_jex_machine(&code);
    // start
    let finished_gracefully = machine.start();
    if !finished_gracefully {
        println!("There was an exception!");
    }
}
