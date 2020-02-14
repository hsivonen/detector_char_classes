/* Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/ */

use encoding_rs::WINDOWS_1252;
use encoding_rs::WINDOWS_1250;
use encoding_rs::WINDOWS_1254;
use encoding_rs::WINDOWS_1257;
use encoding_rs::WINDOWS_1258;
use encoding_rs::ISO_8859_2;
use encoding_rs::ISO_8859_3;
use encoding_rs::ISO_8859_4;
use encoding_rs::ISO_8859_10;
use encoding_rs::ISO_8859_13;
use encoding_rs::ISO_8859_14;
use encoding_rs::ISO_8859_15;
use encoding_rs::ISO_8859_16;
use encoding_rs::MACINTOSH;
use encoding_rs::Encoding;

pub fn is_latin(encoding: &'static Encoding) -> bool {
	encoding == WINDOWS_1252 || encoding == WINDOWS_1250 || encoding == WINDOWS_1254 || encoding == WINDOWS_1257 || encoding == WINDOWS_1258 || encoding == ISO_8859_2 || encoding == ISO_8859_3 || encoding == ISO_8859_4 || encoding == ISO_8859_10 || encoding == ISO_8859_13 || encoding == ISO_8859_14 || encoding == ISO_8859_15 || encoding == ISO_8859_16 || encoding == MACINTOSH
} 

pub const CENTRAL: [&[char]; 72] = [
	&[' '], // Space-like
	&['a'],
	&['b'],
	&['c'],
	&['d'],
	&['e'],
	&['f'],
	&['g'],
	&['h'],
	&['i'],
	&['j'],
	&['k'],
	&['l'],
	&['m'],
	&['n'],
	&['o'],
	&['p'],
	&['q'],
	&['r'],
	&['s'],
	&['t'],
	&['u'],
	&['v'],
	&['w'],
	&['x'],
	&['y'],
	&['z'],
	&['š'],
	&['ś'],
	&['ť'],
	&['ž'],
	&['ź'],
	&['ł'],
	&['ą'],
	&['ş'],
	&['ľ'],
	&['ż'],
	&['ß'],
	&['ŕ'],
	&['á'],
	&['â'],
	&['ă'],
	&['ä'],
	&['ĺ'],
	&['ć'],
	&['ç'],
	&['č'],
	&['é'],
	&['ę'],
	&['ë'],
	&['ě'],
	&['í'],
	&['î'],
	&['ď'],
	&['đ'],
	&['ń'],
	&['ň'],
	&['ó'],
	&['ô'],
	&['ő'],
	&['ö'],
	&['ř'],
	&['ů'],
	&['ú'],
	&['ű'],
	&['ü'],
	&['ý'],
	&['ţ'],
    &['·', '«', '»', '\u{00AD}', '¦'], // plausible next to alphabetic on either side
    &['¨', '¯', '´', '¸', '˛', 'ˇ', '˘', '˙', '¤', '§', '¬', '±', '¢', '×', '÷', 'ƒ'], // implausible next to alphabetic on either side
    &['®', '¶'], // implausible before alphabetic
    &['©', '°', 'µ'], // implausible after alphabetic
];

