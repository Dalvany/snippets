/// Rust adaptation of Lucene reverse function for [reverse function of ReverseStringFilter](https://github.com/apache/lucene/blob/main/lucene/analysis/common/src/java/org/apache/lucene/analysis/reverse/ReverseStringFilter.java#L130).
fn reverse(text: String) -> String {
    let mut text_utf16: Vec<u16> = text.encode_utf16().collect();
    let len = text_utf16.len();
    if len < 2 {
        return text;
    }
    let mut end = len - 1;
    let mut start: usize = 0;
    let mut front_high = text_utf16[start];
    let mut end_low = text_utf16[end];
    let (mut allow_front_sur, mut allow_end_sur) = (true, true);
    let mid = len >> 1;
    for _i in start..mid {
        let front_low = text_utf16[start + 1];
        let sur_at_front = allow_front_sur
            && front_high >= 0xD800u16
            && front_high <= 0xDBFFu16
            && front_low >= 0xDC00u16
            && front_low <= 0xDFFFu16;
        if sur_at_front && len < 3 {
            return text;
        }
        let end_high = text_utf16[end - 1];
        let sur_at_end = allow_end_sur
            && end_high >= 0xD800u16
            && end_high <= 0xDBFFu16
            && end_low >= 0xDC00u16
            && end_low <= 0xDFFFu16;

        (allow_front_sur, allow_end_sur) = (true, true);

        if sur_at_front == sur_at_end {
            if sur_at_front {
                text_utf16[start] = end_high;
                text_utf16[end] = front_low;
                start = start + 1;
                end = end - 1;
                text_utf16[start] = end_low;
                text_utf16[end] = front_high;
                front_high = text_utf16[start + 1];
                end_low = text_utf16[end - 1];
            } else {
                text_utf16[start] = end_low;
                text_utf16[end] = front_high;
                front_high = front_low;
                end_low = end_high;
            }
        } else {
            if sur_at_front {
                text_utf16[end] = front_low;
                text_utf16[start] = end_low;
                end_low = end_high;
                allow_front_sur = false;
            } else {
                text_utf16[end] = front_high;
                text_utf16[start] = end_high;
                front_high = front_low;
                allow_end_sur = false;
            }
        }

        start = start + 1;
        end = end - 1;
    }

    if len & 0x1 == 1 && !(allow_front_sur && allow_end_sur) {
        text_utf16[end] = if allow_front_sur { end_low } else { front_high };
    }

    String::from_utf16_lossy(&*text_utf16)
}
