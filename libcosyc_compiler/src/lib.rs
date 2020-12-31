use libcosyc_diagnostic as diagnostic;
use libcosyc_scan as scan;
use diagnostic::{ Session, error::CompilerError, source::Span };
use scan::{ symbol::SymbolKind, reader::SymbolReader };

pub fn test() {
    let src = "yo waddup";
    let mut reader = SymbolReader::from(src);
    loop {
        let symbol = reader.advance();
        println!("{:?} {}", symbol, reader.span().render(src));
        if let SymbolKind::EoF = symbol {
            break;
        }
    }
}
