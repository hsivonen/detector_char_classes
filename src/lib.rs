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

pub const CENTRAL: [&[char]; 76] = [
	&[' '], // Space-like
    &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], // ASCII digits
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
    &['«', '»'], // guillemets, assigned to letters in windows-874
    &['\u{00AD}'], // shy
	&['¦', '©', '®', '°', 'µ', '¶', '·'], // symbols plausible next to letter, assigned to letters in windows-874
	&['¤', '§', '¬', '±', '¢'], // symbols implausible next to letter, assigned to letters in windows-874
	&['¨', '¯', '´', '¸', '˛', 'ˇ', '˘'], // lone accents, assigned to letters in windows-874
    &['×', '÷'], // math operators, assigned to letters in windows-1251
    &['ƒ'], // kana lead byte in Shift_JIS
];

pub const CYRILLIC: [&[char]; 55] = [
	&[' '], // Space-like
    &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], // ASCII digits
    &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'],
	&['\u{00AD}'], // shy, assigned to letter in windows-874
    &['«', '»'], // guillemets, assigned to letters in windows-874
	&['¦', '©', '®', '°', 'µ', '¶', '·', '∙', '²'], // symbols plausible next to letter, assigned to letters in windows-874
	&['¤', '§', '¬', '±', '¢', '№', '×', '÷', '⌡', '⌠', '≈', '≤', '≥'], // symbols implausible next to letter, assigned to letters in windows-874
	&['─', '│', '┌', '┐', '└', '┘', '├', '┤', '┬', '┴', '┼', '═', '║', '╒', '╓', '╔', '╕', '╖', '╗', '╘', '╙', '╚', '╛', '╜', '╝', '╞', '╟', '╠', '╡', '╢', '╣', '╤', '╥', '╦', '╧', '╨', '╩', '╪', '╫', '╬', '▀', '▄', '█', '▌', '▐', '░', '▒', '▓', '■'], // box drawing
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

pub const WESTERN: [&[char]; 74] = [
	&[' '], // Space-like
    &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], // ASCII digits
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
	&['ž'],
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
	&['ð'],
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
	&['ý'],
	&['þ'],
	&['ÿ'],
	&['¼', '½', '¾'], // fractions, ¼ and ¾ assigned to letters in windows-1250
	&['ª', 'º'], // ordinal indicators, assigned to letters in windows-1250
	&['¹', '²', '³'], // superscripts, ¹ and ³ assigned to letters in windows-1250
    &['¡', '¿'], // Leading punctuation, ¿ assigned to letter in windows-1250
    &['£', '¥', '¢'], // Prefixed currency symbols (euro is deliberately not here), assigned to letters in windows-1250
    &['«', '»'], // guillemets, assigned to letters in windows-874
    &['\u{00AD}'], // shy
	&['¦', '©', '®', '°', 'µ', '¶', '·'], // symbols plausible next to letter, assigned to letters in windows-874
	&['¤', '§', '¬', '±'], // symbols implausible next to letter, assigned to letters in windows-874
	&['¨', '¯', '´', '¸'], // lone accents, assigned to letters in windows-874
    &['×', '÷'], // math operators, assigned to letters in windows-1251
];

pub const GREEK: [&[char]; 48] = [
	&[' '], // Space-like
    &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], // ASCII digits
    &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'],
	&['¹', '²', '³'], // superscripts, assigned to letters in windows-1251
    &['£', '¥', '€'], // currency symbols, assigned to letters in windows-1251
    &['¨', '΅'], // lone accents, assigned to letters in windows-1251 (acute deliberately grouped with guillements because plausible!)
    &['½'], // fraction, assigned to letters in windows-1251
    &['―'], // horizontal bar, assigned to letters in windows-1251
    &['«', '»', '΄'], // guillemets, assigned to letters in windows-874 (acute grouped here, because a letter followed by a non-combining acute/tonos is a thing in Greek content)
	&['¦', '©', '®', '°', 'µ', '¶', '·'], // symbols plausible next to letter, assigned to letters in windows-874
	&['¤', '§', '¬', '±', '№'], // symbols implausible next to letter, assigned to letters in windows-874
    &['ƒ'], // kana lead byte in Shift_JIS
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

