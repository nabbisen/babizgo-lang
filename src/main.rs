
use std::io::{stdin};

extern crate mecab;
//use mecab::Node;
use mecab::Tagger;

const CONSONANT: char = 'k';

fn main() {
    println!("...Waking......");

    let mut s = String::new();
    stdin().read_line(&mut s).expect("Did you enter a correct string?");
    s = remove_linefeeds(s.as_str());
    
    s = do_mecab(s.as_str());
    println!("{}", s);

    let mut r = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut skipped: bool = false;
    for x in 0..chars.len() {
        if skipped {
            skipped = false;
            continue;
        }
        if x == chars.len() - 1 {
            r = format!("{}{}", r, conv_kana_to_alphabet(chars[x].to_string().as_str()));
            break;
        } 
        r = match chars[x].to_string().as_str() {
            " " => format!("{}{}", r, chars[x]),
            _ => {
                let afrom2 = {
                    let afrom2raw = format!("{}{}", chars[x], chars[x + 1]);
                    conv_kana_to_alphabet(afrom2raw.as_str())
                };
                match afrom2.as_str() {
                    "" => {
                        let afrom1 = conv_kana_to_alphabet(chars[x].to_string().as_str());
                        format!("{}{}", r, afrom1)
                    }
                    _ => {
                        skipped = true;
                        format!("{}{}", r, afrom2)
                    }
                }
            },
        };
    }

    println!("{}", r);

    let b = get_babis(r.as_str());
    println!("{}", b);

    let mut br = String::new();
    let bchars: Vec<char> = b.chars().collect();
    let mut skipping: i32 = 0;
    for x in 0..bchars.len() {
        if 0 < skipping {
            skipping = skipping - 1;
            continue;
        }
        if x == bchars.len() - 1 {
            br = format!("{}{}", br, conv_alphabet_to_kana(bchars[x].to_string().as_str()));
            break;
        } 
        br = match bchars[x].to_string().as_str() {
            " " => format!("{}{}", br, bchars[x]),
            _ => {
                let afrom3 =
                    if x < bchars.len() - 2 {
                        let afrom3raw = format!("{}{}{}", bchars[x], bchars[x + 1], bchars[x + 2]);
                        conv_alphabet_to_kana(afrom3raw.as_str())
                    } else {
                        String::from("")
                    };
                if afrom3.as_str() != "" {
                    skipping = 2;
                    format!("{}{}", br, afrom3)
                } else {
                    let afrom2 = {
                        let afrom2raw = format!("{}{}", bchars[x], bchars[x + 1]);
                        conv_alphabet_to_kana(afrom2raw.as_str())
                    };
                    match afrom2.as_str() {
                        "" => {
                            let afrom1 = conv_alphabet_to_kana(bchars[x].to_string().as_str());
                            format!("{}{}", br, afrom1)
                        }
                        _ => {
                            skipping = 1;
                            format!("{}{}", br, afrom2)
                        }
                    }
                }
            },
        };
    }

    println!("{}", br);
}

fn do_mecab(s: &str) -> String {
    let mut mecab: Tagger = mecab::Tagger::new("");

    let mut node: mecab::Node = mecab.parse_to_node(s);
    
    let mut ret = String::new();
    loop {
        match node.next() {
            Some(n) => {
                ret = {
                    let vec: Vec<&str> = n.feature.split(',').collect();
                    let nkana = 
                        if 8 <= vec.len() {
                            vec[7]
                        } else {
                            println!("{:?}", vec);
                            //todo simple hirabana -> katakana
                            ""
                        };
                    match nkana {
                        "" | "*" => ret,
                        "。" | "、" => {
                            ret.pop();
                            format!("{}{}", ret, nkana)
                        },
                        _ => format!("{}{} ", ret, nkana),
                    }
                };
                node = n;
            }
            None => break
        }
    }
    ret
}

fn remove_linefeeds(s: &str) -> String {
    let mut ret = String::from(s);
    if let Some('\n') = ret.chars().next_back() {
        ret.pop();
    }
    if let Some('\r') = ret.chars().next_back() {
        ret.pop();
    }
    ret
}

fn get_babis(s: &str) -> String {
    let mut ret = String::new();
    let chars: Vec<char> = s.chars().collect();
    for c in chars {
        ret.push(c);
        let babis = match c {
            'a' | 'e' | 'i' | 'u' | 'o' => [CONSONANT, c].iter().cloned().collect::<String>(),
            _ => String::new(),
        };
        ret = format!("{}{}", ret, babis);
    }
    ret
}

