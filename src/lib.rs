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
	&['ß'],
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

pub const CYRILLIC: [&[char]; 51] = [
	&[' '], // Space-like
    &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'],
    &['·', '\u{00AD}', '¦', '∙'], // plausible next to alphabetic on either side
    &['¨', '¯', '´', '¸', '˛', 'ˇ', '˘', '¤', '§', '¢', 'ƒ', '№', '⌡', '⌠', '─', '│', '┌', '┐', '└', '┘', '├', '┤', '┬', '┴', '┼', '═', '║', '╒', '╓', '╔', '╕', '╖', '╗', '╘', '╙', '╚', '╛', '╜', '╝', '╞', '╟', '╠', '╡', '╢', '╣', '╤', '╥', '╦', '╧', '╨', '╩', '╪', '╫', '╬', '▀', '▄', '█', '▌', '▐', '░', '▒', '▓', '■'], // implausible next to alphabetic on either side
    &['®', '¶', '²', '»'], // implausible before alphabetic
    &['©', '°', 'µ', '«'], // implausible after alphabetic
    &['¬', '±', '×', '÷', '≈', '≤', '≥'], // implausible next to non-ASCII alphabetic
	&['ѓ', 'ґ', 'ќ', 'ѕ'],
	&['ђ'],
	&['љ'],
	&['њ'],
	&['ћ'],
	&['џ'],
	&['ў'],
	&['і'],
	&['ё'],
	&['є'],
	&['ј'],
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
	&['ß', 'š', 'ž', 'ç', 'œ', 'à', 'â', 'ã', 'å', 'è', 'ê', 'ë', 'ì', 'î', 'ï', 'ò', 'ô', 'õ', 'ù', 'û', 'ü', 'ÿ', 'ñ'],
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

pub const WESTERN: [&[char]; 65] = [
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
	&['ß'],
	&['š'],
	&['œ'],
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
	&['ž'],
	&['ÿ'], // XXX Add Dutch to training set
    &['·', '«', '»', '\u{00AD}', '¦', '∙'], // plausible next to alphabetic on either side
    &['¨', '¯', '´', '¸', '˛', 'ˇ', '˘', '¤', '§', '¢', 'ƒ', '£', '¥', '№', '⌡', '⌠', 'þ', 'ð', 'ý'], // implausible next to alphabetic on either side
    &['®', '¶', '¹', '²', '³'], // implausible before alphabetic
    &['©', '°', 'µ', '¼', '½', '¾', '¡', '¿'], // implausible after alphabetic
    &['¬', '±', '×', '÷', '≈', '≤', '≥'], // implausible next to non-ASCII alphabetic
	&['ª', 'º'], // ordinal indicators
];

pub const GREEK: [&[char]; 41] = [
	&[' '], // Space-like
    &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'],
    &['·', '´', '\u{00AD}', '¦', '∙', '―', '¬', '±', '×', '÷', '≈', '≤', '≥'], // plausible next to alphabetic on either side (acute/tonos and math operators deliberately here)
    &['¨', '¯', '¸', '˛', 'ˇ', '˘', '΅', 'ͺ', '¤', '§', '¢', 'ƒ', '£', '¥', '€', '№', '⌡', '⌠', '₯'], // implausible next to alphabetic on either side
    &['®', '¶', '¹', '²', '³', '»', '’'], // implausible before alphabetic
    &['©', '°', 'µ', '¼', '½', '¾', '¡', '¿', '«', '‘'], // implausible after alphabetic
 	&['ΐ', 'ΰ'],
	&['ά'],
	&['έ'],
	&['ή'],
	&['ί'],
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

pub const TURKISH: [&[char]; 44] = [
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
	&['ı'],
	&['i'], // Dotted i treated as non-ASCII
	&['ß', 'ñ', 'š', 'œ', 'å', 'à', 'á', 'ã', 'æ', 'è', 'é', 'ë', 'ì', 'í', 'ï', 'ò', 'ó', 'ô', 'õ', 'ø', 'ù', 'ú', 'ÿ'], // Unused
	&['ä'], // Used for windows-1254-compatible Azeri spelling
	&['â'],
	&['ç'],
	&['ê'],
	&['î'],
	&['ğ'],
	&['ö'],
	&['û'],
	&['ü'],
	&['ş'],
    &['·', '\u{00AD}', '¦', '∙'], // plausible next to alphabetic on either side
    &['¨', '¯', '´', '¸', '˛', 'ˇ', '˘', '¤', '§', '¢', 'ƒ', '£', '¥', '№', '⌡', '⌠', 'ª', 'º'], // implausible next to alphabetic on either side (ordinal indicators deliberately here)
    &['®', '¶', '¹', '²', '³', '»'], // implausible before alphabetic
    &['©', '°', 'µ', '¼', '½', '¾', '¡', '¿', '«'], // implausible after alphabetic
    &['¬', '±', '×', '÷', '≈', '≤', '≥'], // implausible next to non-ASCII alphabetic
];

pub const HEBREW: [&[char]; 42] = [
	&[' '], // Space-like
    &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'],
    &['·', '\u{00AD}', '¦', '∙', '‗', '׀', '־'], // plausible next to alphabetic on either side
    &['¨', '¯', '´', '¸', '˛', 'ˇ', '˘', '¤', '§', '¢', 'ƒ', '£', '¥', '№', '⌡', '⌠', '₪', '\u{200E}', '\u{200F}', '\u{05BF}'], // implausible next to alphabetic on either side
    &['®', '¶', '¹', '²', '³'], // implausible before alphabetic
    &['©', '°', 'µ', '¼', '½', '¾', '¡', '¿'], // implausible after alphabetic
    &['¬', '±', '×', '÷', '≈', '≤', '≥', '«', '»'], // implausible next to non-ASCII alphabetic
	&['״', '׳'], // implausible next to ASCII alphabetic
	&['ְ', 'ֱ', 'ֲ', 'ֳ', 'ִ', 'ֵ', 'ֶ', 'ֹ', 'ֺ', 'ֻ', 'ֽ', 'ׁ', 'ׂ'],
	&['ַ'], // Don't group
	&['ָ'], // Don't group
	&['ּ'], // Don't group
	&['װ'],
	&['ױ'],
	&['ײ'],
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

pub const ARABIC: [&[char]; 59] = [
	&[' '], // Space-like
    &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'à', 'â', 'ç', 'è', 'é', 'ê', 'ë', 'î', 'ï', 'ô', 'ù', 'û', 'ü', 'œ', 'À', 'Â', 'Ç', 'È', 'É', 'Ê', 'Ë', 'Î', 'Ï', 'Ô', 'Ù', 'Û', 'Ü', 'Œ'],
	&['\u{200C}'], // ZWNJ
    &['؛', '؟', '،'], // punctuation, assigned to letters in windows-1251
    &['·', '\u{00AD}', '¦', '∙', '\u{200D}'], // plausible next to alphabetic on either side
    &['¨', '¯', '´', '¸', '˛', 'ˇ', '˘', '¤', '§', '¢', 'ƒ', '£', '¥', '№', '⌡', '⌠', '\u{200E}', '\u{200F}'], // implausible next to alphabetic on either side
    &['®', '¶', '¹', '²', '³', '»'], // implausible before alphabetic
    &['©', '°', 'µ', '¼', '½', '¾', '¡', '¿', '«'], // implausible after alphabetic
    &['¬', '±', '×', '÷', '≈', '≤', '≥'], // implausible next to non-ASCII alphabetic
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
	&['ً', 'ٌ', 'ٍ', 'َ', 'ُ', 'ِ', 'ّ', 'ْ'],
	&['ے'],
];

pub const BALTIC: [&[char]; 51] = [
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
	&['ß', 'ć', 'ź', 'ń', 'ł', 'ś', 'ż', 'ĸ', 'ŧ', 'ŋ', 'đ', 'ø', 'æ', 'å', 'ó', 'ĩ', 'ô', 'ú', 'ä', 'õ', 'ö', 'ü', 'û', 'ũ', 'í', 'î', 'ë', 'ã', 'â', 'á', 'é'], // Unused
	&['ŗ'], //
	&['ą'], //
	&['į'], //
	&['ā'], // 
	&['ę'], //
	&['ē'], //
	&['č'], //
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

pub const VIETNAMESE: [&[char]; 57] = [
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
	&['ß', 'ç', 'ñ', 'œ', 'ä', 'å', 'æ', 'ë', 'ö', 'ø', 'ü'], // Foreign letters
	&['̀'],
	&['̉'],
	&['̃'],
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

pub const THAI: [&[char]; 73] = [
	&[' '], // Space-like
    &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'],
	&['€', '฿', '๚', '๛', '๏', '๐', '๑', '๒', '๓', '๔', '๕', '๖', '๗', '๘', '๙'],
	&['ก'], // Lead for CJK punctuation
	&['ข'], // GB numbers
	&['ค'], // EUC-JP Hiragana lead
	&['ฆ'], // Greek, parantheses
	&['ง'], // Cyrillic
	&['จ'], // Pinyin
	&['ฉ'], // Box drawing
	&['ช'],
	&['ซ'],
	&['ฌ'],
	&['ญ'],
	&['ฎ'],
	&['ฏ'], // Last IBM866 lower case
	&['ฐ'], // Level 1 start
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
	&['ภ'], // Windows upper case and KOI lower case start
	&['ม'],
	&['ย'],
	&['ร'],
	&['ฤ'],
	&['ล'],
	&['ว'],
	&['ศ'], // Last Hangul
	&['ษ'],
	&['ส'],
	&['ห'],
	&['ฬ'],
	&['อ'],
	&['ฮ'],
	&['ฯ'], // Last EUC-JP Level 1
	&['ะ'], // First ISO-8859-5 lower case
	&['ั'],
	&['า'],
	&['ำ'],
	&['ิ'],
	&['ี'],
	&['ึ'],
	&['ื'], // Last GB Level 1
	&['ุ'],
	&['ู'],
	&['ฺ'],
	&['เ'], // Windows lowel case and KOI upper case start
	&['แ'],
	&['โ'],
	&['ใ'],
	&['ไ'],
	&['ๆ'],
	&['็'],
	&['่'],
	&['้'],
	&['๊'],
	&['๋'],
	&['์'],
	&['ํ'],
	&['๎'],
	&['ๅ', 'ฦ', 'ฅ', 'ฃ'], // Obsolete (contains EUC-JP Katakana lead, full-width ASCII)
];