pub const CYRILLIC: [&[char]; 54] = [
	&[' '], // Space-like
    &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'],
    &['·', '\u{00AD}', '¦', '∙'], // plausible next to alphabetic on either side
    &['¨', '¯', '´', '¸', '˛', 'ˇ', '˘', '¤', '§', '¢', 'ƒ', '№', '⌡', '⌠', '─', '│', '┌', '┐', '└', '┘', '├', '┤', '┬', '┴', '┼', '═', '║', '╒', '╓', '╔', '╕', '╖', '╗', '╘', '╙', '╚', '╛', '╜', '╝', '╞', '╟', '╠', '╡', '╢', '╣', '╤', '╥', '╦', '╧', '╨', '╩', '╪', '╫', '╬', '▀', '▄', '█', '▌', '▐', '░', '▒', '▓', '■'], // implausible next to alphabetic on either side
    &['®', '¶', '²', '»'], // implausible before alphabetic
    &['©', '°', 'µ', '«'], // implausible after alphabetic
    &['¬', '±', '×', '÷', '≈', '≤', '≥'], // implausible next to non-ASCII alphabetic
	&['ѓ'],
	&['ђ'],
	&['љ'],
	&['њ'],
	&['ќ'],
	&['ћ'],
	&['џ'],
	&['ў'],
	&['і'],
	&['ґ'],
	&['ё'],
	&['є'],
	&['ј'],
	&['ѕ'],
	&['ї'],
	&['а'],
	&['б'],
	&['в'],
	&['г'],
	&['д'],
	&['е'],
	&['ж'],
	&['з'],
	&['и'],
	&['й'],
	&['к'],
	&['л'],
	&['м'],
	&['н'],
	&['о'],
	&['п'],
	&['р'],
	&['с'],
	&['т'],
	&['у'],
	&['ф'],
	&['х'],
	&['ц'],
	&['ч'],
	&['ш'],
	&['щ'],
	&['ъ'],
	&['ы'],
	&['ь'],
	&['э'],
	&['ю'],
	&['я'],
];

pub const ICELANDIC: [&[char]; 45] = [
	&[' '], // Space-like
	&['a'],
	&['b'],
	&['c'],
	&['d'],
	&['e'],
	&['f'],
	&['g'],
	&['h'],
	&['i'],
	&['j'],
	&['k'],
	&['l'],
	&['m'],
	&['n'],
	&['o'],
	&['p'],
	&['q'],
	&['r'],
	&['s'],
	&['t'],
	&['u'],
	&['v'],
	&['w'],
	&['x'],
	&['y'],
	&['z'],
	&['š', 'ž', 'ß', 'ç', 'œ', 'à', 'â', 'ã', 'å', 'è', 'ê', 'ë', 'ì', 'î', 'ï', 'ò', 'ô', 'õ', 'ù', 'û', 'ü', 'ÿ', 'ñ'],
	&['á'],
	&['ä'],
	&['æ'],
	&['é'],
	&['í'],
	&['ð'],
	&['ó'],
	&['ö'],
	&['ø'],
	&['ú'],
	&['ý'],
	&['þ'],
    &['·', '\u{00AD}', '¦', '∙'], // plausible next to alphabetic on either side
    &['¨', '¯', '´', '¸', '˛', 'ˇ', '˘', '¤', '§', '¢', 'ƒ', '£', '¥', '№', '⌡', '⌠', 'ª', 'º', '«', '»'], // implausible next to alphabetic on either side (ordinal indicators and guillemets deliberately here)
    &['®', '¶', '¹', '²', '³'], // implausible before alphabetic
    &['©', '°', 'µ', '¼', '½', '¾', '¡', '¿'], // implausible after alphabetic
    &['¬', '±', '×', '÷', '≈', '≤', '≥'], // implausible next to non-ASCII alphabetic
];