fn conv_kana_to_alphabet(s: &str) -> String {
    String::from(match s {
        "ウァ" => "wha",
        "ウィ" => "whi",
        "ウェ" => "whe",
        "ウォ" => "who",
        "ィェ" => "ye",
        "キャ" => "kya",
        "キィ" => "kyi",
        "キュ" => "kyu",
        "キェ" => "kye",
        "キョ" => "kyo",
        "クャ" => "qya",
        "クュ" => "qyu",
        "クョ" => "qyo",
        "クァ" => "qwa",
        "クィ" => "qwi",
        "クゥ" => "qwu",
        "クェ" => "qwe",
        "クォ" => "qwo",
        "ギャ" => "gya",
        "ギィ" => "gyi",
        "ギュ" => "gyu",
        "ギェ" => "gye",
        "ギョ" => "gyo",
        "グァ" => "gwa",
        "グィ" => "gwi",
        "グゥ" => "gwu",
        "グェ" => "gwe",
        "グォ" => "gwo",
        "シャ" => "sya",
        "シィ" => "syi",
        "シュ" => "syu",
        "シェ" => "sye",
        "ショ" => "syo",
        "スァ" => "swa",
        "スィ" => "swi",
        "スゥ" => "swu",
        "スェ" => "swe",
        "スォ" => "swo",
        "ジャ" => "zya",
        "ジィ" => "zyi",
        "ジュ" => "zyu",
        "ジェ" => "zye",
        "ジョ" => "zyo",
        "チャ" => "tya",
        "チィ" => "tyi",
        "チュ" => "tyu",
        "チェ" => "tye",
        "チョ" => "tyo",
        "ツァ" => "tsa",
        "ツィ" => "tsi",
        "ツェ" => "tse",
        "ツォ" => "tso",
        "テャ" => "tha",
        "ティ" => "thi",
        "テュ" => "thu",
        "テェ" => "the",
        "テョ" => "tho",
        "トァ" => "twa",
        "トィ" => "twi",
        "トゥ" => "twu",
        "トェ" => "twe",
        "トォ" => "two",
        "ヂャ" => "dya",
        "ヂィ" => "dyi",
        "ヂュ" => "dyu",
        "ヂェ" => "dye",
        "ヂョ" => "dyo",
        "デャ" => "dha",
        "ディ" => "dhi",
        "デュ" => "dhu",
        "デェ" => "dhe",
        "デョ" => "dho",
        "ドァ" => "dwa",
        "ドィ" => "dwi",
        "ドゥ" => "dwu",
        "ドェ" => "dwe",
        "ドォ" => "dwo",
        "ニャ" => "nya",
        "ニィ" => "nyi",
        "ニュ" => "nyu",
        "ニェ" => "nye",
        "ニョ" => "nyo",
        "ヒャ" => "hya",
        "ヒィ" => "hyi",
        "ヒュ" => "hyu",
        "ヒェ" => "hye",
        "ヒョ" => "hyo",
        "ファ" => "fwa",
        "フィ" => "fwi",
        "フゥ" => "fwu",
        "フェ" => "fwe",
        "フォ" => "fwo",
        "フャ" => "fya",
        "フュ" => "fyu",
        "フョ" => "fyo",
        "ビャ" => "bya",
        "ビィ" => "byi",
        "ビュ" => "byu",
        "ビェ" => "bye",
        "ビョ" => "byo",
        "ヴァ" => "va",
        "ヴィ" => "vi",
        "ヴェ" => "ve",
        "ヴォ" => "vo",
        "ヴャ" => "vya",
        "ヴュ" => "vyu",
        "ヴョ" => "vyo",
        "ピャ" => "pya",
        "ピィ" => "pyi",
        "ピュ" => "pyu",
        "ピェ" => "pye",
        "ピョ" => "pyo",
        "ミャ" => "mya",
        "ミィ" => "myi",
        "ミュ" => "myu",
        "ミェ" => "mye",
        "ミョ" => "myo",
        "リャ" => "rya",
        "リィ" => "ryi",
        "リュ" => "ryu",
        "リェ" => "rye",
        "リョ" => "ryo",
        "ア" => "a",
        "イ" => "i",
        "ウ" => "u",
        "エ" => "e",
        "オ" => "o",
        "ヰ" => "wi",
        "ヱ" => "we",
        "ァ" => "la",
        "ィ" => "li",
        "ゥ" => "lu",
        "ェ" => "le",
        "ォ" => "lo",
        "カ" => "ka",
        "キ" => "ki",
        "ク" => "ku",
        "ケ" => "ke",
        "コ" => "ko",
        "ガ" => "ga",
        "ギ" => "gi",
        "グ" => "gu",
        "ゲ" => "ge",
        "ゴ" => "go",
        "ヵ" => "lka",
        "ヶ" => "lke",
        "サ" => "sa",
        "シ" => "si",
        "ス" => "su",
        "セ" => "se",
        "ソ" => "so",
        "ザ" => "za",
        "ジ" => "zi",
        "ズ" => "zu",
        "ゼ" => "ze",
        "ゾ" => "zo",
        "タ" => "ta",
        "チ" => "ti",
        "ツ" => "tu",
        "テ" => "te",
        "ト" => "to",
        "ダ" => "da",
        "ヂ" => "di",
        "ヅ" => "du",
        "デ" => "de",
        "ド" => "do",
        "ッ" => "ltu",
        "ナ" => "na",
        "ニ" => "ni",
        "ヌ" => "nu",
        "ネ" => "ne",
        "ノ" => "no",
        "ハ" => "ha",
        "ヒ" => "hi",
        "フ" => "hu",
        "ヘ" => "he",
        "ホ" => "ho",
        "バ" => "ba",
        "ビ" => "bi",
        "ブ" => "bu",
        "ベ" => "be",
        "ボ" => "bo",
        "ヴ" => "vu",
        "パ" => "pa",
        "ピ" => "pi",
        "プ" => "pu",
        "ペ" => "pe",
        "ポ" => "po",
        "マ" => "ma",
        "ミ" => "mi",
        "ム" => "mu",
        "メ" => "me",
        "モ" => "mo",
        "ヤ" => "ya",
        "ユ" => "yu",
        "ヨ" => "yo",
        "ャ" => "lya",
        "ュ" => "lyu",
        "ョ" => "lyo",
        "ラ" => "ra",
        "リ" => "ri",
        "ル" => "ru",
        "レ" => "re",
        "ロ" => "ro",
        "ワ" => "wa",
        "ヮ" => "lwa",
        "ヲ" => "wo",
        "ン" => "n",
        "、" => ",",
        "。" => ".",
        _ => "",
    })
}