pub const TURKISH: [&[char]; 74] = [
	&[' '], // Space-like
    &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], // ASCII digits
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
	&['ğ'],
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
	&['ı'],
	&['ş'],
	&['ÿ'],
	&['¼', '½', '¾'], // fractions, ¼ and ¾ assigned to letters in windows-1250
	&['ª', 'º'], // ordinal indicators, assigned to letters in windows-1250
	&['¹', '²', '³'], // superscripts, ¹ and ³ assigned to letters in windows-1250
    &['¡', '¿'], // Leading punctuation, ¿ assigned to letter in windows-1250
    &['£', '¥', '¢'], // Prefixed currency symbols (euro is deliberately not here), assigned to letters in windows-1250
    &['«', '»'], // guillemets, assigned to letters in windows-874
    &['\u{00AD}'], // shy
	&['¦', '©', '®', '°', 'µ', '¶', '·'], // symbols plausible next to letter, assigned to letters in windows-874
	&['¤', '§', '¬', '±'], // symbols implausible next to letter, assigned to letters in windows-874
	&['¨', '¯', '´', '¸'], // lone accents, assigned to letters in windows-874
    &['×', '÷'], // math operators, assigned to letters in windows-1251
    &['ƒ'], // kana lead byte in Shift_JIS
];

pub const HEBREW: [&[char]; 63] = [
	&[' '], // Space-like
    &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], // ASCII digits
    &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'],
	&['.', ',', ':', ';', '?', '!'],
	&['₪'], // currency sign
	&['¹', '²', '³'], // superscripts, assigned to letters in windows-1251
    &['¡', '¿'], // Leading punctuation, assigned to letters in windows-1251
    &['£', '¥', '€', '¢'], // currency symbols, assigned to letters in windows-1251
	&['¼', '½', '¾'], // fractions, ¼ and ¾ assigned to letters in windows-1251
    &['¨', '´', '¯', '¸'], // lone accents, assigned to letters in windows-1251
    &['×', '÷'], // math operators, assigned to letters in windows-1251
	&['¦', '©', '®', '°', 'µ', '¶', '·', '«', '»', '‗'], // symbols plausible next to letter, assigned to letters in windows-874
	&['¤', '§', '¬', '±', '№'], // symbols implausible next to letter, assigned to letters in windows-874
    &['ƒ'], // kana lead byte in Shift_JIS
    &['\u{200E}', '\u{200F}'],
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

pub const ARABIC: [&[char]; 72] = [
	&[' '], // Space-like
    &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], // ASCII digits
    &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'à', 'â', 'ç', 'è', 'é', 'ê', 'ë', 'î', 'ï', 'ô', 'ù', 'û', 'ü', 'œ', 'À', 'Â', 'Ç', 'È', 'É', 'Ê', 'Ë', 'Î', 'Ï', 'Ô', 'Ù', 'Û', 'Ü', 'Œ'],
	&['\u{200C}'], // ZWNJ
	&['\u{200D}'], // ZWJ
	&['¹', '²', '³'], // superscripts, assigned to letters in windows-1251
    &['؛', '؟', '،'], // punctuation, assigned to letters in windows-1251
    &['£', '¥', '€', '¢'], // currency symbols, assigned to letters in windows-1251
	&['¼', '½', '¾'], // fractions, ¼ and ¾ assigned to letters in windows-1251
    &['«', '»'], // guillemets, assigned to letters in windows-874
	&['¦', '©', '®', '°', 'µ', '¶', '·'], // symbols plausible next to letter, assigned to letters in windows-874
	&['¤', '§', '¬', '±'], // symbols implausible next to letter, assigned to letters in windows-874
	&['¨', '¯', '´', '¸'], // lone accents, assigned to letters in windows-874
    &['×', '÷'], // math operators, assigned to letters in windows-1251
    &['ƒ'], // kana lead byte in Shift_JIS
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

// pub const ARABIC_FRENCH: [char; 28] = ['à', 'â', 'ç', 'è', 'é', 'ê', 'ë', 'î', 'ï', 'ô', 'ù', 'û', 'ü', 'œ', 'À', 'Â', 'Ç', 'È', 'É', 'Ê', 'Ë', 'Î', 'Ï', 'Ô', 'Ù', 'Û', 'Ü', 'Œ'];