pub const WESTERN: [&[char]; 66] = [
	&[' '], // Space-like
	&['a'],
	&['b'],
	&['c'],
	&['d'],
	&['e'],
	&['f'],
	&['g'],
	&['h'],
	&['i'],
	&['j'],
	&['k'],
	&['l'],
	&['m'],
	&['n'],
	&['o'],
	&['p'],
	&['q'],
	&['r'],
	&['s'],
	&['t'],
	&['u'],
	&['v'],
	&['w'],
	&['x'],
	&['y'],
	&['z'],
	&['š'],
	&['œ'],
	&['ß'],
	&['à'],
	&['á'],
	&['â'],
	&['ã'],
	&['ä'],
	&['å'],
	&['æ'],
	&['ç'],
	&['è'],
	&['é'],
	&['ê'],
	&['ë'],
	&['ì'],
	&['í'],
	&['î'],
	&['ï'],
	&['ñ'],
	&['ò'],
	&['ó'],
	&['ô'],
	&['õ'],
	&['ö'],
	&['ø'],
	&['ù'],
	&['ú'],
	&['û'],
	&['ü'],
	&['þ', 'ð', 'ý'], // Unused except in Icelandic and Faroese. Grouped together in order to mark these as implausible to avoid misdetection of Turkish
	&['ž'],
	&['ÿ'], // XXX Add Dutch to training set
    &['·', '«', '»', '\u{00AD}', '¦', '∙'], // plausible next to alphabetic on either side
    &['¨', '¯', '´', '¸', '˛', 'ˇ', '˘', '¤', '§', '¢', 'ƒ', '£', '¥', '№', '⌡', '⌠'], // implausible next to alphabetic on either side
    &['®', '¶', '¹', '²', '³'], // implausible before alphabetic
    &['©', '°', 'µ', '¼', '½', '¾', '¡', '¿'], // implausible after alphabetic
    &['¬', '±', '×', '÷', '≈', '≤', '≥'], // implausible next to non-ASCII alphabetic
	&['ª', 'º'], // ordinal indicators
];

pub const GREEK: [&[char]; 42] = [
	&[' '], // Space-like
    &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'],
    &['·', '´', '\u{00AD}', '¦', '∙', '―', '¬', '±', '×', '÷', '≈', '≤', '≥'], // plausible next to alphabetic on either side (acute/tonos and math operators deliberately here)
    &['¨', '¯', '¸', '˛', 'ˇ', '˘', '΅', 'ͺ', '¤', '§', '¢', 'ƒ', '£', '¥', '€', '№', '⌡', '⌠', '₯'], // implausible next to alphabetic on either side
    &['®', '¶', '¹', '²', '³', '»', '’'], // implausible before alphabetic
    &['©', '°', 'µ', '¼', '½', '¾', '¡', '¿', '«', '‘'], // implausible after alphabetic
 	&['ΐ'],
	&['ά'],
	&['έ'],
	&['ή'],
	&['ί'],
	&['ΰ'],
	&['α'],
	&['β'],
	&['γ'],
	&['δ'],
	&['ε'],
	&['ζ'],
	&['η'],
	&['θ'],
	&['ι'],
	&['κ'],
	&['λ'],
	&['μ'],
	&['ν'],
	&['ξ'],
	&['ο'],
	&['π'],
	&['ρ'],
	&['ς'],
	&['σ'],
	&['τ'],
	&['υ'],
	&['φ'],
	&['χ'],
	&['ψ'],
	&['ω'],
	&['ϊ'],
	&['ϋ'],
	&['ό'],
	&['ύ'],
	&['ώ'],
];

pub const TURKISH: [&[char]; 45] = [
	&[' '], // Space-like
	&['a'],
	&['b'],
	&['c'],
	&['d'],
	&['e'],
	&['f'],
	&['g'],
	&['h'],
	// i treated as non-ASCII
	&['j'],
	&['k'],
	&['l'],
	&['m'],
	&['n'],
	&['o'],
	&['p'],
	&['q'],
	&['r'],
	&['s'],
	&['t'],
	&['u'],
	&['v'],
	&['w'],
	&['x'],
	&['y'],
	&['z'],
	&['œ', 'å', 'à', 'á', 'ã', 'æ', 'è', 'é', 'ë', 'ì', 'í', 'ï', 'ò', 'ó', 'ô', 'õ', 'ø', 'ù', 'ú', 'ÿ'], // Unused vowels
	&['ß', 'ñ', 'š'], // Unused consonants
	&['ä'], // Used for windows-1254-compatible Azeri spelling
	&['â'],
	&['ç'],
	&['ê'],
	&['î'],
	&['ğ'],
	&['ö'],
	&['û'],
	&['ü'],
	&['i'], // Dotted i treated as non-ASCII
	&['ı'],
	&['ş'],
    &['·', '\u{00AD}', '¦', '∙'], // plausible next to alphabetic on either side
    &['¨', '¯', '´', '¸', '˛', 'ˇ', '˘', '¤', '§', '¢', 'ƒ', '£', '¥', '№', '⌡', '⌠', 'ª', 'º'], // implausible next to alphabetic on either side (ordinal indicators deliberately here)
    &['®', '¶', '¹', '²', '³', '»'], // implausible before alphabetic
    &['©', '°', 'µ', '¼', '½', '¾', '¡', '¿', '«'], // implausible after alphabetic
    &['¬', '±', '×', '÷', '≈', '≤', '≥'], // implausible next to non-ASCII alphabetic
];

