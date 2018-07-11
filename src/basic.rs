//! Basic Latin. `U+0000` - `U+007F`
use super::{FontUtf16, Utf16Fonts, legacy::BASIC_LEGACY};

use core::fmt;

/// A constant `[FontUtf16; 128]`, for Basic Latin fonts (`U+0000` - `U+007F`).
///
/// ## `BASIC_UTF16[0]`: `U+0000` " "
/// ## `BASIC_UTF16[1]`: `U+0001` " "
/// ## `BASIC_UTF16[2]`: `U+0002` " "
/// ## `BASIC_UTF16[3]`: `U+0003` " "
/// ## `BASIC_UTF16[4]`: `U+0004` " "
/// ## `BASIC_UTF16[5]`: `U+0005` " "
/// ## `BASIC_UTF16[6]`: `U+0006` " "
/// ## `BASIC_UTF16[7]`: `U+0007` " "
/// ## `BASIC_UTF16[8]`: `U+0008` " "
/// ## `BASIC_UTF16[9]`: `U+0009` " "
/// ## `BASIC_UTF16[10]`: `U+000A` " "
/// ## `BASIC_UTF16[11]`: `U+000B` " "
/// ## `BASIC_UTF16[12]`: `U+000C` " "
/// ## `BASIC_UTF16[13]`: `U+000D` " "
/// ## `BASIC_UTF16[14]`: `U+000E` " "
/// ## `BASIC_UTF16[15]`: `U+000F` " "
/// ## `BASIC_UTF16[16]`: `U+0010` " "
/// ## `BASIC_UTF16[17]`: `U+0011` " "
/// ## `BASIC_UTF16[18]`: `U+0012` " "
/// ## `BASIC_UTF16[19]`: `U+0013` " "
/// ## `BASIC_UTF16[20]`: `U+0014` " "
/// ## `BASIC_UTF16[21]`: `U+0015` " "
/// ## `BASIC_UTF16[22]`: `U+0016` " "
/// ## `BASIC_UTF16[23]`: `U+0017` " "
/// ## `BASIC_UTF16[24]`: `U+0018` " "
/// ## `BASIC_UTF16[25]`: `U+0019` " "
/// ## `BASIC_UTF16[26]`: `U+001A` " "
/// ## `BASIC_UTF16[27]`: `U+001B` " "
/// ## `BASIC_UTF16[28]`: `U+001C` " "
/// ## `BASIC_UTF16[29]`: `U+001D` " "
/// ## `BASIC_UTF16[30]`: `U+001E` " "
/// ## `BASIC_UTF16[31]`: `U+001F` " "
/// ## `BASIC_UTF16[32]`: `U+0020` " "
/// ## `BASIC_UTF16[33]`: `U+0021` `"!"`
///
/// ```text
/// ░░░██░░░
/// ░░████░░
/// ░░████░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[34]`: `U+0022` `"\""`
///
/// ```text
/// ░██░██░░
/// ░██░██░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[35]`: `U+0023` `"#"`
///
/// ```text
/// ░██░██░░
/// ░██░██░░
/// ███████░
/// ░██░██░░
/// ███████░
/// ░██░██░░
/// ░██░██░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[36]`: `U+0024` `"$"`
///
/// ```text
/// ░░██░░░░
/// ░█████░░
/// ██░░░░░░
/// ░████░░░
/// ░░░░██░░
/// █████░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[37]`: `U+0025` `"%"`
///
/// ```text
/// ░░░░░░░░
/// ██░░░██░
/// ██░░██░░
/// ░░░██░░░
/// ░░██░░░░
/// ░██░░██░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[38]`: `U+0026` `"&"`
///
/// ```text
/// ░░███░░░
/// ░██░██░░
/// ░░███░░░
/// ░███░██░
/// ██░███░░
/// ██░░██░░
/// ░███░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[39]`: `U+0027` `"\'"`
///
/// ```text
/// ░██░░░░░
/// ░██░░░░░
/// ██░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[40]`: `U+0028` `"("`
///
/// ```text
/// ░░░██░░░
/// ░░██░░░░
/// ░██░░░░░
/// ░██░░░░░
/// ░██░░░░░
/// ░░██░░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[41]`: `U+0029` `")"`
///
/// ```text
/// ░██░░░░░
/// ░░██░░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░██░░░░
/// ░██░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[42]`: `U+002A` `"*"`
///
/// ```text
/// ░░░░░░░░
/// ░██░░██░
/// ░░████░░
/// ████████
/// ░░████░░
/// ░██░░██░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[43]`: `U+002B` `"+"`
///
/// ```text
/// ░░░░░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ██████░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[44]`: `U+002C` `","`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░██░░░░░
/// ```
///
/// ## `BASIC_UTF16[45]`: `U+002D` `"-"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ██████░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[46]`: `U+002E` `"."`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[47]`: `U+002F` `"/"`
///
/// ```text
/// ░░░░░██░
/// ░░░░██░░
/// ░░░██░░░
/// ░░██░░░░
/// ░██░░░░░
/// ██░░░░░░
/// █░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[48]`: `U+0030` `"0"`
///
/// ```text
/// ░█████░░
/// ██░░░██░
/// ██░░███░
/// ██░████░
/// ████░██░
/// ███░░██░
/// ░█████░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[49]`: `U+0031` `"1"`
///
/// ```text
/// ░░██░░░░
/// ░███░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ██████░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[50]`: `U+0032` `"2"`
///
/// ```text
/// ░████░░░
/// ██░░██░░
/// ░░░░██░░
/// ░░███░░░
/// ░██░░░░░
/// ██░░██░░
/// ██████░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[51]`: `U+0033` `"3"`
///
/// ```text
/// ░████░░░
/// ██░░██░░
/// ░░░░██░░
/// ░░███░░░
/// ░░░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[52]`: `U+0034` `"4"`
///
/// ```text
/// ░░░███░░
/// ░░████░░
/// ░██░██░░
/// ██░░██░░
/// ███████░
/// ░░░░██░░
/// ░░░████░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[53]`: `U+0035` `"5"`
///
/// ```text
/// ██████░░
/// ██░░░░░░
/// █████░░░
/// ░░░░██░░
/// ░░░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[54]`: `U+0036` `"6"`
///
/// ```text
/// ░░███░░░
/// ░██░░░░░
/// ██░░░░░░
/// █████░░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[55]`: `U+0037` `"7"`
///
/// ```text
/// ██████░░
/// ██░░██░░
/// ░░░░██░░
/// ░░░██░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[56]`: `U+0038` `"8"`
///
/// ```text
/// ░████░░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[57]`: `U+0039` `"9"`
///
/// ```text
/// ░████░░░
/// ██░░██░░
/// ██░░██░░
/// ░█████░░
/// ░░░░██░░
/// ░░░██░░░
/// ░███░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[58]`: `U+003A` `":"`
///
/// ```text
/// ░░░░░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[59]`: `U+003B` `";"`
///
/// ```text
/// ░░░░░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░██░░░░░
/// ```
///
/// ## `BASIC_UTF16[60]`: `U+003C` `"<"`
///
/// ```text
/// ░░░██░░░
/// ░░██░░░░
/// ░██░░░░░
/// ██░░░░░░
/// ░██░░░░░
/// ░░██░░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[61]`: `U+003D` `"="`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██████░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ██████░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[62]`: `U+003E` `">"`
///
/// ```text
/// ░██░░░░░
/// ░░██░░░░
/// ░░░██░░░
/// ░░░░██░░
/// ░░░██░░░
/// ░░██░░░░
/// ░██░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[63]`: `U+003F` `"?"`
///
/// ```text
/// ░████░░░
/// ██░░██░░
/// ░░░░██░░
/// ░░░██░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[64]`: `U+0040` `"@"`
///
/// ```text
/// ░█████░░
/// ██░░░██░
/// ██░████░
/// ██░████░
/// ██░████░
/// ██░░░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[65]`: `U+0041` `"A"`
///
/// ```text
/// ░░██░░░░
/// ░████░░░
/// ██░░██░░
/// ██░░██░░
/// ██████░░
/// ██░░██░░
/// ██░░██░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[66]`: `U+0042` `"B"`
///
/// ```text
/// ██████░░
/// ░██░░██░
/// ░██░░██░
/// ░█████░░
/// ░██░░██░
/// ░██░░██░
/// ██████░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[67]`: `U+0043` `"C"`
///
/// ```text
/// ░░████░░
/// ░██░░██░
/// ██░░░░░░
/// ██░░░░░░
/// ██░░░░░░
/// ░██░░██░
/// ░░████░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[68]`: `U+0044` `"D"`
///
/// ```text
/// █████░░░
/// ░██░██░░
/// ░██░░██░
/// ░██░░██░
/// ░██░░██░
/// ░██░██░░
/// █████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[69]`: `U+0045` `"E"`
///
/// ```text
/// ███████░
/// ░██░░░█░
/// ░██░█░░░
/// ░████░░░
/// ░██░█░░░
/// ░██░░░█░
/// ███████░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[70]`: `U+0046` `"F"`
///
/// ```text
/// ███████░
/// ░██░░░█░
/// ░██░█░░░
/// ░████░░░
/// ░██░█░░░
/// ░██░░░░░
/// ████░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[71]`: `U+0047` `"G"`
///
/// ```text
/// ░░████░░
/// ░██░░██░
/// ██░░░░░░
/// ██░░░░░░
/// ██░░███░
/// ░██░░██░
/// ░░█████░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[72]`: `U+0048` `"H"`
///
/// ```text
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██████░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[73]`: `U+0049` `"I"`
///
/// ```text
/// ░████░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[74]`: `U+004A` `"J"`
///
/// ```text
/// ░░░████░
/// ░░░░██░░
/// ░░░░██░░
/// ░░░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[75]`: `U+004B` `"K"`
///
/// ```text
/// ███░░██░
/// ░██░░██░
/// ░██░██░░
/// ░████░░░
/// ░██░██░░
/// ░██░░██░
/// ███░░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[76]`: `U+004C` `"L"`
///
/// ```text
/// ████░░░░
/// ░██░░░░░
/// ░██░░░░░
/// ░██░░░░░
/// ░██░░░█░
/// ░██░░██░
/// ███████░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[77]`: `U+004D` `"M"`
///
/// ```text
/// ██░░░██░
/// ███░███░
/// ███████░
/// ███████░
/// ██░█░██░
/// ██░░░██░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[78]`: `U+004E` `"N"`
///
/// ```text
/// ██░░░██░
/// ███░░██░
/// ████░██░
/// ██░████░
/// ██░░███░
/// ██░░░██░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[79]`: `U+004F` `"O"`
///
/// ```text
/// ░░███░░░
/// ░██░██░░
/// ██░░░██░
/// ██░░░██░
/// ██░░░██░
/// ░██░██░░
/// ░░███░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[80]`: `U+0050` `"P"`
///
/// ```text
/// ██████░░
/// ░██░░██░
/// ░██░░██░
/// ░█████░░
/// ░██░░░░░
/// ░██░░░░░
/// ████░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[81]`: `U+0051` `"Q"`
///
/// ```text
/// ░████░░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██░███░░
/// ░████░░░
/// ░░░███░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[82]`: `U+0052` `"R"`
///
/// ```text
/// ██████░░
/// ░██░░██░
/// ░██░░██░
/// ░█████░░
/// ░██░██░░
/// ░██░░██░
/// ███░░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[83]`: `U+0053` `"S"`
///
/// ```text
/// ░████░░░
/// ██░░██░░
/// ███░░░░░
/// ░███░░░░
/// ░░░███░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[84]`: `U+0054` `"T"`
///
/// ```text
/// ██████░░
/// █░██░█░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[85]`: `U+0055` `"U"`
///
/// ```text
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██████░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[86]`: `U+0056` `"V"`
///
/// ```text
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[87]`: `U+0057` `"W"`
///
/// ```text
/// ██░░░██░
/// ██░░░██░
/// ██░░░██░
/// ██░█░██░
/// ███████░
/// ███░███░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[88]`: `U+0058` `"X"`
///
/// ```text
/// ██░░░██░
/// ██░░░██░
/// ░██░██░░
/// ░░███░░░
/// ░░███░░░
/// ░██░██░░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[89]`: `U+0059` `"Y"`
///
/// ```text
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[90]`: `U+005A` `"Z"`
///
/// ```text
/// ███████░
/// ██░░░██░
/// █░░░██░░
/// ░░░██░░░
/// ░░██░░█░
/// ░██░░██░
/// ███████░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[91]`: `U+005B` `"["`
///
/// ```text
/// ░████░░░
/// ░██░░░░░
/// ░██░░░░░
/// ░██░░░░░
/// ░██░░░░░
/// ░██░░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[92]`: `U+005C` `"\\"`
///
/// ```text
/// ██░░░░░░
/// ░██░░░░░
/// ░░██░░░░
/// ░░░██░░░
/// ░░░░██░░
/// ░░░░░██░
/// ░░░░░░█░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[93]`: `U+005D` `"]"`
///
/// ```text
/// ░████░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[94]`: `U+005E` `"^"`
///
/// ```text
/// ░░░█░░░░
/// ░░███░░░
/// ░██░██░░
/// ██░░░██░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[95]`: `U+005F` `"_"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████████
/// ```
///
/// ## `BASIC_UTF16[96]`: `U+0060` `"`"`
///
/// ```text
/// ░░██░░░░
/// ░░██░░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[97]`: `U+0061` `"a"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░████░░░
/// ░░░░██░░
/// ░█████░░
/// ██░░██░░
/// ░███░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[98]`: `U+0062` `"b"`
///
/// ```text
/// ███░░░░░
/// ░██░░░░░
/// ░██░░░░░
/// ░█████░░
/// ░██░░██░
/// ░██░░██░
/// ██░███░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[99]`: `U+0063` `"c"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░████░░░
/// ██░░██░░
/// ██░░░░░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[100]`: `U+0064` `"d"`
///
/// ```text
/// ░░░███░░
/// ░░░░██░░
/// ░░░░██░░
/// ░█████░░
/// ██░░██░░
/// ██░░██░░
/// ░███░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[101]`: `U+0065` `"e"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░████░░░
/// ██░░██░░
/// ██████░░
/// ██░░░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[102]`: `U+0066` `"f"`
///
/// ```text
/// ░░███░░░
/// ░██░██░░
/// ░██░░░░░
/// ████░░░░
/// ░██░░░░░
/// ░██░░░░░
/// ████░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[103]`: `U+0067` `"g"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░███░██░
/// ██░░██░░
/// ██░░██░░
/// ░█████░░
/// ░░░░██░░
/// █████░░░
/// ```
///
/// ## `BASIC_UTF16[104]`: `U+0068` `"h"`
///
/// ```text
/// ███░░░░░
/// ░██░░░░░
/// ░██░██░░
/// ░███░██░
/// ░██░░██░
/// ░██░░██░
/// ███░░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[105]`: `U+0069` `"i"`
///
/// ```text
/// ░░██░░░░
/// ░░░░░░░░
/// ░███░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[106]`: `U+006A` `"j"`
///
/// ```text
/// ░░░░██░░
/// ░░░░░░░░
/// ░░░░██░░
/// ░░░░██░░
/// ░░░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ```
///
/// ## `BASIC_UTF16[107]`: `U+006B` `"k"`
///
/// ```text
/// ███░░░░░
/// ░██░░░░░
/// ░██░░██░
/// ░██░██░░
/// ░████░░░
/// ░██░██░░
/// ███░░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[108]`: `U+006C` `"l"`
///
/// ```text
/// ░███░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[109]`: `U+006D` `"m"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░░██░░
/// ███████░
/// ███████░
/// ██░█░██░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[110]`: `U+006E` `"n"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// █████░░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[111]`: `U+006F` `"o"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░████░░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[112]`: `U+0070` `"p"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░███░░
/// ░██░░██░
/// ░██░░██░
/// ░█████░░
/// ░██░░░░░
/// ████░░░░
/// ```
///
/// ## `BASIC_UTF16[113]`: `U+0071` `"q"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░███░██░
/// ██░░██░░
/// ██░░██░░
/// ░█████░░
/// ░░░░██░░
/// ░░░████░
/// ```
///
/// ## `BASIC_UTF16[114]`: `U+0072` `"r"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░███░░
/// ░███░██░
/// ░██░░██░
/// ░██░░░░░
/// ████░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[115]`: `U+0073` `"s"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░█████░░
/// ██░░░░░░
/// ░████░░░
/// ░░░░██░░
/// █████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[116]`: `U+0074` `"t"`
///
/// ```text
/// ░░░█░░░░
/// ░░██░░░░
/// ░█████░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░█░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[117]`: `U+0075` `"u"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░███░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[118]`: `U+0076` `"v"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[119]`: `U+0077` `"w"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░░░██░
/// ██░█░██░
/// ███████░
/// ███████░
/// ░██░██░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[120]`: `U+0078` `"x"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░░░██░
/// ░██░██░░
/// ░░███░░░
/// ░██░██░░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[121]`: `U+0079` `"y"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░█████░░
/// ░░░░██░░
/// █████░░░
/// ```
///
/// ## `BASIC_UTF16[122]`: `U+007A` `"z"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██████░░
/// █░░██░░░
/// ░░██░░░░
/// ░██░░█░░
/// ██████░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[123]`: `U+007B` `"{"`
///
/// ```text
/// ░░░███░░
/// ░░██░░░░
/// ░░██░░░░
/// ███░░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░░███░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[124]`: `U+007C` `"|"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[125]`: `U+007D` `"}"`
///
/// ```text
/// ███░░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░░███░░
/// ░░██░░░░
/// ░░██░░░░
/// ███░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UTF16[126]`: `U+007E` `"~"`
///
/// ```text
/// ░███░██░
/// ██░███░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
pub const BASIC_UTF16: [FontUtf16; 128] = [
    FontUtf16(0x0000 as u16, BASIC_LEGACY[0]),
    FontUtf16(0x0001 as u16, BASIC_LEGACY[1]),
    FontUtf16(0x0002 as u16, BASIC_LEGACY[2]),
    FontUtf16(0x0003 as u16, BASIC_LEGACY[3]),
    FontUtf16(0x0004 as u16, BASIC_LEGACY[4]),
    FontUtf16(0x0005 as u16, BASIC_LEGACY[5]),
    FontUtf16(0x0006 as u16, BASIC_LEGACY[6]),
    FontUtf16(0x0007 as u16, BASIC_LEGACY[7]),
    FontUtf16(0x0008 as u16, BASIC_LEGACY[8]),
    FontUtf16(0x0009 as u16, BASIC_LEGACY[9]),
    FontUtf16(0x000A as u16, BASIC_LEGACY[10]),
    FontUtf16(0x000B as u16, BASIC_LEGACY[11]),
    FontUtf16(0x000C as u16, BASIC_LEGACY[12]),
    FontUtf16(0x000D as u16, BASIC_LEGACY[13]),
    FontUtf16(0x000E as u16, BASIC_LEGACY[14]),
    FontUtf16(0x000F as u16, BASIC_LEGACY[15]),
    FontUtf16(0x0010 as u16, BASIC_LEGACY[16]),
    FontUtf16(0x0011 as u16, BASIC_LEGACY[17]),
    FontUtf16(0x0012 as u16, BASIC_LEGACY[18]),
    FontUtf16(0x0013 as u16, BASIC_LEGACY[19]),
    FontUtf16(0x0014 as u16, BASIC_LEGACY[20]),
    FontUtf16(0x0015 as u16, BASIC_LEGACY[21]),
    FontUtf16(0x0016 as u16, BASIC_LEGACY[22]),
    FontUtf16(0x0017 as u16, BASIC_LEGACY[23]),
    FontUtf16(0x0018 as u16, BASIC_LEGACY[24]),
    FontUtf16(0x0019 as u16, BASIC_LEGACY[25]),
    FontUtf16(0x001A as u16, BASIC_LEGACY[26]),
    FontUtf16(0x001B as u16, BASIC_LEGACY[27]),
    FontUtf16(0x001C as u16, BASIC_LEGACY[28]),
    FontUtf16(0x001D as u16, BASIC_LEGACY[29]),
    FontUtf16(0x001E as u16, BASIC_LEGACY[30]),
    FontUtf16(0x001F as u16, BASIC_LEGACY[31]),
    FontUtf16(0x0020 as u16, BASIC_LEGACY[32]),
    FontUtf16(0x0021 as u16, BASIC_LEGACY[33]),
    FontUtf16(0x0022 as u16, BASIC_LEGACY[34]),
    FontUtf16(0x0023 as u16, BASIC_LEGACY[35]),
    FontUtf16(0x0024 as u16, BASIC_LEGACY[36]),
    FontUtf16(0x0025 as u16, BASIC_LEGACY[37]),
    FontUtf16(0x0026 as u16, BASIC_LEGACY[38]),
    FontUtf16(0x0027 as u16, BASIC_LEGACY[39]),
    FontUtf16(0x0028 as u16, BASIC_LEGACY[40]),
    FontUtf16(0x0029 as u16, BASIC_LEGACY[41]),
    FontUtf16(0x002A as u16, BASIC_LEGACY[42]),
    FontUtf16(0x002B as u16, BASIC_LEGACY[43]),
    FontUtf16(0x002C as u16, BASIC_LEGACY[44]),
    FontUtf16(0x002D as u16, BASIC_LEGACY[45]),
    FontUtf16(0x002E as u16, BASIC_LEGACY[46]),
    FontUtf16(0x002F as u16, BASIC_LEGACY[47]),
    FontUtf16(0x0030 as u16, BASIC_LEGACY[48]),
    FontUtf16(0x0031 as u16, BASIC_LEGACY[49]),
    FontUtf16(0x0032 as u16, BASIC_LEGACY[50]),
    FontUtf16(0x0033 as u16, BASIC_LEGACY[51]),
    FontUtf16(0x0034 as u16, BASIC_LEGACY[52]),
    FontUtf16(0x0035 as u16, BASIC_LEGACY[53]),
    FontUtf16(0x0036 as u16, BASIC_LEGACY[54]),
    FontUtf16(0x0037 as u16, BASIC_LEGACY[55]),
    FontUtf16(0x0038 as u16, BASIC_LEGACY[56]),
    FontUtf16(0x0039 as u16, BASIC_LEGACY[57]),
    FontUtf16(0x003A as u16, BASIC_LEGACY[58]),
    FontUtf16(0x003B as u16, BASIC_LEGACY[59]),
    FontUtf16(0x003C as u16, BASIC_LEGACY[60]),
    FontUtf16(0x003D as u16, BASIC_LEGACY[61]),
    FontUtf16(0x003E as u16, BASIC_LEGACY[62]),
    FontUtf16(0x003F as u16, BASIC_LEGACY[63]),
    FontUtf16(0x0040 as u16, BASIC_LEGACY[64]),
    FontUtf16(0x0041 as u16, BASIC_LEGACY[65]),
    FontUtf16(0x0042 as u16, BASIC_LEGACY[66]),
    FontUtf16(0x0043 as u16, BASIC_LEGACY[67]),
    FontUtf16(0x0044 as u16, BASIC_LEGACY[68]),
    FontUtf16(0x0045 as u16, BASIC_LEGACY[69]),
    FontUtf16(0x0046 as u16, BASIC_LEGACY[70]),
    FontUtf16(0x0047 as u16, BASIC_LEGACY[71]),
    FontUtf16(0x0048 as u16, BASIC_LEGACY[72]),
    FontUtf16(0x0049 as u16, BASIC_LEGACY[73]),
    FontUtf16(0x004A as u16, BASIC_LEGACY[74]),
    FontUtf16(0x004B as u16, BASIC_LEGACY[75]),
    FontUtf16(0x004C as u16, BASIC_LEGACY[76]),
    FontUtf16(0x004D as u16, BASIC_LEGACY[77]),
    FontUtf16(0x004E as u16, BASIC_LEGACY[78]),
    FontUtf16(0x004F as u16, BASIC_LEGACY[79]),
    FontUtf16(0x0050 as u16, BASIC_LEGACY[80]),
    FontUtf16(0x0051 as u16, BASIC_LEGACY[81]),
    FontUtf16(0x0052 as u16, BASIC_LEGACY[82]),
    FontUtf16(0x0053 as u16, BASIC_LEGACY[83]),
    FontUtf16(0x0054 as u16, BASIC_LEGACY[84]),
    FontUtf16(0x0055 as u16, BASIC_LEGACY[85]),
    FontUtf16(0x0056 as u16, BASIC_LEGACY[86]),
    FontUtf16(0x0057 as u16, BASIC_LEGACY[87]),
    FontUtf16(0x0058 as u16, BASIC_LEGACY[88]),
    FontUtf16(0x0059 as u16, BASIC_LEGACY[89]),
    FontUtf16(0x005A as u16, BASIC_LEGACY[90]),
    FontUtf16(0x005B as u16, BASIC_LEGACY[91]),
    FontUtf16(0x005C as u16, BASIC_LEGACY[92]),
    FontUtf16(0x005D as u16, BASIC_LEGACY[93]),
    FontUtf16(0x005E as u16, BASIC_LEGACY[94]),
    FontUtf16(0x005F as u16, BASIC_LEGACY[95]),
    FontUtf16(0x0060 as u16, BASIC_LEGACY[96]),
    FontUtf16(0x0061 as u16, BASIC_LEGACY[97]),
    FontUtf16(0x0062 as u16, BASIC_LEGACY[98]),
    FontUtf16(0x0063 as u16, BASIC_LEGACY[99]),
    FontUtf16(0x0064 as u16, BASIC_LEGACY[100]),
    FontUtf16(0x0065 as u16, BASIC_LEGACY[101]),
    FontUtf16(0x0066 as u16, BASIC_LEGACY[102]),
    FontUtf16(0x0067 as u16, BASIC_LEGACY[103]),
    FontUtf16(0x0068 as u16, BASIC_LEGACY[104]),
    FontUtf16(0x0069 as u16, BASIC_LEGACY[105]),
    FontUtf16(0x006A as u16, BASIC_LEGACY[106]),
    FontUtf16(0x006B as u16, BASIC_LEGACY[107]),
    FontUtf16(0x006C as u16, BASIC_LEGACY[108]),
    FontUtf16(0x006D as u16, BASIC_LEGACY[109]),
    FontUtf16(0x006E as u16, BASIC_LEGACY[110]),
    FontUtf16(0x006F as u16, BASIC_LEGACY[111]),
    FontUtf16(0x0070 as u16, BASIC_LEGACY[112]),
    FontUtf16(0x0071 as u16, BASIC_LEGACY[113]),
    FontUtf16(0x0072 as u16, BASIC_LEGACY[114]),
    FontUtf16(0x0073 as u16, BASIC_LEGACY[115]),
    FontUtf16(0x0074 as u16, BASIC_LEGACY[116]),
    FontUtf16(0x0075 as u16, BASIC_LEGACY[117]),
    FontUtf16(0x0076 as u16, BASIC_LEGACY[118]),
    FontUtf16(0x0077 as u16, BASIC_LEGACY[119]),
    FontUtf16(0x0078 as u16, BASIC_LEGACY[120]),
    FontUtf16(0x0079 as u16, BASIC_LEGACY[121]),
    FontUtf16(0x007A as u16, BASIC_LEGACY[122]),
    FontUtf16(0x007B as u16, BASIC_LEGACY[123]),
    FontUtf16(0x007C as u16, BASIC_LEGACY[124]),
    FontUtf16(0x007D as u16, BASIC_LEGACY[125]),
    FontUtf16(0x007E as u16, BASIC_LEGACY[126]),
    FontUtf16(0x007F as u16, BASIC_LEGACY[127]),
];

