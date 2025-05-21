use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "जंगम्" => "jangam",
        "त्रुटिः" => "Err",
        "सम्यक्" => "Ok",
        "सूत्र" => "String",
        "कोश" => "HashMap",
        "मूलभूतम्‌" => "Default",
        "विकल्प" => "Option",
        "अस्ति" => "Some",
        "नास्ति" => "None",
        "फल" => "Result",
        "लिख" => "println",
        "विराम" => "break",
        "अनुक्रम" => "async",
        "प्रतीक्षा" => "await",
        "चक्र" => "loop",
        "चल" => "move",
        "पेटिका" => "crate",
        "अप्राप्य" => "unreachable_code",
        "यथा" => "as",
        "नित्य" => "const",
        "गुण" => "trait",
        "असुरक्षित" => "unsafe",
        "अन्तर" => "in",
        "आरभ्य" => "from",
        "गतिशील" => "dyn",
        "उद्घाट" => "unwrap",
        "स्वभाव" => "default",
        "यथासन्दर्भ" => "as_ref",
        "आयात" => "io",
        "बाह्यम्" => "extern",
        "मिथ्या" => "false",
        "कर्म" => "fn",
        "उत्तर" => "super",
        "निवेश" => "insert",
        "प्राप्" => "get",
        "अनुज्ञा" => "allow",
        "भीति" => "panic",
        "अंश" => "mod",
        "परिवर्तनीय" => "mut",
        "नूतन" => "new",
        "यत्र" => "where",
        "कृते" => "for",
        "प्राप्नुवन्तु" => "get_or_insert_with",
        "मुख्यः" => "main",
        "सार्वजनिक" => "pub",
        "प्रत्यागम" => "return",
        "क्रियान्वयन" => "impl",
        "सन्दर्भ" => "ref",
        "योजय" => "match",
        "यदि" => "if",
        "अथवा" => "else",
        "स्वयम्" => "self",
        "दा" => "let",
        "स्थिर" => "static",
        "संरचना" => "struct",
        "अपेक्षा" => "expect",
        "यावत्" => "while",
        "उपयोग" => "use",
        "प्रविश" => "into",
        "सत्य" => "true",
        "वर्गीकरण" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn jangam(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}