pub const HEBREW: [&[char]; 58] = [
	&[' '], // Space-like
    &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'],
    &['·', '\u{00AD}', '¦', '∙', '‗', '׀', '־'], // plausible next to alphabetic on either side
    &['¨', '¯', '´', '¸', '˛', 'ˇ', '˘', '¤', '§', '¢', 'ƒ', '£', '¥', '№', '⌡', '⌠'], // implausible next to alphabetic on either side
    &['®', '¶', '¹', '²', '³'], // implausible before alphabetic
    &['©', '°', 'µ', '¼', '½', '¾', '¡', '¿'], // implausible after alphabetic
    &['¬', '±', '×', '÷', '≈', '≤', '≥', '«', '»'], // implausible next to non-ASCII alphabetic
	&['₪'], // currency sign (deliberately not merged with other currency signs)
    &['\u{200E}', '\u{200F}'],
    &['\u{05BF}'],
	&['ְ'],
	&['ֱ'],
	&['ֲ'],
	&['ֳ'],
	&['ִ'],
	&['ֵ'],
	&['ֶ'],
	&['ַ'],
	&['ָ'],
	&['ֹ'],
	&['ֺ'],
	&['ֻ'],
	&['ּ'],
	&['ֽ'],
	&['ׁ'],
	&['ׂ'],
	&['װ'],
	&['ױ'],
	&['ײ'],
	&['׳'],
	&['״'],
	&['א'],
	&['ב'],
	&['ג'],
	&['ד'],
	&['ה'],
	&['ו'],
	&['ז'],
	&['ח'],
	&['ט'],
	&['י'],
	&['ך'],
	&['כ'],
	&['ל'],
	&['ם'],
	&['מ'],
	&['ן'],
	&['נ'],
	&['ס'],
	&['ע'],
	&['ף'],
	&['פ'],
	&['ץ'],
	&['צ'],
	&['ק'],
	&['ר'],
	&['ש'],
	&['ת'],
];