fn conv_alphabet_to_kana(s: &str) -> String {
    String::from(match s {
		"wha" => "ウァ",
        "whi" => "ウィ",
        "whe" => "ウェ",
        "who" => "ウォ",
        "kya" => "キャ",
        "kyi" => "キィ",
        "kyu" => "キュ",
        "kye" => "キェ",
        "kyo" => "キョ",
        "qya" => "クャ",
        "qyu" => "クュ",
        "qyo" => "クョ",
        "qwa" => "クァ",
        "qwi" => "クィ",
        "qwu" => "クゥ",
        "qwe" => "クェ",
        "qwo" => "クォ",
        "gya" => "ギャ",
        "gyi" => "ギィ",
        "gyu" => "ギュ",
        "gye" => "ギェ",
        "gyo" => "ギョ",
        "gwa" => "グァ",
        "gwi" => "グィ",
        "gwu" => "グゥ",
        "gwe" => "グェ",
        "gwo" => "グォ",
        "sya" => "シャ",
        "syi" => "シィ",
        "syu" => "シュ",
        "sye" => "シェ",
        "syo" => "ショ",
        "swa" => "スァ",
        "swi" => "スィ",
        "swu" => "スゥ",
        "swe" => "スェ",
        "swo" => "スォ",
        "zya" => "ジャ",
        "zyi" => "ジィ",
        "zyu" => "ジュ",
        "zye" => "ジェ",
        "zyo" => "ジョ",
        "tya" => "チャ",
        "tyi" => "チィ",
        "tyu" => "チュ",
        "tye" => "チェ",
        "tyo" => "チョ",
        "tsa" => "ツァ",
        "tsi" => "ツィ",
        "tse" => "ツェ",
        "tso" => "ツォ",
        "tha" => "テャ",
        "thi" => "ティ",
        "thu" => "テュ",
        "the" => "テェ",
        "tho" => "テョ",
        "twa" => "トァ",
        "twi" => "トィ",
        "twu" => "トゥ",
        "twe" => "トェ",
        "two" => "トォ",
        "dya" => "ヂャ",
        "dyi" => "ヂィ",
        "dyu" => "ヂュ",
        "dye" => "ヂェ",
        "dyo" => "ヂョ",
        "dha" => "デャ",
        "dhi" => "ディ",
        "dhu" => "デュ",
        "dhe" => "デェ",
        "dho" => "デョ",
        "dwa" => "ドァ",
        "dwi" => "ドィ",
        "dwu" => "ドゥ",
        "dwe" => "ドェ",
        "dwo" => "ドォ",
        "nya" => "ニャ",
        "nyi" => "ニィ",
        "nyu" => "ニュ",
        "nye" => "ニェ",
        "nyo" => "ニョ",
        "hya" => "ヒャ",
        "hyi" => "ヒィ",
        "hyu" => "ヒュ",
        "hye" => "ヒェ",
        "hyo" => "ヒョ",
        "fwa" => "ファ",
        "fwi" => "フィ",
        "fwu" => "フゥ",
        "fwe" => "フェ",
        "fwo" => "フォ",
        "fya" => "フャ",
        "fyu" => "フュ",
        "fyo" => "フョ",
        "bya" => "ビャ",
        "byi" => "ビィ",
        "byu" => "ビュ",
        "bye" => "ビェ",
        "byo" => "ビョ",
        "vya" => "ヴャ",
        "vyi" => "ヴィ",
        "vyu" => "ヴュ",
        "vye" => "ヴェ",
        "vyo" => "ヴョ",
        "pya" => "ピャ",
        "pyi" => "ピィ",
        "pyu" => "ピュ",
        "pye" => "ピェ",
        "pyo" => "ピョ",
        "mya" => "ミャ",
        "myi" => "ミィ",
        "myu" => "ミュ",
        "mye" => "ミェ",
        "myo" => "ミョ",
        "rya" => "リャ",
        "ryi" => "リィ",
        "ryu" => "リュ",
        "rye" => "リェ",
        "ryo" => "リョ",
        "lka" => "ヵ",
        "lke" => "ヶ",
        "ltu" => "ッ",
        "lya" => "ャ",
        "lyu" => "ュ",
        "lyo" => "ョ",
        "lwa" => "ヮ",
        "ye" => "ィェ",
        "va" => "ヴァ",
        "vi" => "ヴィ",
        "ve" => "ヴェ",
        "vo" => "ヴォ",
        "wi" => "ヰ",
        "we" => "ヱ",
        "la" => "ァ",
        "li" => "ィ",
        "lu" => "ゥ",
        "le" => "ェ",
        "lo" => "ォ",
        "ka" => "カ",
        "ki" => "キ",
        "ku" => "ク",
        "ke" => "ケ",
        "ko" => "コ",
        "ga" => "ガ",
        "gi" => "ギ",
        "gu" => "グ",
        "ge" => "ゲ",
        "go" => "ゴ",
        "sa" => "サ",
        "si" => "シ",
        "su" => "ス",
        "se" => "セ",
        "so" => "ソ",
        "za" => "ザ",
        "zi" => "ジ",
        "zu" => "ズ",
        "ze" => "ゼ",
        "zo" => "ゾ",
        "ta" => "タ",
        "ti" => "チ",
        "tu" => "ツ",
        "te" => "テ",
        "to" => "ト",
        "da" => "ダ",
        "di" => "ヂ",
        "du" => "ヅ",
        "de" => "デ",
        "do" => "ド",
        "na" => "ナ",
        "ni" => "ニ",
        "nu" => "ヌ",
        "ne" => "ネ",
        "no" => "ノ",
        "ha" => "ハ",
        "hi" => "ヒ",
        "hu" => "フ",
        "he" => "ヘ",
        "ho" => "ホ",
        "ba" => "バ",
        "bi" => "ビ",
        "bu" => "ブ",
        "be" => "ベ",
        "bo" => "ボ",
        "vu" => "ヴ",
        "pa" => "パ",
        "pi" => "ピ",
        "pu" => "プ",
        "pe" => "ペ",
        "po" => "ポ",
        "ma" => "マ",
        "mi" => "ミ",
        "mu" => "ム",
        "me" => "メ",
        "mo" => "モ",
        "ya" => "ヤ",
        "yu" => "ユ",
        "yo" => "ヨ",
        "ra" => "ラ",
        "ri" => "リ",
        "ru" => "ル",
        "re" => "レ",
        "ro" => "ロ",
        "wa" => "ワ",
        "wo" => "ヲ",
        "a" => "ア",
        "i" => "イ",
        "u" => "ウ",
        "e" => "エ",
        "o" => "オ",
        "n" => "ン",
        "," => "、",
        "." => "。",
        _ => "",
    })
}
