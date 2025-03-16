use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Bc" => "Err",
        "Theek" => "Ok",
        "Shabd" => "String",
        "Hashkamap" => "HashMap",
        "Default" => "Default",
        "Bhenchod" => "Error",
        "Vikalp" => "Option",
        "Kuch" => "Some",
        "KuchNahi" => "None",
        "Parinaam" => "Result",
        "Khud" => "Self",
        "Dikhana" => "println",
        "Rokna" => "break",
        "Asynk" => "async",
        "Intezaar" => "await",
        "Chakkar" => "loop",
        "Hilaana" => "move",
        "Sandook" => "crate",
        "Pahunchna" => "unreachable_code",
        "Jaise" => "as",
        "Sthir" => "const",
        "Samjhauta" => "trait",
        "Khatarnak" => "unsafe",
        "Me" => "in",
        "Se" => "from",
        "Gati" => "dyn",
        "Khulna" => "unwrap",
        "Mool" => "default",
        "Jaise_Ref" => "as_ref",
        "AndarBahar" => "io",
        "Bahar" => "extern",
        "Jhooth" => "false",
        "Karya" => "fn",
        "Mahan" => "super",
        "Daalna" => "insert",
        "Paana" => "get",
        "Anumati" => "allow",
        "Gussa" | "Gali" | "Arre" => "panic",
        "Khand" => "mod",
        "BadalneYogya" => "mut",
        "Naya" => "new",
        "Kahan" => "where",
        "KeLiye" => "for",
        "PaanaYaDaalna" => "get_or_insert_with",
        "Mukhya" => "main",
        "Sarvajanik" => "pub",
        "Kya" => None?,
        "Wapas" => "return",
        "Karyanvayan" => "impl",
        "Sandarbh" => "ref",
        "Milaana" => "match",
        "Agar" => "if",
        "Warna" => "else",
        "khud" => "self",
        "Manzoor" => "let",
        "Sthayi" => "static",
        "Sanrachna" => "struct",
        "Apeksha" => "expect",
        "JabTak" => "while",
        "Upyog" => "use",
        "Mein" => "into",
        "Sach" => "true",
        "Ginti" => "enum",
        "Samuh" => "Group",
        "Pehchaan" => "Ident",
        "JetonPravah" => "TokenStream",
        "JetonVriksh" => "TokenTree",
        "Pehchaan_Se_Sutra" => "to_string",
        "Sutra_Ke_Roop_Mein" => "as_str",
        "Kshetra" => "span",
        "Suchi" => "Vec",
        "Pravah" => "stream",
        "Dabana" => "push",
        "Vistar" => "extend",
        "Seema" => "delimiter",
        "ViramChinh" => "Punct",
        "Akshar" => "Literal",
        "Karya_Macro" => "proc_macro",
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
pub fn rouille(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