pub const ARABIC: [&[char]; 68] = [
	&[' '], // Space-like
    &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'à', 'â', 'ç', 'è', 'é', 'ê', 'ë', 'î', 'ï', 'ô', 'ù', 'û', 'ü', 'œ', 'À', 'Â', 'Ç', 'È', 'É', 'Ê', 'Ë', 'Î', 'Ï', 'Ô', 'Ù', 'Û', 'Ü', 'Œ'],
	&['\u{200C}'], // ZWNJ
	&['\u{200D}'], // ZWJ // Consider collapsing to plausible next to alphabetic
    &['؛', '؟', '،'], // punctuation, assigned to letters in windows-1251
    &['·', '\u{00AD}', '¦', '∙'], // plausible next to alphabetic on either side
    &['¨', '¯', '´', '¸', '˛', 'ˇ', '˘', '¤', '§', '¢', 'ƒ', '£', '¥', '№', '⌡', '⌠'], // implausible next to alphabetic on either side
    &['®', '¶', '¹', '²', '³', '»'], // implausible before alphabetic
    &['©', '°', 'µ', '¼', '½', '¾', '¡', '¿', '«'], // implausible after alphabetic
    &['¬', '±', '×', '÷', '≈', '≤', '≥'], // implausible next to non-ASCII alphabetic
    &['\u{200E}', '\u{200F}'],
	&['پ'],
	&['ٹ'],
	&['چ'],
	&['ژ'],
	&['ڈ'],
	&['گ'],
	&['ک'],
	&['ڑ'],
	&['ں'],
	&['ھ'],
	&['ہ'],
	&['ء'],
	&['آ'],
	&['أ'],
	&['ؤ'],
	&['إ'],
	&['ئ'],
	&['ا'],
	&['ب'],
	&['ة'],
	&['ت'],
	&['ث'],
	&['ج'],
	&['ح'],
	&['خ'],
	&['د'],
	&['ذ'],
	&['ر'],
	&['ز'],
	&['س'],
	&['ش'],
	&['ص'],
	&['ض'],
	&['ط'],
	&['ظ'],
	&['ع'],
	&['غ'],
	&['ـ'],
	&['ف'],
	&['ق'],
	&['ك'],
	&['ل'],
	&['م'],
	&['ن'],
	&['ه'],
	&['و'],
	&['ى'],
	&['ي'],
	&['ً'],
	&['ٌ'],
	&['ٍ'],
	&['َ'],
	&['ُ'],
	&['ِ'],
	&['ّ'],
	&['ْ'],
	&['ے'],
];

pub const BALTIC: [&[char]; 53] = [
	&[' '], // Space-like
	&['a'],
	&['b'],
	&['c'],
	&['d'],
	&['e'],
	&['f'],
	&['g'],
	&['h'],
	&['i'],
	&['j'],
	&['k'],
	&['l'],
	&['m'],
	&['n'],
	&['o'],
	&['p'],
	&['q'], // Unused ASCII consonant
	&['r'],
	&['s'],
	&['t'],
	&['u'],
	&['v'],
	&['w'], // Unused ASCII consonant
	&['x'], // Unusad ASCII consonant
	&['y'], // Unused ASCII vowel
	&['z'],
	&['ø', 'æ', 'å', 'ó', 'ĩ', 'ô', 'ú', 'ä', 'õ', 'ö', 'ü', 'û', 'ũ', 'í', 'î', 'ë', 'ã', 'â', 'á'], // Unused non-ASCII vowels
	&['ŗ'], //
	&['ß', 'ć', 'ź', 'ń', 'ł', 'ś', 'ż', 'ĸ', 'ŧ', 'ŋ', 'đ'], // Unused non-ASCII consonants
	&['ą'], //
	&['į'], //
	&['ā'], // 
	&['ę'], //
	&['ē'], //
	&['č'], //
	&['é'], // In principle unused but stands out
	&['ė'], //
	&['ģ'], //
	&['ķ'], //
	&['ī'], //
	&['ļ'], //
	&['š'], //
	&['ņ'], //
	&['ō'], //
	&['ų'], //
	&['ū'], //
	&['ž'], //
    &['·', '\u{00AD}', '¦', '∙', '’', '“'], // plausible next to alphabetic on either side
    &['¨', '¯', '´', '¸', '˛', 'ˇ', '˘', '˙', '¤', '§', '¢', 'ƒ', '£', '¥', '№', '⌡', '⌠'], // implausible next to alphabetic on either side
    &['®', '¶', '¹', '²', '³', '»', '”'], // implausible before alphabetic
    &['©', '°', 'µ', '¼', '½', '¾', '¡', '¿', '«', '„'], // implausible after alphabetic
    &['¬', '±', '×', '÷', '≈', '≤', '≥'], // implausible next to non-ASCII alphabetic
];