/// A convenient constant for Basic Latin fonts (`U+0000` - `U+007F`), that implements the [`Utf16Fonts`](./utf16/trait.Utf16Fonts.html) trait.
pub const BASIC_FONTS: BasicFonts = BasicFonts(BASIC_UTF16);

/// Strong-typed collection wrapper for [BASIC_UTF16](./constant.BASIC_UTF16.html).
pub struct BasicFonts([FontUtf16; 128]);

impl fmt::Debug for BasicFonts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", stringify!(BASIC_UTF16))
    }
}

impl PartialEq for BasicFonts {
    fn eq(&self, other: &BasicFonts) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(true, |eq, (a, b)| eq && a == b)
    }
}

impl BasicFonts {
    /// Create a new collection of `BASIC_UTF16` fonts.
    pub fn new() -> Self {
        BasicFonts(BASIC_UTF16)
    }
}

impl Default for BasicFonts {
    fn default() -> Self {
        BasicFonts::new()
    }
}

impl Utf16Fonts for BasicFonts {
    fn get(&self, key: u16) -> Option<[u8; 8]> {
        match self.get_font(key) {
            Some(font) => Some(font.into()),
            None => None,
        }
    }

    fn get_font(&self, key: u16) -> Option<FontUtf16> {
        match self.0.binary_search_by_key(&key, |&f| f.utf16()) {
            Ok(idx) => Some(self.0[idx]),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_set_implements_default_trait_with_method_new() {
        let basic_set: BasicFonts = Default::default();
        assert_eq!(basic_set, BasicFonts::new());
    }

    #[test]
    fn basic_fonts_constant_is_equal_to_a_new_instance() {
        assert_eq!(BASIC_FONTS, BasicFonts::new());
    }

    #[test]
    fn basic_fonts_constant_wraps_basic_utf16_constant() {
        let basic = BasicFonts::new();
        assert!(basic.0.len() == BASIC_UTF16.len());
        for (idx, font) in basic.0.iter().enumerate() {
            assert_eq!(font, &BASIC_UTF16[idx]);
        }
    }
}