pub const BALTIC: [&[char]; 61] = [
	&[' '], // Space-like
    &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], // ASCII digits
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
	&['q', 'w', 'x'], // Unused ASCII consonants
	&['r'],
	&['s'],
	&['t'],
	&['u'],
	&['v'],
	&['y'], // Unused ASCII vowel
	&['z'],
	&['ø', 'æ', 'å', 'ó'], // Unused non-ASCII vowels
	&['ŗ'], //
	&['ß', 'ć', 'ź', 'ń', 'ł', 'ś', 'ż'], // Unused non-ASCII consonants
	&['ą'], //
	&['į'], //
	&['ā'], // 
	&['ä'], //
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
	&['õ'], //
	&['ö'], //
	&['ų'], //
	&['ū'], //
	&['ü'], //
	&['ž'], //
	&['¼', '½', '¾'], // fractions, ¼ and ¾ assigned to letters in windows-1250
	&['¹', '²', '³'], // superscripts, ¹ and ³ assigned to letters in windows-1250
    &['¡', '¿'], // Leading punctuation, ¿ assigned to letter in windows-1250
    &['£', '¥', '¢'], // Prefixed currency symbols (euro is deliberately not here), assigned to letters in windows-1250
    &['«', '»'], // guillemets, assigned to letters in windows-874
    &['\u{00AD}'], // shy
	&['¦', '©', '®', '°', 'µ', '¶', '·'], // symbols plausible next to letter, assigned to letters in windows-874
	&['¤', '§', '¬', '±'], // symbols implausible next to letter, assigned to letters in windows-874
	&['¨', '¯', '´', '¸'], // lone accents, assigned to letters in windows-874
    &['×', '÷'], // math operators, assigned to letters in windows-1251
];

pub const VIETNAMESE: [&[char]; 75] = [
	&[' '], // Space-like
    &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], // ASCII digits
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
	&['œ'],
	&['̀'],
	&['̉'],
	&['̃'],
	&['ß'],
	&['à'],
	&['á'],
	&['â'],
	&['ă'],
	&['ä'],
	&['å'],
	&['æ'],
	&['ç'],
	&['è'],
	&['é'],
	&['ê'],
	&['ë'],
	&['́'],
	&['í'],
	&['î'],
	&['ï'],
	&['đ'],
	&['ñ'],
	&['̣'],
	&['ó'],
	&['ô'],
	&['ơ'],
	&['ö'],
	&['ø'],
	&['ù'],
	&['ú'],
	&['û'],
	&['ü'],
	&['ư'],
	&['ÿ'],
	&['¼', '½', '¾'], // fractions, ¼ and ¾ assigned to letters in windows-1250
	&['ª', 'º'], // ordinal indicators, assigned to letters in windows-1250
	&['¹', '²', '³'], // superscripts, ¹ and ³ assigned to letters in windows-1250
    &['¡', '¿'], // Leading punctuation, ¿ assigned to letter in windows-1250
    &['£', '¥', '¢'], // Prefixed currency symbols (euro is deliberately not here), assigned to letters in windows-1250
    &['«', '»'], // guillemets, assigned to letters in windows-874
    &['\u{00AD}'], // shy
	&['¦', '©', '®', '°', 'µ', '¶', '·'], // symbols plausible next to letter, assigned to letters in windows-874
	&['¤', '§', '¬', '±'], // symbols implausible next to letter, assigned to letters in windows-874
	&['¨', '¯', '´', '¸'], // lone accents, assigned to letters in windows-874
    &['×', '÷'], // math operators, assigned to letters in windows-1251
    &['ƒ'], // kana lead byte in Shift_JIS
];

pub const THAI: [&[char]; 78] = [
	&[' '], // Space-like
    &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], // ASCII digits
    &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'],
	&['€'], // euro, assigned to letter in windows-1251
	&['๐', '๑', '๒', '๓', '๔', '๕', '๖', '๗', '๘', '๙'],
	&['ก'],
	&['ข'],
	&['ฃ'],
	&['ค'],
	&['ฅ'],
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
	&['ฦ'],
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
	&['ๅ'],
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