pub const VIETNAMESE: [&[char]; 58] = [
	&[' '], // Space-like
	&['a'],
	&['b'],
	&['c'],
	&['d'],
	&['e'],
	&['f'],
	&['g'],
	&['h'],
	&['i'],
	&['j'],
	&['k'],
	&['l'],
	&['m'],
	&['n'],
	&['o'],
	&['p'],
	&['q'],
	&['r'],
	&['s'],
	&['t'],
	&['u'],
	&['v'],
	&['w'],
	&['x'],
	&['y'],
	&['z'],
	&['œ', 'ä', 'å', 'æ', 'ë', 'ö', 'ø', 'ü'], // Foreign vowels
	&['̀'],
	&['̉'],
	&['̃'],
	&['ß', 'ç', 'ñ'], // Foreign consonants
	&['à'],
	&['á'],
	&['â'],
	&['ă'],
	&['è'],
	&['é'],
	&['ê'],
	&['́'],
	&['í'],
	&['î'],
	&['ï'], // Used quite a bit in a manner similar to French (to indicate absence of diphtong)
	&['đ'],
	&['̣'],
	&['ó'],
	&['ô'],
	&['ơ'],
	&['ù'],
	&['ú'],
	&['û'],
	&['ư'],
	&['ÿ'], // Distinctive enough not to be grouped together with foreign vowels
    &['·', '\u{00AD}', '¦', '∙'], // plausible next to alphabetic on either side
    &['¨', '¯', '´', '¸', '˛', 'ˇ', '˘', '¤', '§', '¢', 'ƒ', '£', '¥', '№', '⌡', '⌠', '«', '»', 'ª', 'º', '₫'], // implausible next to alphabetic on either side (ordinal indicators and guillements deliberately here)
    &['®', '¶', '¹', '²', '³'], // implausible before alphabetic
    &['©', '°', 'µ', '¼', '½', '¾', '¡', '¿'], // implausible after alphabetic
    &['¬', '±', '×', '÷', '≈', '≤', '≥'], // implausible next to non-ASCII alphabetic
];

pub const THAI: [&[char]; 78] = [
	&[' '], // Space-like
    &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'],
	&['€', '฿'], // euro, assigned to letter in windows-1251
	&['๚', '๛', '๏'],
	&['๐', '๑', '๒', '๓', '๔', '๕', '๖', '๗', '๘', '๙'],
	&['ก'],
	&['ข'],
	&['ฃ'], // Obsolete but more common
	&['ค'],
	&['ฅ'], // Obsolete
	&['ฆ'],
	&['ง'],
	&['จ'],
	&['ฉ'],
	&['ช'],
	&['ซ'],
	&['ฌ'],
	&['ญ'],
	&['ฎ'],
	&['ฏ'],
	&['ฐ'],
	&['ฑ'],
	&['ฒ'],
	&['ณ'],
	&['ด'],
	&['ต'],
	&['ถ'],
	&['ท'],
	&['ธ'],
	&['น'],
	&['บ'],
	&['ป'],
	&['ผ'],
	&['ฝ'],
	&['พ'],
	&['ฟ'],
	&['ภ'],
	&['ม'],
	&['ย'],
	&['ร'],
	&['ฤ'],
	&['ล'],
	&['ฦ'], // Obsolete
	&['ว'],
	&['ศ'],
	&['ษ'],
	&['ส'],
	&['ห'],
	&['ฬ'],
	&['อ'],
	&['ฮ'],
	&['ฯ'],
	&['ะ'],
	&['ั'],
	&['า'],
	&['ำ'],
	&['ิ'],
	&['ี'],
	&['ึ'],
	&['ื'],
	&['ุ'],
	&['ู'],
	&['ฺ'],
	&['เ'],
	&['แ'],
	&['โ'],
	&['ใ'],
	&['ไ'],
	&['ๅ'], // Obsolete
	&['ๆ'],
	&['็'],
	&['่'],
	&['้'],
	&['๊'],
	&['๋'],
	&['์'],
	&['ํ'],
	&['๎'],
];
